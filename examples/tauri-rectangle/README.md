# Tauri Rectangle 예제

데스크탑 화면에 투명·프레임리스·Always-on-top 윈도우를 띄우고 그 안에 네모 도형을 그리는 최소 예제입니다.

## 폴더 구조

```
tauri-rectangle/
├── src-tauri/              # Rust 셸 (Tauri 영역, 회피 불가능한 Rust 코드)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── src/
│       ├── main.rs                       # 진입점. 트레이/setup 정의
│       ├── window/                       # 윈도우 생성 모듈
│       │   ├── mod.rs
│       │   └── transparent_window.rs     # Builder 패턴
│       └── commands/                     # IPC 커맨드
│           ├── mod.rs
│           └── window_control.rs         # close/minimize
└── src/                    # TypeScript UI (Tauri webview)
    ├── main.ts                            # 엔트리
    ├── styles.css                         # 투명 배경 CSS
    ├── shapes/                            # 도메인 추상화
    │   ├── Shape.ts                       # abstract class
    │   ├── Rectangle.ts                   # impl
    │   └── index.ts
    ├── render/                            # 렌더링 백엔드 추상화
    │   ├── ShapeRenderer.ts               # interface
    │   ├── DomShapeRenderer.ts            # impl (DOM)
    │   └── index.ts
    └── app/                               # 부트스트랩
        ├── App.ts                         # DI로 도형/렌더러 조립
        └── index.ts
```

## 분리 원칙

세 레이어가 한 방향으로만 의존합니다.

- `shapes/` ← OS와 프레임워크를 모르는 순수 도메인. 새 도형 추가는 여기서.
- `render/` ← `shapes/`만 의존. DOM 외 Canvas/SVG 렌더러 추가는 여기서.
- `app/` ← `shapes/`와 `render/`를 조립. 부트스트랩 전용.
- `src-tauri/` ← Tauri 셸. 위 어디도 의존하지 않음. 윈도우/IPC만 처리.

Rust 영역은 `window/`와 `commands/`로 명확히 분리되어, 새 윈도우 종류는 `window/`에, 새 IPC 명령은 `commands/`에만 추가하면 됩니다.

## 실행

```bash
# 의존성 설치
npm install

# 개발 모드 (Tauri 셸 + Vite dev 서버 동시)
npm run tauri:dev

# 프로덕션 빌드
npm run tauri:build
```

요구 환경: Node 20+, Rust 1.75+, 그리고 [Tauri 사전 요건](https://tauri.app/v1/guides/getting-started/prerequisites/) (macOS는 Xcode CLT, Windows는 MSVC).

## 새 도형 추가 시 변경 범위

`Circle`을 추가한다고 가정하면 아래 한 파일만 추가하고, `shapes/index.ts`에 export 한 줄을 더하면 됩니다. Rust 코드는 변경 0.

```ts
// src/shapes/Circle.ts
export class Circle extends Shape {
  toCSSStyle() {
    return {
      width: `${this.style.width}px`,
      height: `${this.style.width}px`,
      backgroundColor: this.style.color,
      borderRadius: "50%",
    };
  }
}
```
