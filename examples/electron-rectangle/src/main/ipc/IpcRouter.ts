// IPC 채널과 핸들러 객체를 매핑하는 라우터.
// 새 IPC 명령이 추가될 때 register() 안에 한 줄만 추가하면 된다.

import { ipcMain } from "electron";
import { WindowControlHandler } from "./handlers/WindowControlHandler";

export class IpcRouter {
  private readonly windowControl = new WindowControlHandler();

  register(): void {
    ipcMain.handle("window:close", (event) => this.windowControl.close(event));
    ipcMain.handle("window:minimize", (event) =>
      this.windowControl.minimize(event),
    );
  }
}
