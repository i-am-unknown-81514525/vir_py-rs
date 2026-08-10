import init, {Builtin, FnStreamOutput, MachineWrapper, StateEnum} from "./pkg/virtual_exec_js.js";

await init();

const editor = CodeMirror.fromTextArea(document.getElementById("editor"), {
    lineNumbers: true,
    mode: '',
    theme: 'material-darker',
});

const outputEl = document.getElementById("output");

function clearOutput() {
    outputEl.textContent = "";
}

function write(text) {
    outputEl.textContent += text;
}

const MEMORY_LIMIT = 1024 * 1024;
const INSTRUCTION_LIMIT = 200_000n;

let is_running = false;

const STDLIB = `stdout = get_output_stream();

fn print(v) {
    v = to_str(v);
    write_stream(std.stdout, v);
}

fn println(v) {
    v = to_str(v);
    v = concat(v, "\\n");
    std.print(v);
}`;

function set_running_state(state) {
    if (state) {
        document.getElementById("run_button").classList.add("hidden");
        document.getElementById("stop_button").classList.remove("hidden");
        is_running = true;
    } else {
        document.getElementById("run_button").classList.remove("hidden");
        document.getElementById("stop_button").classList.add("hidden");
        is_running = false;
    }
}

const streamDecoder = new TextDecoder("utf-8");
function vm_writer(arr) {
    write(streamDecoder.decode(arr))
}

const CHUNK = 20_000n;

const metric = {
    state: document.getElementById("metric_state"),
    note: document.getElementById("metric_note"),
    mem: document.getElementById("metric_mem"),
    mem_fill: document.getElementById("metric_mem_fill"),
    inst: document.getElementById("metric_inst"),
    inst_fill: document.getElementById("metric_inst_fill"),
    obj: document.getElementById("metric_obj"),
};

let mem_peak = 0;

function fmt_bytes(n) {
    if (n < 1024) return `${n} B`;
    if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KiB`;
    return `${(n / 1024 / 1024).toFixed(2)} MiB`;
}

function set_bar(el, ratio) {
    el.style.width = `${Math.min(100, ratio * 100)}%`;
    el.classList.toggle("warn", ratio >= 0.75 && ratio < 0.9);
    el.classList.toggle("danger", ratio >= 0.9);
}

function set_state(name, note) {
    metric.state.dataset.state = name;
    metric.state.textContent = name;
    metric.note.textContent = note ?? "";
}

function reset_metrics() {
    mem_peak = 0;
    set_state("running");
    set_bar(metric.mem_fill, 0);
    set_bar(metric.inst_fill, 0);
    metric.mem.textContent = "—";
    metric.inst.textContent = "—";
    metric.obj.textContent = "—";
}

function sample_metrics(alloc, requested) {
    const curr = alloc.curr();
    const max = alloc.max();
    mem_peak = Math.max(mem_peak, curr);

    set_bar(metric.mem_fill, max === 0 ? 0 : curr / max);
    metric.mem.textContent = mem_peak > curr
        ? `${fmt_bytes(curr)} / ${fmt_bytes(max)}  ↑${fmt_bytes(mem_peak)}`
        : `${fmt_bytes(curr)} / ${fmt_bytes(max)}`;

    const used = requested < INSTRUCTION_LIMIT ? requested : INSTRUCTION_LIMIT;
    set_bar(metric.inst_fill, Number(used) / Number(INSTRUCTION_LIMIT));
    const exact = used === INSTRUCTION_LIMIT;
    metric.inst.textContent = `${exact ? "" : "≤"}${Number(used).toLocaleString()} / ${Number(INSTRUCTION_LIMIT).toLocaleString()}`;

    metric.obj.textContent = String(alloc.obj_count());
}

function finish_metrics(state, thrown, state_error) {
    if (thrown) {
        set_state("error", String(thrown.message ?? thrown));
        return;
    }
    switch (state) {
        case StateEnum.TerminatedEOI:
        case StateEnum.TerminatedNotEOI:
            set_state("done");
            break;
        case StateEnum.Timeout:
            set_state("timeout", "instruction budget exhausted");
            break;
        case StateEnum.Error:
            set_state("error", String(state_error?.message ?? "execution error"));
            break;
        default:
            set_state("stopped");
    }
}

async function run() {
    clearOutput();
    reset_metrics();
    set_running_state(true);
    let pre = null, post = null, output = null, alloc = null;
    let requested = 0n, final_state = null, final_error = null, thrown = null;
    try {
        pre = new MachineWrapper(MEMORY_LIMIT, INSTRUCTION_LIMIT);
        output = new FnStreamOutput(vm_writer);
        pre.push_resolver(new Builtin().default())
        pre.push_resolver(output.to_resolver("get_output_stream"));
        post = pre.load_named_module_sync_all("std", STDLIB);
        pre.free();
        pre = null;
        alloc = post.get_alloc();
        post.push_code(editor.getValue());
        sample_metrics(alloc, requested);
        while (is_running) {
            let r = post.sync_run_for(CHUNK);
            requested += CHUNK;
            sample_metrics(alloc, requested);
            if (!r.can_continue_executing) {
                final_state = r.state_enum;
                final_error = r.get_error();
                set_running_state(false);
                continue;
            }
            await new Promise(resolve => setTimeout(resolve, 0));
        }
    } catch (e) {
        thrown = e;
        console.error(e);
    } finally {
        set_running_state(false);
        finish_metrics(final_state, thrown, final_error);
        alloc?.free();
        output?.free();
        post?.free();
        pre?.free();
    }
}

document.getElementById("run_button").addEventListener("click", run);
document.getElementById("stop_button").addEventListener("click", () => set_running_state(false));

