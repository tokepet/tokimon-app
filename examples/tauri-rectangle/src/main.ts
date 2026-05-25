// 엔트리. App을 부트스트랩하고 시작 신호만 보낸다.
// 새 도형/렌더러를 끼울 때도 이 파일은 거의 변경되지 않는다.

import { App } from "./app";

const root = document.getElementById("root");
if (!root) throw new Error("#root element not found");

new App(root).start();
