// renderer에서 안전하게 호출할 수 있는 API만 contextBridge로 노출한다.
// nodeIntegration=false + contextIsolation=true 환경에서의 표준 패턴.

import { contextBridge, ipcRenderer } from "electron";

contextBridge.exposeInMainWorld("windowAPI", {
  close: () => ipcRenderer.invoke("window:close"),
  minimize: () => ipcRenderer.invoke("window:minimize"),
});
