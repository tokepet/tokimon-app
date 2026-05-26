# TokiMon

AI 토큰 사용량에 반응해 성장하는 크로스플랫폼 데스크탑 펫.

Claude · OpenAI(그리고 OpenRouter · Together · Ollama 같은 호환 서비스)의 토큰 사용량을 한 곳에 모아, 그에 반응해 성장하고, 누적 토큰을 재화 삼아 새 펫을 뽑을 수 있는 데스크탑 펫입니다. 데스크탑 한쪽에 작은 펫이 떠 있고, 사용자가 AI를 쓸 때마다 펫이 자랍니다.

자세한 기획 · 기술 명세 · 일정은 아래 문서를 참고하세요.

- [기획.md](./기획.md) — 무엇을·왜·누구를 위해 만드는가
- [기술명세.md](./기술명세.md) — 기술 디테일
- [개발리스트.md](./개발리스트.md) — 일정과 작업 목록
- [목차.md](./목차.md)
- [펫_에셋_사양서.md](./펫_에셋_사양서.md)

---

## 기술 스택

- **Tauri 2** — 데스크탑 셸 (Rust)
- **React 18 + TypeScript** — UI
- **Vite 5** — 프론트엔드 번들러

앱 소스는 [`tokimon-app/`](./tokimon-app) 아래에 있습니다.

---

## 처음 시작하기

### 1. 사전 요구사항

| 도구 | 버전 | 비고 |
|---|---|---|
| Node.js | 18 이상 | 프론트엔드 빌드 |
| Rust (cargo) | 최신 stable | Tauri 빌드에 필수 |
| Xcode Command Line Tools | — | macOS에서 필요 |

#### macOS 기준 설치

```bash
# Xcode Command Line Tools
xcode-select --install

# Node.js (Homebrew 사용 시)
brew install node

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

설치 확인:

```bash
node --version
cargo --version
rustc --version
```

### 2. 의존성 설치

```bash
cd tokimon-app
npm install
```

`src-tauri`의 Rust 의존성은 처음 `tauri:dev` 또는 `tauri:build`를 실행할 때 자동으로 다운로드 · 컴파일됩니다.

### 3. 개발 모드 실행

```bash
npm run tauri:dev
```

첫 실행은 Rust 크레이트 컴파일 때문에 5~10분 정도 걸릴 수 있습니다. 이후로는 캐시되어 빠르게 뜹니다.

---

## 사용 가능한 스크립트

`tokimon-app/` 디렉터리에서 실행합니다.

| 명령어 | 설명 |
|---|---|
| `npm run dev` | 웹 브라우저용 Vite 개발 서버만 실행 (http://localhost:1420) |
| `npm run tauri:dev` | Tauri 데스크탑 앱 개발 모드 실행 |
| `npm run build` | 프론트엔드 타입 체크 + 프로덕션 빌드 |
| `npm run tauri:build` | 데스크탑 앱 배포 패키지 빌드 |

---

## 디렉터리 구조

```
tokepet-app/
├── tokimon-app/           # 앱 소스
│   ├── src/               # React + TypeScript 프론트엔드
│   ├── src-tauri/         # Tauri (Rust) 백엔드
│   ├── public/
│   ├── index.html
│   ├── package.json
│   └── vite.config.ts
├── 기획.md
├── 기술명세.md
├── 개발리스트.md
├── 목차.md
└── 펫_에셋_사양서.md
```

---

## 문제 해결

- **`cargo not found`** — Rust가 설치되지 않았거나 PATH가 적용되지 않은 상태입니다. `source "$HOME/.cargo/env"` 를 실행하거나 터미널을 새로 여세요.
- **`npm run tauri:dev` 첫 실행이 멈춘 것처럼 보임** — Rust 의존성 컴파일 중입니다. 정상이며, 인터넷 속도와 머신 성능에 따라 수 분 소요됩니다.
- **macOS에서 빌드 실패** — `xcode-select --install` 로 Command Line Tools를 설치했는지 확인하세요.
