# Electron Rectangle 예제

데스크탑 화면에 투명·프레임리스·Always-on-top 윈도우를 띄우고 그 안에 네모 도형을 그리는 최소 예제입니다. Tauri 예제와 동일한 도메인 추상화를 쓰며, 셸 영역만 Electron 방식으로 다릅니다.

## 폴더 구조

```
electron-rectangle/
└── src/
    ├── main/                                # main process (Node 환경)
    │   ├── index.ts                          # 진입점. 라이프사이클 훅
    │   ├── app/
    │   │   ├── Application.ts                # 앱 컨테이너 (Factory + Router 보유)
    │   │   └── index.ts
    │   ├── window/                           # 윈도우 생성
    │   │   ├── WindowFactory.ts
    │   │   ├── TransparentWindow.ts          # BrowserWindow 래퍼
    │   │   └── index.ts
    │   └── ipc/                              # IPC 라우팅
    │       ├── IpcRouter.ts
    │       ├── handlers/
    │       │   └── WindowControlHandler.ts
    │       └── index.ts
    ├── preload/                             # 보안 경계 (renderer ↔ main)
    │   └── index.ts                          # contextBridge로 windowAPI 노출
    └── renderer/                            # renderer process (브라우저 환경)
        ├── index.html
        ├── styles.css                        # 투명 배경 + drag region
        ├── main.ts                           # 엔트리
        ├── shapes/                           # 도메인 추상화 (Tauri와 동일)
        │   ├── Shape.ts
        │   ├── Rectangle.ts
        │   └── index.ts
        ├── render/                          # 렌더러 추상화 (Tauri와 동일)
        │   ├── ShapeRenderer.ts
        │   ├── DomShapeRenderer.ts
        │   └── index.ts
        └── app/                             # 부트스트랩 (Tauri와 동일)
            ├── App.ts
            └── index.ts
```

## 분리 원칙

Electron의 process boundary가 폴더 구조와 일치합니다.

- `main/` ← Node.js 환경. OS 권한이 있고 BrowserWindow를 만들 수 있다.
- `preload/` ← main과 renderer 사이의 안전한 다리. contextBridge로 명시한 함수만 renderer에 노출.
- `renderer/` ← 브라우저 환경. DOM만 만질 수 있고 OS 직접 접근 불가.

`renderer/shapes/`, `renderer/render/`, `renderer/app/`는 Tauri 예제와 **파일 단위로 동일**합니다. 두 예제 간 비교가 쉽도록 의도한 설계입니다.

## 실행

```bash
# 의존성 설치
npm install

# 빌드 + 실행 (한 번에)
npm start

# 또는 단계별
npm run build
npx electron .

# 인스톨러 빌드
npm run package
```

요구 환경: Node 20+.

## 새 도형 추가 시 변경 범위

Tauri 예제와 동일하게 `renderer/shapes/Circle.ts` 하나 추가 + `index.ts` export 한 줄. main process 코드는 변경 0.

## main vs Tauri 셸의 코드량

Electron의 `main/`만 따져도 5개 파일에 걸쳐 IPC 라우팅·핸들러·preload·BrowserWindow 래퍼가 필요합니다 (대략 100~150줄). Tauri는 Rust 셸이 같은 일을 하지만 webview ↔ Python sidecar HTTP 호출로 우회할 수 있어, 더 적은 IPC command만으로 끝납니다. 이 차이가 TokiMon에서 Tauri를 선택한 근본 이유입니다.
