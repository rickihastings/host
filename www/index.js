import * as wasm from "host";

window.application = new wasm.Client();

window.update = () => {
	application.update();
}
