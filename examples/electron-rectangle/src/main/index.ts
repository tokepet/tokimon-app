// Electron main process 진입점. 라이프사이클 훅만 걸고 Application에 위임한다.

import { app } from "electron";
import { Application } from "./app";

const application = new Application();

app.whenReady().then(() => application.start());

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") app.quit();
});

app.on("activate", () => {
  // macOS: 독에서 다시 클릭했을 때 윈도우가 없으면 새로 띄운다.
  application.ensureWindow();
});
