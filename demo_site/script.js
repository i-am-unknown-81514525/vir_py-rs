import init, { MachineWrapper } from "./pkg/virtual_exec_js.js";

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

async function run() {
    clearOutput();
    set_running_state(true);
    const machine = new MachineWrapper(MEMORY_LIMIT, INSTRUCTION_LIMIT);
    try {
        machine.push_code(editor.getValue());
        while (is_running) {
            let r = machine.sync_run_for(2000n);
            if (!r.can_continue_executing) {
                set_running_state(false);
            }
        }
    } finally {
        machine.free();
    }
    set_running_state(false);
}

document.getElementById("run_button").addEventListener("click", run);
document.getElementById("stop_button").addEventListener("click", () => set_running_state(false));

