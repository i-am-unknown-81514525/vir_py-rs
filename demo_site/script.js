import init, {Builtin, FnStreamOutput, MachineWrapper} from "./pkg/virtual_exec_js.js";

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

async function run() {
    clearOutput();
    set_running_state(true);
    let pre = null, post = null, output = null;
    try {
        const pre = new MachineWrapper(MEMORY_LIMIT, INSTRUCTION_LIMIT);
        let output = new FnStreamOutput(vm_writer);
        pre.push_resolver(new Builtin().default())
        pre.push_resolver(output.to_resolver("get_output_stream"));
        const post = pre.load_named_module_sync_all("std", STDLIB);
        pre.free();
        post.push_code(editor.getValue());
        while (is_running) {
            let r = post.sync_run_for(20_000n);
            if (!r.can_continue_executing) {
                set_running_state(false);
                continue;
            }
            await new Promise(resolve => setTimeout(resolve, 0));
        }
    } finally {
        set_running_state(false);
        output?.free();
        post?.free();
        pre?.free();
    }
}

document.getElementById("run_button").addEventListener("click", run);
document.getElementById("stop_button").addEventListener("click", () => set_running_state(false));

