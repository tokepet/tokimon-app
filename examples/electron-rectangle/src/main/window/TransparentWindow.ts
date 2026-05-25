// BrowserWindow를 한 번 감싸서 옵션 묶음을 일관되게 적용한다.
// 외부에서는 show/close/isDestroyed 같은 의미 있는 메서드만 노출.

import { BrowserWindow } from "electron";
import * as path from "node:path";

export interface TransparentWindowOptions {
  width: number;
  height: number;
  title: string;
}

export class TransparentWindow {
  private readonly window: BrowserWindow;

  constructor(options: TransparentWindowOptions) {
    this.window = new BrowserWindow({
      width: options.width,
      height: options.height,
      title: options.title,
      transparent: true,
      frame: false,
      alwaysOnTop: true,
      resizable: false,
      hasShadow: false,
      webPreferences: {
        preload: path.join(__dirname, "../../preload/index.js"),
        contextIsolation: true,
        nodeIntegration: false,
      },
    });

    this.window.loadFile(
      path.join(__dirname, "../../../src/renderer/index.html"),
    );
  }

  show(): void {
    this.window.show();
  }

  close(): void {
    this.window.close();
  }

  isDestroyed(): boolean {
    return this.window.isDestroyed();
  }
}
