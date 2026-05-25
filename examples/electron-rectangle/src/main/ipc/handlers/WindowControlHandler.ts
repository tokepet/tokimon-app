// 윈도우 제어 IPC 핸들러. 한 도메인의 핸들러를 한 클래스에 모은다.

import { BrowserWindow, IpcMainInvokeEvent } from "electron";

export class WindowControlHandler {
  close(event: IpcMainInvokeEvent): void {
    BrowserWindow.fromWebContents(event.sender)?.close();
  }

  minimize(event: IpcMainInvokeEvent): void {
    BrowserWindow.fromWebContents(event.sender)?.minimize();
  }
}
