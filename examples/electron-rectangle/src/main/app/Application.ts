// 앱 라이프사이클을 책임지는 단일 객체.
// WindowFactory와 IpcRouter를 컴포지션으로 들고 있어, 진입점에서 직접 의존하지 않게 한다.

import { TransparentWindow, WindowFactory } from "../window";
import { IpcRouter } from "../ipc";

export class Application {
  private readonly windowFactory: WindowFactory;
  private readonly ipcRouter: IpcRouter;
  private mainWindow: TransparentWindow | null = null;

  constructor() {
    this.windowFactory = new WindowFactory();
    this.ipcRouter = new IpcRouter();
  }

  start(): void {
    this.ipcRouter.register();
    this.ensureWindow();
  }

  ensureWindow(): void {
    if (this.mainWindow && !this.mainWindow.isDestroyed()) return;

    this.mainWindow = this.windowFactory.createTransparent({
      width: 240,
      height: 240,
      title: "Rectangle",
    });
    this.mainWindow.show();
  }
}
