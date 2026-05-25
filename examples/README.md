# 데스크탑 네모 예제 — Tauri vs Electron

같은 일(투명 윈도우 + 빨간 네모)을 두 프레임워크로 똑같이 구현한 비교용 미니 프로젝트 모음입니다. 두 예제는 **도메인 계층(`shapes/`, `render/`, `app/`)을 파일 단위로 동일하게 공유**하며, 차이는 오직 **셸 영역**에서만 드러납니다.

## 프로젝트

- [`tauri-rectangle/`](./tauri-rectangle/) — Tauri 2.x (Rust 셸 + TS UI)
- [`electron-rectangle/`](./electron-rectangle/) — Electron 31 (TS main + preload + renderer)

## 폴더 구조 한눈에 비교

```
tauri-rectangle/                          electron-rectangle/
├── src-tauri/      ◀── 셸 (Rust)         └── src/
│   └── src/                                  ├── main/      ◀── 셸 (TS, Node)
│       ├── window/                            │   ├── app/
│       └── commands/                          │   ├── window/
└── src/            ◀── UI (TS)               │   └── ipc/
    ├── shapes/      ─┐                       ├── preload/  ◀── 보안 경계
    ├── render/      ─┼─ 두 예제 공유          └── renderer/  ◀── UI (TS)
    └── app/         ─┘                           ├── shapes/    ─┐
                                                  ├── render/    ─┼─ 두 예제 공유
                                                  └── app/       ─┘
```

`shapes/Shape.ts`, `Rectangle.ts`, `render/DomShapeRenderer.ts`, `app/App.ts` 4개 파일은 **두 프로젝트가 동일 내용**입니다. 그래서 도형/렌더링 로직을 비교할 필요는 없고, 차이가 나는 셸 부분만 봐도 두 프레임워크의 철학이 드러납니다.

## OOP 설계 (양쪽 공통)

3개의 명확한 책임 경계를 둡니다.

| 레이어 | 책임 | 의존 방향 |
|---|---|---|
| `shapes/` | 도형이 무엇인지(width·height·color·CSS 변환). OS·프레임워크 모름. | 의존 없음 |
| `render/` | 도형을 어디에 어떻게 그릴지. 백엔드 추상화. | `shapes/` |
| `app/` | 도형과 렌더러를 조립해 시작 신호 송신. | `shapes/`, `render/` |

확장은 다음 두 방향으로만 일어납니다.

- **새 도형 추가** (예: Circle, Star) → `shapes/` 안에 클래스 1개 추가
- **새 렌더 백엔드** (예: Canvas, SVG, WebGL) → `render/` 안에 `ShapeRenderer` 구현 1개 추가

## 셸 영역 비교

| 항목 | Tauri | Electron |
|---|---|---|
| 셸 언어 | Rust | TypeScript (Node) |
| 진입점 | `src-tauri/src/main.rs` | `src/main/index.ts` |
| 윈도우 옵션 패턴 | Builder (`TransparentWindow::new().with_size(…)`) | Options 객체 (`new TransparentWindow({…})`) |
| IPC 정의 위치 | `#[tauri::command]` 함수 | `ipcMain.handle("…", handler)` |
| renderer 보안 | webview 기본 격리 | preload + `contextIsolation: true` 필수 |
| 윈도우 드래그 | `data-tauri-drag-region` HTML 속성 | `-webkit-app-region: drag` CSS |
| 셸 코드량 (이 예제) | ~80줄 (Rust) | ~120줄 (TS, main+preload) |

## 실행 방법 요약

```bash
# Tauri
cd tauri-rectangle
npm install
npm run tauri:dev

# Electron
cd electron-rectangle
npm install
npm start
```

자세한 빌드/실행 안내는 각 프로젝트의 README 참조.

## TokiMon 본 프로젝트에 주는 시사점

본 예제는 [TokiMon 기술명세](../기술명세.md)의 **"Rust 최소화" 정책**의 근거를 시각화하기 위해 만들었습니다.

- Tauri는 셸 영역이 Rust지만 **양이 적고 책임이 좁다** (윈도우 옵션 빌더 + 몇 개 command). 본 예제 기준 ~80줄.
- Electron은 셸이 TypeScript지만 **process boundary가 셋**이라(main · preload · renderer) 보일러플레이트가 많다. 본 예제 기준 ~120줄.

TokiMon에서 백엔드 로직(토큰 수집, 어댑터, SQLite)을 Python 사이드카로 빼면, Tauri 셸은 본 예제 정도의 분량(~150–270줄)에서 거의 늘지 않습니다. 그게 "Rust 최소화"가 실제로 어떻게 작동하는지의 모습입니다.
