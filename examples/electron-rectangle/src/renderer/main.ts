// renderer 엔트리. main process와의 통신은 preload가 노출한 window.windowAPI를 통해서만.

import { App } from "./app";

const root = document.getElementById("root");
if (!root) throw new Error("#root element not found");

new App(root).start();
