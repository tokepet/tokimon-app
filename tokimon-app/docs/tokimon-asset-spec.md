# 토키몬 에셋 제작 규격

작성일: 2026-06-18
개정일: 2026-06-19

## 1. 목적

이 문서는 토키몬 에셋 제작 규격을 정의한다.

사용 대상:

- 신규 토키몬 에셋 제작
- 기존 토키몬 에셋 교체
- 생성된 sprite sheet 검수

GPT 이미지 생성 프롬프트는 이 문서를 그대로 사용하지 않는다. 생성 프롬프트와 작업 순서는 `docs/tokimon-gpt-asset-generation-guide.md`를 따른다.

## 2. 파일 구조

| 항목 | 값 |
| --- | --- |
| 에셋 위치 | `public/pets/{species}` |
| manifest 위치 | `public/pets/{species}/manifest.json` |
| identity spec 위치 | `public/pets/{species}/identity.md` |
| identity reference 이미지 | `public/pets/{species}/identity-reference.png` |
| 런타임 정의 | `src/domain/petCatalog.ts` |
| 렌더러 | `src/components/SpriteSheet.tsx` |
| 시트 합성 스크립트 | `scripts/assemble-sprite-sheet.*` |
| 시트 검증 스크립트 | `scripts/verify-pet-assets.*` |

필수 파일:

```text
public/pets/{species}/
  manifest.json
  identity.md
  identity-reference.png
  idle.png
  eat.png
  attack.png
  sad.png
  hungry.png
  sleepy.png
  sleep.png
  walk.png
  struggle.png
```

`identity-reference.png`는 §5 "캐릭터 연속성 규칙"에서 정의한다.

상태별 파일 분리 규칙:

- 각 시각 상태는 반드시 별도 PNG 파일로 제작한다.
- 한 PNG 파일에는 한 상태의 프레임만 포함한다.
- 여러 상태를 한 PNG 또는 한 contact sheet에 함께 넣지 않는다.
- 예: `idle.png`에는 `idle` 프레임만, `attack.png`에는 `attack` 프레임만 포함한다.

## 3. 고정 규격

신규 제작 또는 교체 제작하는 모든 토키몬 에셋은 아래 규격을 따른다. 기존 `256×256` 에셋과 `887×887` 에셋은 교체 전까지 legacy로 유지한다.

| 항목 | 고정값 |
| --- | --- |
| 표시 크기 | 80px |
| 프레임 크기 | 512×512px |
| sprite sheet 방향 | 가로 1줄 |
| 배경 | 완전 투명 |
| 프레임 간격 | 0px |
| 프레임 테두리 | 없음 |
| 파일 형식 | PNG RGBA |
| 정렬 기준 | 캐릭터 중심과 바닥선을 모든 프레임에서 유지 |
| base facing | 우향 3/4 (모든 상태 공통) |
| safe area | 캐릭터 높이는 프레임의 60–80%, 가로 중심 x ≈ 256, 발 바닥선 y ≈ 460 |
| 80px 가독성 | 80×80 미리보기에서 identity.md의 "식별 우선 요소 Top 3"가 모두 분별되어야 함 |
| 걷기 방향 | `walk.png` 하나만 제작 |
| `walk.png` 기본 방향 | 우향 3/4 |
| 왼쪽 이동 | 코드에서 수평 반전 |

PNG 전체 크기:

```text
width  = 512 * frames
height = 512
```

최종 제출 PNG는 모든 프레임을 왼쪽에서 오른쪽 순서로 가로 한 줄에 길게 이어 붙인 sprite sheet여야 한다. 세로 배열, 격자 배열, 여러 줄 배열은 최종 에셋으로 사용하지 않는다.

base facing 규칙은 `idle`을 포함한 모든 상태에 동일하게 적용한다. 캐릭터가 상태 사이에서 좌우로 "돌아보는" 부조화를 만들지 않기 위함이다.

## 4. 상태별 프레임 정의

| 시각 상태 | 파일 | frame | frames | PNG 전체 크기 | fps | loop |
| --- | --- | --- | ---: | --- | ---: | --- |
| `idle`     | `idle.png`     | 512×512 | 4 | 2048×512 | 2 | true |
| `eat`      | `eat.png`      | 512×512 | 6 | 3072×512 | 4 | true |
| `attack`   | `attack.png`   | 512×512 | 6 | 3072×512 | 8 | true |
| `sad`      | `sad.png`      | 512×512 | 4 | 2048×512 | 2 | true |
| `hungry`   | `hungry.png`   | 512×512 | 4 | 2048×512 | 2 | true |
| `sleepy`   | `sleepy.png`   | 512×512 | 4 | 2048×512 | 1 | true |
| `sleep`    | `sleep.png`    | 512×512 | 4 | 2048×512 | 1 | true |
| `walk`     | `walk.png`     | 512×512 | 8 | 4096×512 | 8 | true |
| `struggle` | `struggle.png` | 512×512 | 6 | 3072×512 | 8 | true |

이 표는 단일 진실의 출처(single source of truth)다. `manifest.json`, 후처리 스크립트, 검증 스크립트, 프롬프트 템플릿의 모든 수치는 이 표에서 파생되어야 한다.

## 5. 캐릭터 연속성 규칙

### 토키몬 공통 그림체

몽키, 드래곤, 로봇 에셋의 공통점을 기준으로 모든 신규 토키몬은 아래 그림체를 따른다. 이 규칙은 캐릭터 고유 identity보다 상위의 공통 스타일이다.

| 항목 | 공통 스타일 |
| --- | --- |
| 장르 | 귀여운 2D 픽셀 아트 게임 캐릭터 |
| 비율 | 큰 머리, 작은 몸, 짧은 팔다리의 chibi 비율 |
| 실루엣 | 둥글고 읽기 쉬운 전체 실루엣, 날카롭거나 사실적인 비율 금지 |
| 방향 | 기본은 우향 3/4 full-body view |
| 선 처리 | 진한 외곽선과 짧고 또렷한 내부 선 |
| 픽셀 처리 | 깨끗한 픽셀 클러스터, 과한 노이즈나 사진 질감 금지 |
| 색상 | 고채도 주색 1-2개, 밝은 보조색 1개, 눈에 띄는 포인트색 1개 |
| 명암 | 셀 셰이딩에 가까운 단순 명암, 강한 하이라이트와 어두운 그림자 구분 |
| 표정 | 큰 눈 또는 명확한 얼굴 신호로 감정이 바로 읽혀야 함 |
| 고정 특징 | 80px에서도 보이는 고유 실루엣 요소 또는 소품 1-3개 |
| 효과 | 상태 표현용 작은 픽셀 효과만 허용, 배경 효과에 의존하지 않음 |
| 배경 | 완전 투명, 바닥/풍경/소품 배경 없음 |

공통 스타일 금지:

- 실사, 3D 렌더, 점토, 수채화, 유화, 벡터 일러스트 스타일로 만들지 않는다.
- 사람형 7-8등신 비율로 만들지 않는다.
- 지나치게 복잡한 장식으로 80px에서 형태가 뭉개지게 만들지 않는다.
- 캐릭터마다 선 두께, 명암 방식, 픽셀 밀도, 채도 수준이 크게 달라지게 만들지 않는다.
- 배경, 바닥 그림자, 큰 이펙트로 캐릭터의 실루엣을 가리지 않는다.

기존 토키몬 에셋을 추가하거나 교체할 때는 반드시 `public/pets/{species}/identity.md`와 `public/pets/{species}/identity-reference.png`를 함께 참조한다. 공통 규격 문서에는 캐릭터 고유 외형을 적지 않는다.

새 토키몬을 처음 만들 때는 아직 확정된 `identity.md`/`identity-reference.png`가 없으므로 기존 identity를 적용하지 않는다. 대신 기획 설명, 레퍼런스 이미지, 색상 팔레트, 키워드, 금지 요소 같은 추가 자료를 사용해 먼저 identity를 확정한다. 첫 `idle.png`와 `identity.md`, `identity-reference.png`가 확정된 뒤 나머지 상태 에셋을 제작한다.

`identity-reference.png`:

- 512×512 RGBA, 투명 배경.
- 승인된 `idle.png`의 첫 프레임을 알파 채널 기준으로 잘라낸 뒤 §3 safe area 규격에 정렬해 저장한 단일 정사각 이미지.
- AI 호출 시 image edit/reference 입력으로 매번 첨부한다.
- 캐릭터의 절대 기준이다. 이 이미지를 갱신했다면 모든 상태 에셋을 재검수해야 한다.

`identity.md`는 아래 항목을 포함한다.

| 항목 | 목적 |
| --- | --- |
| 기준 이미지 | 어떤 파일과 프레임을 identity 기준으로 삼는지 지정 |
| 참조 자산 경로 | `identity-reference.png`의 상대 경로 명시 |
| 색상 팔레트 (hex) | 주색, 보조색, 포인트색, 라인색의 hex 값. 자연어 색상명만으로는 모델이 drift 한다. |
| 고정 요소 | 얼굴형, 눈, 비율, 실루엣, 색상, 장식, 소품 고정 |
| 금지 요소 | 바뀌면 안 되는 형태, 색상, 장식 변경 방지 |
| 허용 변화 | 상태 표현을 위해 바꿀 수 있는 포즈와 표정 범위 |
| 한 줄 캐릭터 묘사 | 프롬프트 본문에 그대로 붙일 1문장의 영문 정의 |
| 80px 식별 우선 요소 Top 3 | 80×80 축소 후에도 반드시 분별되어야 하는 요소 3개 |
| 제작 프롬프트 조각 | AI 생성 시 공통 프롬프트에 붙일 캐릭터 고유 문장 |
| 검수 기준 | 결과물이 같은 캐릭터로 보이는지 확인하는 항목 |

제작 원칙:

- 모든 상태는 같은 캐릭터의 다른 행동이어야 한다.
- 캐릭터의 얼굴, 눈, 비율, 실루엣, 팔레트, 장식, 소품은 유지한다.
- 상태별 변화는 포즈, 표정, 손발 위치, 몸 기울기, 상태 소품, 작은 효과선으로 표현한다.
- 각 상태는 라벨 없이도 구분되어야 한다.
- `idle` 포즈를 다른 상태에서 그대로 재사용하지 않는다.
- 캐릭터 identity는 유지하되 상태별 실루엣은 명확히 다르게 만든다.
- `idle.png` 첫 프레임을 기본 reference로 삼는다. AI 호출 시에는 `identity-reference.png`를 첨부한다.
- `eat`, `attack`, `sad`, `hungry`, `sleepy`, `sleep`, `walk`, `struggle`은 같은 identity를 유지해야 한다.

공통 identity 프롬프트:

```text
Use the attached identity-reference.png as the strict character reference. Do not redesign.
Use the provided character identity spec as the identity source.
Keep the same face, eyes, proportions, silhouette, color palette, markings, accessories, and fixed props.
Use exactly the hex palette listed in identity.md. Do not introduce other hues.
Only change the pose and expression required for the animation state.
Every frame must look like the exact same character.
```

상태 차별화 원칙:

| 상태 | 반드시 보여야 하는 차이 |
| --- | --- |
| `idle` | 중립 자세, 작은 호흡, 과한 감정 효과 없음 |
| `eat` | 입 또는 손이 먹는 대상과 상호작용, 먹는 행동이 명확해야 함 |
| `attack` | 팔, 다리, 꼬리, 고정 소품 중 하나로 전방을 때리는 동작, 준비 자세와 타격 순간이 명확해야 함 |
| `sad` | 처진 눈썹/눈, 낮은 머리, 움츠린 어깨 또는 눈물 |
| `hungry` | 배를 잡거나 힘없는 자세, 배고픔을 읽을 수 있는 떨림/표정 |
| `sleepy` | 졸린 눈, 고개 숙임, 느린 몸 기울기, 작은 졸림 표시 |
| `sleep` | 눕거나 웅크린 수면 자세, 눈 감김, 깨어 있는 자세와 구분 |
| `walk` | 좌우 다리가 번갈아 앞으로 나오는 자연스러운 보행, 다리 보폭과 몸 이동감, 우향 3/4 진행이 명확해야 함 |
| `struggle` | 당황/저항/버둥거림, 흔들림 효과, 팔과 다리의 빠른 반응 |

상태 차별화 금지:

- 모든 상태를 `idle`과 거의 같은 직립 자세로 만들지 않는다.
- 표정만 살짝 바꾸고 몸 포즈를 그대로 두지 않는다.
- 효과선만 추가하고 행동 포즈를 바꾸지 않는 방식으로 상태를 표현하지 않는다.
- `sad`, `hungry`, `sleepy`, `sleep`을 같은 축 처진 포즈로 반복하지 않는다.
- `attack`과 `struggle`을 같은 팔 휘두르기 포즈로 반복하지 않는다.

혼동 쌍 시각 분리 규칙 (DALL-E가 자주 헷갈리는 쌍을 명시한다):

| 혼동 쌍 | 분리 규칙 |
| --- | --- |
| `sleepy` vs `sleep` | sleepy = 서 있거나 앉아 있고 눈은 반쯤 감김. sleep = 완전히 누워있거나 웅크림, 눈 완전히 감김. 자세 차이가 1순위. |
| `sad` vs `hungry` | sad = 표정 중심 (눈물/눈썹/숙인 고개). hungry = 반드시 손이 배에 닿거나 배가 강조됨. |
| `attack` vs `struggle` | attack = 전방으로 의도적인 타격, 집중한 표정. struggle = X축 횡방향 흔들림, 당황 표정. 의도와 운동축이 다름. |

신규 토키몬 제작 순서:

1. 추가 자료를 수집한다.
2. 추가 자료를 바탕으로 캐릭터 identity 초안을 만든다.
3. `idle.png`를 먼저 제작한다.
4. `idle.png` 첫 프레임을 기준으로 `identity.md`와 `identity-reference.png`를 확정한다.
5. 확정된 `identity.md`/`identity-reference.png`를 기준으로 나머지 상태 에셋을 제작한다.

신규 토키몬에 사용할 수 있는 추가 자료:

| 자료 | 용도 |
| --- | --- |
| 캐릭터 컨셉 설명 | 종, 성격, 분위기 정의 |
| 레퍼런스 이미지 | 얼굴, 비율, 실루엣 방향 정의 |
| 색상 팔레트 (hex) | 주색, 보조색, 포인트색, 라인색 고정 |
| 고정 소품 설명 | 유지해야 할 장식이나 오브젝트 정의 |
| 금지 요소 | 바뀌면 안 되는 형태와 제외할 표현 정의 |
| 상태별 행동 설명 | 각 상태에서 허용되는 포즈 범위 정의 |

신규 토키몬 identity 생성 프롬프트:

```text
Create a new character identity from the provided concept materials.
Do not reuse an existing pet identity.
Define a consistent face, proportions, silhouette, color palette (with exact hex values), markings, accessories, and fixed props.
Generate the first idle sprite sheet from this new identity.
After the idle reference is approved, use it as the identity source for all other animation states.
```

## 6. AI 모델 제약과 권장 워크플로우

현재 워크플로우는 **GPT / DALL-E 계열을 사용해 한 번에 시트 단위로 생성**하는 것을 전제로 한다. 이 모델 계열의 다음 제약을 spec 차원에서 인정한다.

| 제약 | 내용 | 대응 |
| --- | --- | --- |
| 네이티브 비율 제한 | 1:1 (1024×1024), 16:9 (1792×1024), 9:16 (1024×1792)만 직접 출력 가능 | 가로 N프레임 1줄 시트는 한 번에 출력 불가. 그리드 출력 후 후처리 합성으로 우회 |
| 정확한 픽셀 크기 약함 | 512 같은 정수 캔버스를 그대로 받기 어려움 | 후처리 스크립트에서 알파 정렬 후 정확히 512×512로 정규화 |
| seed 미지원 | 동일 결과 재현 불가 | 동일 프롬프트로 후보 N장 생성 후 best 선택 |
| negative prompt 미지원 | 금지 요소가 종종 섞임 | 프롬프트 본문에 명시적 "Do not include …" 블록 포함 |
| 텍스트만으로 캐릭터 동일성 약함 | 호출마다 캐릭터가 미세하게 바뀜 | `identity-reference.png`를 매 호출 reference로 첨부 |

권장 1차 워크플로우 — **그리드 생성 후 후처리 합성**:

- DALL-E에 **1024×1024 정사각 캔버스**로 프레임을 그리드 배치해 달라고 요청한다.
- 그리드 규격:

  | state | frames | gen grid | gen size | post-process 목표 |
  | --- | ---: | --- | --- | --- |
  | `idle` / `sad` / `hungry` / `sleepy` / `sleep` | 4 | 2행 × 2열 (좌→우, 위→아래) | 1024×1024 | 2048×512 1줄 가로 시트 |
  | `eat` / `attack` / `struggle`                   | 6 | 2행 × 3열 (좌→우, 위→아래) | 1024×1024 | 3072×512 1줄 가로 시트 |
  | `walk`                                          | 8 | 2행 × 4열 (좌→우, 위→아래) | 1024×1024 | 4096×512 1줄 가로 시트 |

- 후처리 스크립트는 `scripts/assemble-sprite-sheet.*`(별도 작업)에서 담당한다. 처리 순서:
  1. 그리드 셀을 행/열 수만큼 정사각 슬라이스.
  2. 각 셀에서 알파 채널 기준 캐릭터 bbox 추출.
  3. 캐릭터 중심을 (x=256, y=320), 발 바닥선을 y≈460에 맞춰 정렬.
  4. 512×512 RGBA 캔버스로 정규화.
  5. `width = 512 × frames`인 가로 1줄 시트로 합성.
- 검증 스크립트는 `scripts/verify-pet-assets.*`(별도 작업)에서 §10 검수 기준을 자동화한다.

권장 2차 워크플로우 — **프레임별 개별 생성 후 합성** (점진 도입):

- 캐릭터 일관성이 그리드 방식으로 부족한 경우 사용.
- 1024×1024 정사각으로 1프레임씩 생성. 호출마다 `identity-reference.png` + 직전 프레임을 reference로 첨부.
- 후처리 스크립트는 위와 동일하지만 입력이 N장의 1024×1024이다.

운영 권장:

- 동일 프롬프트로 후보 **3~5장** 생성 후 best를 선택한다. seed가 없으므로 1회 생성으로 최종을 확정하지 않는다.
- best 선택 기준은 §5 검수 기준과 §10 검수 기준에 따른다.

## 7. AI 제작용 지시 기준

AI로 에셋을 만들 때 아래 조건을 그대로 전달한다. 캐릭터의 종, 색, 장식, 실루엣은 `identity.md`와 `identity-reference.png`에서 지정한다.

필수 조건:

- 투명 배경 PNG로 만든다.
- 출력 캔버스는 1024×1024 정사각이다. 셀 분할 후 512×512 프레임이 되도록 §6 그리드 규격을 지킨다.
- 모든 프레임은 §6 그리드 규격(2×2 / 2×3 / 2×4)에 맞춰 배치한다.
- 프레임 사이 간격, 구분선, 테두리, 텍스트, 번호를 넣지 않는다.
- 캐릭터의 크기, 위치, 중심축, 바닥선을 모든 프레임에서 유지한다.
- 프레임 전환 시 캐릭터가 좌우 또는 상하로 튀지 않게 한다.
- 그림자가 필요하면 캐릭터에 포함된 픽셀 그림자만 사용한다. 배경 그림자나 바닥면은 넣지 않는다.
- 같은 species의 모든 상태는 동일한 캐릭터 디자인과 동일한 hex 팔레트를 유지한다.
- 상태 표현은 포즈, 표정, 손발 위치, 몸 기울기, 상태 소품, 작은 효과선으로 구분한다. 배경 효과에 의존하지 않는다.
- 각 상태는 `idle`과 다른 실루엣을 가져야 한다.
- 라벨이나 파일명 없이 sprite만 보아도 상태가 읽혀야 한다.
- 80px로 축소했을 때 identity.md의 "식별 우선 요소 Top 3"가 모두 보여야 한다.
- 걷기는 우향 3/4 `walk.png` 하나만 만든다. 왼쪽 이동은 앱 코드에서 수평 반전한다.
- `walk`는 좌우 다리가 번갈아 앞으로 나오는 8프레임 보행 사이클이어야 한다.

제작 프롬프트 템플릿:

```text
Use the attached identity-reference.png as the strict character reference. Do not redesign the character.
For an existing pet, use the character identity spec from public/pets/{species}/identity.md.
For a new pet, use the provided concept materials to establish the identity before generating other states.

Create a transparent 1024x1024 PNG that arranges exactly {frames} frames of the same character in a {rows}-row × {cols}-column grid.
Frames are ordered left-to-right, then top-to-bottom.
Each cell is approximately {cellSize}px square and contains one frame of the animation.
Keep the character centered consistently in every cell, with the feet baseline aligned across all cells.
Keep the same character scale, baseline, silhouette, hex color palette, and design across all frames.

Use exactly the hex palette from identity.md: primary {primaryHex}, secondary {secondaryHex}, accent {accentHex}, line {lineHex}. Do not introduce other hues.

The animation state is: {state}.
{stateInstruction}

Make the animation state visually distinct from idle and from every other state.
Do not reuse the idle pose for this state.
The state must be readable from the character pose and facial expression without labels or text.

Do not include: text, labels, frame numbers, borders, dividers, background fills, ground shadows,
multiple character variants per frame, redesigned face, altered palette, or any element outside the character.
Use transparent background only.
```

상태별 템플릿 값:

| state | frames | rows × cols | cellSize | gen size | post totalWidth | fps |
| --- | ---: | --- | ---: | --- | ---: | ---: |
| `idle`     | 4 | 2×2 | 512 | 1024×1024 | 2048 | 2 |
| `eat`      | 6 | 2×3 | 512 | 1024×1024 | 3072 | 4 |
| `attack`   | 6 | 2×3 | 512 | 1024×1024 | 3072 | 8 |
| `sad`      | 4 | 2×2 | 512 | 1024×1024 | 2048 | 2 |
| `hungry`   | 4 | 2×2 | 512 | 1024×1024 | 2048 | 2 |
| `sleepy`   | 4 | 2×2 | 512 | 1024×1024 | 2048 | 1 |
| `sleep`    | 4 | 2×2 | 512 | 1024×1024 | 2048 | 1 |
| `walk`     | 8 | 2×4 | 512 | 1024×1024 | 4096 | 8 |
| `struggle` | 6 | 2×3 | 512 | 1024×1024 | 3072 | 8 |

상태별 프롬프트 보강 문장 (`{stateInstruction}`):

| state | 추가 지시 |
| --- | --- |
| `idle`     | Neutral relaxed standing pose with subtle breathing only. |
| `eat`      | Show a clear eating action with hands or mouth interacting with food or a fixed prop. |
| `attack`   | Show a clear forward attack motion: wind-up, strike, impact, follow-through, and recovery. Use a determined expression, not a joyful pose. |
| `sad`      | Show drooped eyes, lowered head, slumped shoulders, and a sad body posture. Expression-centered, not belly-centered. |
| `hungry`   | Show the character with one or both hands touching the belly and a weak hungry expression. Belly must be the visual focus. |
| `sleepy`   | Show half-closed eyes, lowered head, slow drooping posture, and small sleepiness marks. Character is still standing or sitting. |
| `sleep`    | Show a clearly sleeping pose, lying down or curled up with closed eyes. Character is no longer upright. |
| `walk`     | Show a clear right-facing 3/4 view 8-frame walk cycle where the left and right legs alternate naturally, with visible stride, weight shift, and body motion. |
| `struggle` | Show a clear startled struggling reaction with flailing limbs and small shake effects. X-axis lateral motion, not Y-axis upward motion. |

DALL-E 호출 체크리스트:

- [ ] `identity-reference.png`를 첨부했는가
- [ ] `identity.md`의 hex 팔레트 값을 프롬프트에 명시했는가
- [ ] `{rows}-row × {cols}-column grid` 지시를 정확한 frames 수에 맞게 넣었는가
- [ ] `Do not include …` 부정 명령 블록을 포함했는가
- [ ] `{stateInstruction}` 보강 문장을 넣었는가
- [ ] 후보를 3~5장 생성해 best를 선택했는가
- [ ] `scripts/assemble-sprite-sheet.*`로 후처리를 진행했는가

## 8. 상태별 프레임 동작 정의

공통 규칙:

- 1번 프레임은 해당 상태의 시작 자세다. 그리드 출력에서는 좌상단 셀이 1번이다.
- 마지막 프레임은 1번 프레임으로 자연스럽게 이어져야 한다.
- 명시된 정지 프레임이 아니라면 인접 프레임을 동일하게 복제하지 않는다.
- 캐릭터의 바닥 기준점은 프레임마다 크게 흔들리지 않아야 한다(허용 편차 ≤ 4px @ 512 기준).
- 점프, 반동, 흔들림이 있는 상태도 캐릭터가 셀 밖으로 나가면 안 된다.
- 80px 표시 크기에서도 상태 차이가 보일 만큼 몸 포즈가 달라야 한다.

### idle

| frame | 동작 |
| ---: | --- |
| 1 | 우향 3/4 기본 대기 자세. 눈은 열려 있고 몸은 기준 높이에 둔다. |
| 2 | 숨을 들이마시는 느낌으로 몸통과 머리가 1-2px 정도 위로 올라간다. 팔과 어깨는 힘을 뺀다. |
| 3 | 올라간 자세를 짧게 유지한다. 눈 깜빡임이나 작은 머리 기울임을 넣을 수 있다. |
| 4 | 숨을 내쉬며 1번 프레임의 높이와 자세로 돌아간다. |

### eat

| frame | 동작 |
| ---: | --- |
| 1 | 음식 또는 고정 소품을 손이나 입 근처에 준비한 자세. |
| 2 | 손이 음식 또는 소품을 입 쪽으로 올리고 머리가 살짝 앞으로 숙여진다. |
| 3 | 입이 음식에 닿거나 한입 베어 문다. 입 모양과 볼이 먹는 동작으로 바뀐다. |
| 4 | 씹는 자세. 손은 입 근처에 있고 볼, 입, 눈에서 먹는 느낌이 보여야 한다. |
| 5 | 삼키거나 만족하는 작은 반응. 음식 또는 손이 약간 내려간다. |
| 6 | 다시 먹을 준비 자세로 돌아가며 1번 프레임으로 자연스럽게 이어진다. |

### attack

| frame | 동작 |
| ---: | --- |
| 1 | 공격 준비 자세. 몸을 살짝 낮추고 팔, 다리, 꼬리, 고정 소품 중 타격에 사용할 부위를 뒤로 당긴다. |
| 2 | 힘을 모으는 자세. 몸통이 비틀리고 시선은 전방을 향한다. 표정은 집중하거나 결심한 느낌이어야 한다. |
| 3 | 전방 타격 시작. 타격 부위가 앞으로 빠르게 나가고 몸의 중심이 앞쪽으로 이동한다. |
| 4 | 타격 임팩트. 팔, 다리, 꼬리, 고정 소품 중 하나가 가장 앞으로 뻗고 작은 충격선이나 타격 효과가 보인다. |
| 5 | 후속 동작. 타격한 부위가 지나가며 몸과 장식에 반동이 생긴다. |
| 6 | 준비 자세로 회수한다. 완전한 `idle` 자세가 아니라 다음 공격으로 이어질 수 있는 긴장감을 유지한다. |

### sad

| frame | 동작 |
| ---: | --- |
| 1 | 머리와 어깨가 아래로 처지기 시작한다. |
| 2 | 눈썹, 눈, 입이 슬픈 표정이 되고 몸이 안쪽으로 웅크린다. |
| 3 | 가장 슬픈 자세. 눈물, 떨림, 고개 숙임 중 하나 이상이 명확해야 한다. |
| 4 | 아주 조금 회복하지만 여전히 처진 자세를 유지한다. 1번 프레임으로 자연스럽게 이어진다. |

### hungry

| frame | 동작 |
| ---: | --- |
| 1 | 손이 배 쪽으로 이동하거나 배를 의식하는 자세. 손과 배의 거리가 1순위 단서다. |
| 2 | 몸이 구부러지고 힘없는 표정이 나타난다. 손 하나 이상이 배에 닿는다. |
| 3 | 배를 감싸거나 떨림 효과가 보인다. 배고픔이 가장 강하게 읽혀야 한다. |
| 4 | 몸이 살짝 되돌아오지만 손과 표정은 배고픈 상태를 유지한다. |

### sleepy

| frame | 동작 |
| ---: | --- |
| 1 | 서 있거나 앉은 자세에서 눈이 반쯤 감기고 머리가 내려가기 시작한다. |
| 2 | 머리가 더 숙여지고 몸이 한쪽으로 느리게 기운다. 자세는 여전히 직립/좌식이다. |
| 3 | 눈이 거의 감긴다. 졸림 표시나 코/입의 졸린 반응을 넣을 수 있다. |
| 4 | 머리가 조금 들리지만 여전히 졸린 상태다. 1번 프레임으로 느리게 이어진다. |

### sleep

| frame | 동작 |
| ---: | --- |
| 1 | 완전히 누워 있거나 웅크린 수면 자세. 직립이 아니다. 눈은 감겨 있어야 한다. |
| 2 | 숨을 들이마시는 느낌으로 몸통이 약간 올라간다. 누운 자세는 유지한다. |
| 3 | 숨을 내쉬며 몸통이 내려간다. 수면 표시가 있다면 위치가 조금 이동한다. |
| 4 | 1번 프레임의 수면 자세로 돌아간다. 깨어 있는 표정이 나오면 안 된다. |

### walk

| frame | 동작 |
| ---: | --- |
| 1 | 우향 3/4 접지 자세. 왼쪽 다리는 앞으로, 오른쪽 다리는 뒤로 가며 한쪽 발이 바닥을 딛는다. |
| 2 | 통과 자세. 뒤쪽 다리가 앞으로 이동하고 몸이 살짝 올라간다. 팔, 꼬리, 장식은 다리와 반대로 흔들린다. |
| 3 | 반대 접지 자세. 오른쪽 다리가 앞으로, 왼쪽 다리가 뒤로 가며 무게 중심이 바뀐다. |
| 4 | 체중 이동 자세. 몸이 살짝 내려가고 팔, 꼬리, 장식이 반동을 가진다. |
| 5 | 1번과 같은 방향의 접지 자세로 돌아오되 완전 복제처럼 보이면 안 된다. 보폭과 몸 이동감이 유지되어야 한다. |
| 6 | 2번과 같은 통과 자세. 뒤쪽 다리가 다시 앞으로 넘어오고 몸이 올라간다. |
| 7 | 3번과 같은 반대 접지 자세. 좌우 다리의 위치가 다시 교차되어야 한다. |
| 8 | 1번 프레임으로 이어지는 회복 자세. 발이 미끄러지는 느낌 없이 루프가 연결되어야 한다. |

### struggle

| frame | 동작 |
| ---: | --- |
| 1 | 놀라거나 버둥거리기 시작하는 반응. 눈과 입이 긴장된 표정이다. |
| 2 | 몸이 X축 한쪽으로 밀리고 팔이나 다리가 크게 흔들린다. |
| 3 | 몸이 반대쪽으로 흔들리고 팔다리가 2번과 다른 방향으로 벌어진다. |
| 4 | 가장 강한 버둥거림. 흔들림 효과, 긴장선, 팔다리 움직임이 가장 크게 보인다. |
| 5 | 반동으로 몸이 되돌아오지만 표정은 계속 당황한 상태다. |
| 6 | 1번 프레임으로 이어지는 긴장 자세. `idle`처럼 안정된 자세가 되면 안 된다. |

## 9. manifest 형식

`manifest.json`은 에셋의 실제 파일과 렌더링 값을 기록한다. `frames`/`fps`/`file` 값은 §4 표에서 파생되어야 하며, §10 검증 절차로 교차 검증한다.

예시:

```json
{
  "id": "{species}",
  "name": "{displayName}",
  "description": "{description}",
  "displaySize": 80,
  "frameWidth": 512,
  "frameHeight": 512,
  "canvasSize": 512,
  "transparentBackground": true,
  "identitySpec": "identity.md",
  "identityReference": "identity-reference.png",
  "species": "{species}",
  "sprites": {
    "idle": {
      "file": "idle.png",
      "frames": 4,
      "fps": 2,
      "loop": true
    }
  }
}
```

## 10. 검수 기준

파일 검수:

- 모든 PNG 파일이 존재한다.
- 각 PNG의 width가 `512 × frames`와 일치한다.
- 각 PNG의 height가 `512`와 일치한다.
- `manifest.json`의 `frames`, `fps`, `file` 값이 §4 표 및 실제 PNG와 일치한다.
- `identity.md`와 `identity-reference.png`가 존재한다.
- 배경이 투명하다 (캔버스 네 모서리 픽셀의 alpha = 0).

그리드 출력 검수 (DALL-E 1024×1024 그리드 결과물에 대해):

- 셀 개수가 §6 그리드 규격(2×2 / 2×3 / 2×4)과 일치한다.
- 캐릭터가 누락된 셀이 없다.
- 셀 간 캐릭터 높이/폭 편차가 ±5% 이내다.
- 셀 간 캐릭터 색상이 visual diff로 일치한다(hex 팔레트 ±ΔE 시각 검수).
- 텍스트, 라벨, 프레임 번호, 셀 구분선이 없다.

후처리 1줄 시트 검수:

- 각 슬라이스 셀이 정확히 512×512이다.
- 각 슬라이스에 캐릭터가 존재한다 (alpha > 임계값인 픽셀 비율 검사).
- 모든 프레임에서 캐릭터 중심 x ≈ 256, 발 바닥선 y ≈ 460의 편차가 ≤ 4px이다.
- 인접 프레임 간 alpha 마스크 IoU가 너무 높지 않다(= 단순 복제 방지).

공통 그림체 검수:

- 큰 머리, 작은 몸, 짧은 팔다리의 chibi 비율을 유지한다.
- 둥글고 읽기 쉬운 실루엣을 가진다.
- 진한 외곽선, 단순 명암, 고채도 제한 팔레트가 유지된다.
- 80px 축소 시 얼굴 신호와 고유 실루엣 요소가 보인다.
- 실사, 3D, 회화, 벡터 일러스트처럼 다른 매체 스타일로 보이면 반려한다.
- 기존 몽키, 드래곤, 로봇과 선 두께, 픽셀 밀도, 명암 방식이 크게 다르면 반려한다.

애니메이션 검수:

- 상태 전환 중 캐릭터가 사라지지 않는다.
- 프레임 전환 중 캐릭터 중심이 튀지 않는다.
- 발 또는 하단 기준점이 프레임마다 흔들리지 않는다.
- 얼굴, 눈, 비율, 실루엣, 팔레트, 장식, 소품이 `identity.md` 및 `identity-reference.png`와 일치한다.
- 각 상태가 `idle`과 구분되는 포즈를 가진다.
- 각 상태가 다른 상태와 구분되는 실루엣을 가진다.
- 파일명이나 라벨 없이도 상태를 추측할 수 있다.
- 표정만 바뀌고 몸 포즈가 같은 상태는 반려한다.
- 효과선만 추가하고 행동 포즈가 같은 상태는 반려한다.
- §5 혼동 쌍 분리 규칙을 만족한다 (sleepy/sleep, sad/hungry, attack/struggle).
- `walk.png`는 우향 3/4 방향만 포함한다.
- `walk.png`는 좌우 다리가 번갈아 움직이는 자연스러운 보행 사이클이어야 한다.
- `walk.png`에서 다리가 같은 위치로 반복되거나 미끄러지는 느낌이면 반려한다.
- 왼쪽 이동은 앱의 `flipX` 처리로 확인한다.
- `struggle`은 클릭 반응으로 읽힌다.

80px 가독성 검수:

- 시트의 각 프레임을 80×80으로 다운샘플한 미리보기에서 `identity.md`의 "식별 우선 요소 Top 3"가 모두 보여야 한다.
- 미리보기에서 상태가 라벨 없이 식별 가능해야 한다.

자동 검증:

- 위 검수 중 정량적으로 표현 가능한 항목은 `scripts/verify-pet-assets.*`(별도 작업)이 수행한다.
- spec §4 표가 단일 진실의 출처다. PNG·manifest·스크립트 모두 이 표와의 일치 여부를 보고해야 한다.
