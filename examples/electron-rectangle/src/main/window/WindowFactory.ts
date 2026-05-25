// 윈도우 생성 패턴을 한 곳에 모은다.
// 새 윈도우 종류(설정 창, 미니맵 등)가 추가될 때 Application 코드는 변경되지 않는다.

import { TransparentWindow, TransparentWindowOptions } from "./TransparentWindow";

export class WindowFactory {
  createTransparent(options: TransparentWindowOptions): TransparentWindow {
    return new TransparentWindow(options);
  }
}
