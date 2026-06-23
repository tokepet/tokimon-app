# 토키몬 상태 정의

작성일: 2026-06-17

## 1. 상태 분류

토키몬 상태는 세 종류로 나눈다.

| 분류      | 목적                                  | 저장 위치                     |
| --------- | ------------------------------------- | ----------------------------- |
| 영속 상태 | 성장, 능력치, 마지막 먹이 시각 저장   | SQLite `pet_state.state_json` |
| 시각 상태 | 어떤 sprite를 보여줄지 결정           | 프론트엔드 `PetState`         |
| 행동 상태 | 영속 상태와 이벤트를 시각 상태로 변환 | `src/domain/tokimonState.ts`  |

## 2. 영속 상태

Rust 타입: `src-tauri/src/growth.rs`의 `PetState`

| 필드             | 타입             | 범위                                        | 의미                         |
| ---------------- | ---------------- | ------------------------------------------- | ---------------------------- |
| `species`        | `StarterSpecies` | `dragon-mint`, `robot-spark`, `coin-monkey` | 토키몬 종류                  |
| `name`           | `String`         | 제한 없음                                   | 표시 이름                    |
| `level`          | `i64`            | 1 이상                                      | 누적 경험치 기반 레벨        |
| `exp`            | `i64`            | 0 이상                                      | 누적 경험치                  |
| `energy`         | `i64`            | 0-100                                       | 활동 에너지                  |
| `mood`           | `i64`            | 0-100                                       | 기분                         |
| `evolutionStage` | `i64`            | 1 이상                                      | 진화 단계                    |
| `wisdom`         | `i64`            | 0 이상                                      | 추론 토큰 기반 능력치        |
| `curiosity`      | `i64`            | 0 이상                                      | 생각 토큰 기반 능력치        |
| `craft`          | `i64`            | 0 이상                                      | 도구 토큰 기반 능력치        |
| `lastFedAt`      | `String \| null` | ISO timestamp                               | 마지막 토큰 사용량 반영 시각 |

## 3. 스타터 초기값

공통 초기값:

| 필드             | 값     |
| ---------------- | ------ |
| `level`          | `1`    |
| `exp`            | `0`    |
| `energy`         | `50`   |
| `mood`           | `50`   |
| `evolutionStage` | `1`    |
| `lastFedAt`      | `null` |

종별 초기 능력치:

| species       | 표시 이름   | wisdom | curiosity | craft |
| ------------- | ----------- | -----: | --------: | ----: |
| `dragon-mint` | 민트 드래곤 |      5 |         2 |     2 |
| `robot-spark` | 스파크봇    |      2 |         5 |     2 |
| `coin-monkey` | 코인몽      |      2 |         2 |     5 |

## 4. 성장 계산

토큰 사용량 1건이 들어오면 다음 규칙을 적용한다.

| 항목           | 계산                                        |
| -------------- | ------------------------------------------- |
| 경험치 증가    | `floor(sqrt(totalTokens))`                  |
| energy 증가    | `inputTokens / 1000`                        |
| wisdom 증가    | `reasoningTokens / 500`                     |
| curiosity 증가 | `thoughtsTokens / 500`                      |
| craft 증가     | `toolTokens / 500`                          |
| mood 증가      | `wisdom 증가 + curiosity 증가 + craft 증가` |
| lastFedAt      | usage event의 `timestamp`                   |

제한:

- `energy`는 0-100 사이로 고정한다.
- `mood`는 0-100 사이로 고정한다.

레벨 계산:

- 기본 레벨은 1이다.
- 첫 레벨업 기준은 100 EXP다.
- 이후 기준치는 `level * 100`씩 증가한다.

예시:

| 누적 EXP | level |
| -------: | ----: |
|     0-99 |     1 |
|  100-299 |     2 |
|  300-599 |     3 |
|  600-999 |     4 |

## 5. 시각 상태

TypeScript 타입: `src/domain/petCatalog.ts`의 `PetState`

| 상태        | 의미               | 현재 사용 여부      |
| ----------- | ------------------ | ------------------- |
| `idle`      | 대기               | 사용 중             |
| `walk`      | 걷기               | 사용 중             |
| `sleep`     | 잠                 | 조건부 사용         |
| `sleepy`    | 졸림               | 조건부 사용         |
| `eat`       | 먹기               | 조건부 사용         |
| `attack`    | 공격/때리기        | 조건부 사용         |
| `sad`       | 슬픔               | 조건부 사용         |
| `hungry`    | 배고픔             | 조건부 사용         |
| `struggle`  | 버둥거림           | 코인몽 클릭 시 사용 |

현재 자동 표시 규칙:

| 조건             | 표시 상태   |
| ---------------- | ----------- |
| 이동 없음        | `idle`      |
| 왼쪽 방향 이동   | `walk` + 코드상 좌우 반전 |
| 오른쪽 방향 이동 | `walk` |
| 토키몬 클릭      | `struggle`  |

주의:

- 걷기 에셋은 오른쪽 방향 `walk.png` 하나만 사용한다.
- 왼쪽 이동은 `SpriteSheet.tsx`에서 `flipX`로 좌우 반전한다.
- 이동 transition은 3.5초다.
- 이동이 끝나면 `facing`을 `idle`로 되돌린다.
- 다시 이동하면 이동 상태를 다시 적용한다.

## 6. 에셋 문서

에셋 파일 규격, 상태별 프레임 수, fps, GPT 생성 가이드, 검수 기준은 별도 문서에서 관리한다.

- 에셋 규격 문서: `docs/tokimon-asset-spec.md`
- GPT 에셋 생성 가이드: `docs/tokimon-gpt-asset-generation-guide.md`
- 런타임 정의: `src/domain/petCatalog.ts`
- 에셋 위치: `public/pets/{species}`

## 7. 행동 상태 정의

행동 상태는 `src/domain/tokimonState.ts`에 정의한다.

권장 행동 상태:

| 행동 상태    | 표시 상태               | 진입 조건                       |
| ------------ | ----------------------- | ------------------------------- |
| `idle`       | `idle`                  | 기본 상태                       |
| `walking`    | `walk`                  | 위치 이동 중                    |
| `feeding`    | `eat`                   | 새 토큰 사용량 반영 직후        |
| `attacking`  | `attack`                | 큰 토큰 사용량 또는 레벨업 직후 |
| `hungry`     | `hungry`                | `energy <= 25`                  |
| `sad`        | `sad`                   | `mood <= 25`                    |
| `sleepy`     | `sleepy`                | 장시간 토큰 사용량 없음         |
| `sleeping`   | `sleep`                 | 매우 장시간 토큰 사용량 없음    |
| `struggling` | `struggle`              | 토키몬 클릭 직후                |

## 8. 행동 우선순위

여러 조건이 동시에 참이면 아래 순서로 하나만 선택한다.

| 우선순위 | 행동 상태    | 이유                                   |
| -------: | ------------ | -------------------------------------- |
|        1 | `attacking`  | 레벨업 또는 큰 이벤트는 공격 모션으로 즉시 보여준다. |
|        2 | `feeding`    | 새 토큰 사용량 반영을 보여준다.        |
|        3 | `struggling` | 사용자가 클릭한 즉시 반응을 보여준다.  |
|        4 | `walking`    | 움직일 때는 이동 에셋을 우선 보여준다. |
|        5 | `sad`        | 낮은 mood를 보여준다.                  |
|        6 | `hungry`     | 낮은 energy를 보여준다.                |
|        7 | `sleeping`   | 장시간 미사용 상태를 보여준다.         |
|        8 | `sleepy`     | 미사용 경고 상태를 보여준다.           |
|        9 | `idle`       | 기본 fallback이다.                     |

## 9. 권장 조건값

초기 구현 기준값:

| 조건                   | 값                     |
| ---------------------- | ---------------------- |
| `feeding` 유지 시간    | 2초                    |
| `attacking` 유지 시간  | 3초                    |
| `struggling` 유지 시간 | 1.5초                  |
| 큰 토큰 사용량         | `totalTokens >= 10000` |
| `hungry` 기준          | `energy <= 25`         |
| `sad` 기준             | `mood <= 25`           |
| `sleepy` 기준          | 마지막 먹이 후 30분    |
| `sleeping` 기준        | 마지막 먹이 후 2시간   |

## 10. 상태 전환 이벤트

| 이벤트         | 영향                          |
| -------------- | ----------------------------- |
| `pet:fed`      | `feeding` 또는 `attacking` 진입 |
| 레벨업         | `attacking` 진입              |
| 위치 이동 시작 | `walking` 진입                |
| 위치 이동 종료 | 다른 조건 없으면 `idle` 진입  |
| 토키몬 클릭    | `struggling` 진입             |
| 시간 경과      | `sleepy`, `sleeping` 판정     |
| 낮은 mood      | `sad` 판정                    |
| 낮은 energy    | `hungry` 판정                 |

## 11. 구현 상태

구현된 것:

- 행동 상태 타입
- 행동 상태 결정 함수
- `pet:fed` 이후 일시적 `eat` 표시
- 레벨업 또는 큰 토큰 증가 시 `attack` 표시
- 토키몬 클릭 시 `struggle` 표시
- 시간 경과 기반 `sleepy`, `sleep` 판정
- `energy`, `mood` 기반 `hungry`, `sad` 판정

추가된 파일:

```text
src/domain/tokimonState.ts
```

권장 타입:

```ts
export type TokimonBehaviorState =
  | "idle"
  | "walking"
  | "feeding"
  | "attacking"
  | "hungry"
  | "sad"
  | "sleepy"
  | "sleeping"
  | "struggling";
```

권장 함수:

```ts
export function resolveTokimonVisualState(input: {
  pet: PetDef;
  behavior: TokimonBehaviorState;
  facing: "idle" | "left" | "right";
}): PetState;
```

## 12. fallback 규칙

에셋이 없으면 다음 순서로 대체한다.

| 요청 상태   | 1차 fallback | 2차 fallback |
| ----------- | ------------ | ------------ |
| `attack`    | `struggle`   | `eat`        |
| `sleepy`    | `sleep`      | `idle`       |
| `hungry`    | `sad`        | `idle`       |
| `struggle`  | `sad`        | `idle`       |
| 기타        | `idle`       | 표시 안 함   |

## 13. 현재 결론

현재 토키몬은 성장 데이터와 이벤트를 기준으로 화면 행동 상태를 결정한다.

다음 구현 목표는 상태별 지속 시간과 조건값을 실제 사용감에 맞게 조정하는 것이다.
