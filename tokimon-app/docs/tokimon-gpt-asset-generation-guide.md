# 토키몬 GPT 에셋 생성 가이드

작성일: 2026-06-19

## 1. 목적

이 문서는 GPT 이미지 생성으로 토키몬 에셋을 만들 때 사용할 작업 지시서다.

`tokimon-asset-spec.md`는 최종 파일 규격과 검수 기준이다. GPT 프롬프트에는 이 스펙 전체를 그대로 넣지 않는다. GPT에는 필요한 시각 지시만 짧게 전달하고, 정확한 크기와 파일 규격은 후처리와 검수에서 맞춘다.

## 2. 기본 원칙

- 한 번에 모든 상태를 만들지 않는다.
- 한 번에 완성 sprite sheet를 기대하지 않는다.
- 한 상태씩 만든다.
- 상태별 에셋은 각기 따로 만든다.
- 한 이미지 또는 한 contact sheet에 여러 상태를 섞지 않는다.
- 먼저 캐릭터 기준 이미지를 확정한다.
- 그 다음 상태별 대표 포즈를 확정한다.
- 마지막에 해당 상태의 프레임 시트를 만든다.
- 프롬프트에는 이미지 모델이 그려야 할 내용만 넣는다.
- `manifest`, `fps`, `파일명`, `검증 스크립트` 같은 구현 정보는 이미지 프롬프트에 넣지 않는다.
- `512×512`, `4096×512` 같은 최종 픽셀 규격은 후처리 단계에서 맞춘다.
- GPT 생성 결과는 원본이 아니라 후처리 입력물로 취급한다.
- 앱에 넣는 최종 PNG는 모든 프레임을 왼쪽에서 오른쪽으로 가로 한 줄에 길게 이어 붙인 sprite sheet여야 한다.
- GPT에서 만든 contact sheet가 격자 형태라면, 후처리에서 반드시 가로 1줄 sprite sheet로 변환한다.

## 3. GPT가 자주 망가지는 원인

| 원인 | 결과 | 해결 |
| --- | --- | --- |
| 스펙 문서 전체를 프롬프트에 넣음 | 지시가 길어져 중요한 시각 조건을 놓침 | 생성용 프롬프트만 사용 |
| 모든 상태를 한 번에 요청 | 상태별 캐릭터가 달라짐 | 상태 하나씩 생성 |
| 정확한 sprite sheet 크기를 직접 요구 | 프레임 누락, 잘림, 비율 깨짐 | 1024×1024 contact sheet로 생성 후 후처리 |
| `같은 캐릭터`를 텍스트로만 설명 | 얼굴, 비율, 색이 drift | `identity-reference.png`를 매번 첨부 |
| 프레임별 지시가 너무 세밀함 | 부자연스러운 포즈, 복제 프레임 발생 | 큰 동작 흐름을 먼저 고정 |
| 상태 차이를 표정만으로 지시 | `sad`, `hungry`, `sleepy`가 비슷해짐 | 몸 포즈와 실루엣 차이를 강제 |
| 배경 효과를 많이 요청 | 80px에서 캐릭터가 묻힘 | 효과는 작게, 캐릭터 포즈 우선 |

## 4. 권장 작업 순서

### 4.1 신규 토키몬

1. 컨셉 자료를 준비한다.
2. 단일 캐릭터 기준 이미지를 만든다.
3. 기준 이미지를 보고 `identity.md`를 작성한다.
4. 기준 이미지를 `identity-reference.png`로 저장한다.
5. `idle` 대표 포즈를 만든다.
6. `idle` 프레임 시트를 만든다.
7. 나머지 상태를 한 상태씩 만든다.
8. 후처리로 최종 PNG 규격을 맞춘다.
9. 검수 기준으로 반려 여부를 판단한다.

신규 토키몬에는 기존 캐릭터 identity를 적용하지 않는다. 기획 설명, 레퍼런스 이미지, 색상 팔레트, 금지 요소를 사용해 새 identity를 먼저 확정한다.

### 4.2 기존 토키몬

1. `identity-reference.png`를 첨부한다.
2. `identity.md`에서 고정 요소, 금지 요소, 팔레트, 프롬프트 조각만 가져온다.
3. 만들 상태를 하나만 고른다.
4. 상태 대표 포즈를 먼저 만든다.
5. 대표 포즈가 통과하면 프레임 시트를 만든다.
6. 후처리로 최종 PNG 규격을 맞춘다.
7. 검수 기준으로 반려 여부를 판단한다.

## 5. 생성 단위

GPT에는 아래 3단계로 요청한다.

| 단계 | 결과물 | 목적 |
| --- | --- | --- |
| 1 | `identity-reference.png` | 캐릭터 외형 고정 |
| 2 | 상태 대표 포즈 1장 | 상태 차이 확인 |
| 3 | 상태별 contact sheet | 애니메이션 프레임 확보 |

대표 포즈가 실패하면 프레임 시트로 넘어가지 않는다.

상태별 contact sheet는 반드시 한 상태만 포함한다. 예를 들어 `attack`을 만들 때는 `attack` 프레임만 만들고, `idle`, `eat`, `walk` 프레임을 같은 이미지에 넣지 않는다.

## 6. GPT에 넣을 정보

필수 입력:

- `identity-reference.png`
- `identity.md`의 제작 프롬프트 조각
- 만들 상태 하나
- 상태별 핵심 동작
- 프레임 수와 contact sheet 배열
- 금지 요소

넣지 않는 정보:

- 앱 내부 코드 경로
- `manifest.json` 전체
- `fps`
- 최종 파일명
- 최종 PNG 전체 width
- 검증 스크립트 설명
- 긴 프레임별 기술 표 전체

## 7. 공통 스타일 지시

아래 문장은 모든 GPT 이미지 생성 프롬프트에 넣는다.

```text
Create a clean 2D pixel-art game character asset on a transparent background.
Use the Tokimon house style: cute chibi monster proportions, large head, small body, short limbs, rounded readable silhouette, thick dark pixel outline, clean pixel clusters, simple cel-shaded highlights and shadows, saturated limited palette, and one clear accent color.
Use the attached character reference as the strict identity reference.
Do not redesign the character.
Keep the same face, proportions, silhouette, color palette, markings, accessories, and fixed props.
The character must remain readable at small icon size.
Use a consistent 3/4 right-facing view unless the state explicitly requires lying down.
Keep the character centered in every frame.
Do not add text, labels, frame numbers, borders, grid lines, background scenery, floor, or extra characters.
Do not use photorealistic, 3D render, clay, watercolor, oil painting, vector illustration, or realistic humanoid proportions.
Only change the pose and expression needed for the requested state.
```

## 8. 신규 토키몬 기준 이미지 프롬프트

신규 토키몬을 처음 만들 때 사용한다.

```text
Create one original Tokimon character concept as a clean 2D pixel-art game character.
Use the Tokimon house style: cute chibi monster proportions, large head, small body, short limbs, rounded readable silhouette, thick dark pixel outline, clean pixel clusters, simple cel-shaded highlights and shadows, saturated limited palette, and one clear accent color.

Character concept:
{concept}

Required identity anchors:
- clear face shape
- clear body proportions
- 3 to 5 fixed colors
- 1 to 3 fixed silhouette features
- optional fixed prop or accessory
- readable at 80px

Create a single neutral idle pose.
Transparent background.
3/4 right-facing view.
Full body visible.
Centered in the canvas.
Readable at small icon size.

Do not create animation frames.
Do not create multiple character variants.
Do not add text, labels, background scenery, floor, or borders.
```

통과 기준:

- 80px 크기에서도 캐릭터의 고유 특징 3개가 보인다.
- 나중에 모든 상태에서 유지할 수 있는 단순한 실루엣이다.
- 고정 색상이 3-5개로 정리된다.
- 소품이 있다면 작아도 알아볼 수 있다.

## 9. 상태 대표 포즈 프롬프트

프레임 시트를 만들기 전에 상태 차이를 확인하기 위한 1장 프롬프트다.

```text
Use the attached identity-reference image as the strict character reference.
Create one key pose for the animation state: {state}.

Character identity:
{identityPromptFragment}

State goal:
{stateGoal}

The pose must be clearly different from idle.
The state must be readable from body pose first, facial expression second, and small effects last.
Keep the same character design, palette, proportions, and fixed props.
Transparent background.
3/4 right-facing view unless the state is sleep.
Full body visible and centered.

Do not create a sprite sheet yet.
Do not create multiple variants.
Do not add text, labels, background, floor, or frame borders.
```

대표 포즈가 통과해야 contact sheet를 만든다.

## 10. 상태별 contact sheet 프롬프트

상태 대표 포즈가 확정된 뒤 사용한다.

```text
Use the attached identity-reference image and approved key pose as references.
Create a pixel-art contact sheet for the animation state: {state}.

Character identity:
{identityPromptFragment}

Animation goal:
{stateGoal}

Create exactly {frames} animation frames arranged as a {rows} by {cols} contact sheet.
Read the frames from left to right, then top to bottom.
Each cell contains one full-body frame of the same character.
Keep the character centered and at the same scale in every cell.
Keep the feet or body baseline visually stable across frames.
The last frame must loop naturally back to the first frame.
This contact sheet is a temporary generation layout. The final app asset will be converted into one long horizontal sprite sheet with all frames placed left to right in a single row.

Motion plan:
{motionPlan}

Transparent background.
No grid lines.
No cell borders.
No text.
No frame numbers.
No background scenery.
No extra characters.
Do not crop the character.
Do not change the character identity.
```

## 11. 상태별 생성 카드

아래 카드에서 `{stateGoal}`과 `{motionPlan}`을 가져와 프롬프트에 넣는다.

### idle

| 항목 | 값 |
| --- | --- |
| frames | 4 |
| contact sheet | 2×2 |
| stateGoal | Calm neutral breathing idle. No strong emotion. |
| motionPlan | Frame 1 neutral stance. Frame 2 slight inhale and tiny body lift. Frame 3 short hold with optional blink. Frame 4 exhale and return to frame 1. |
| 금지 | 점프, 큰 팔동작, 과한 효과, 다른 상태처럼 보이는 소품 |

### eat

| 항목 | 값 |
| --- | --- |
| frames | 6 |
| contact sheet | 2×3 |
| stateGoal | Clearly eating or biting a small fixed food or prop. |
| motionPlan | Frame 1 holds food near mouth. Frame 2 lifts food closer. Frame 3 bite. Frame 4 chew. Frame 5 swallow or satisfied reaction. Frame 6 returns to eating-ready pose. |
| 금지 | 음식 없이 입만 움직임, `idle`과 같은 손 위치, 캐릭터보다 큰 소품 |

### attack

| 항목 | 값 |
| --- | --- |
| frames | 6 |
| contact sheet | 2×3 |
| stateGoal | Clear forward attack or hitting motion with a determined expression. |
| motionPlan | Frame 1 attack-ready stance. Frame 2 wind-up with the striking limb, tail, or fixed prop pulled back. Frame 3 forward strike begins. Frame 4 impact pose with a small hit effect. Frame 5 follow-through and recoil. Frame 6 returns to a tense ready pose. |
| 금지 | 기쁜 점프, 축하 포즈, 당황한 버둥거림, 다른 캐릭터나 맞는 대상 추가 |

### sad

| 항목 | 값 |
| --- | --- |
| frames | 4 |
| contact sheet | 2×2 |
| stateGoal | Sad posture with lowered head, drooped shoulders, and unhappy expression. |
| motionPlan | Frame 1 head begins to drop. Frame 2 shoulders slump. Frame 3 saddest pose with tear or tremble. Frame 4 slight recovery while still sad. |
| 금지 | 배를 잡는 포즈 중심, 졸린 표정, 누운 자세 |

### hungry

| 항목 | 값 |
| --- | --- |
| frames | 4 |
| contact sheet | 2×2 |
| stateGoal | Hungry weakness focused on the belly. |
| motionPlan | Frame 1 hands move toward belly. Frame 2 body hunches. Frame 3 hands hold belly with small tremble. Frame 4 slight sway while still hungry. |
| 금지 | 눈물 중심의 슬픔, 졸음 표시, 음식 먹는 동작 |

### sleepy

| 항목 | 값 |
| --- | --- |
| frames | 4 |
| contact sheet | 2×2 |
| stateGoal | Drowsy but still awake, standing or sitting. |
| motionPlan | Frame 1 half-closed eyes. Frame 2 head droops. Frame 3 almost asleep with small sleep mark. Frame 4 head lifts slightly but remains sleepy. |
| 금지 | 완전히 누움, 활짝 감은 수면 자세, `sleep`과 같은 자세 |

### sleep

| 항목 | 값 |
| --- | --- |
| frames | 4 |
| contact sheet | 2×2 |
| stateGoal | Fully asleep, lying down or curled up with closed eyes. |
| motionPlan | Frame 1 sleeping pose. Frame 2 slow inhale. Frame 3 slow exhale with tiny sleep mark movement. Frame 4 returns to sleeping pose. |
| 금지 | 서 있는 자세, 반쯤 뜬 눈, 졸린 상태처럼 보이는 자세 |

### walk

| 항목 | 값 |
| --- | --- |
| frames | 8 |
| contact sheet | 2×4 |
| stateGoal | Natural right-facing walk cycle with alternating legs and stable body rhythm. |
| motionPlan | Frame 1 near leg forward and far leg back. Frame 2 passing pose. Frame 3 far leg forward and near leg back. Frame 4 weight shift down. Frame 5 repeat contact with natural variation. Frame 6 passing pose. Frame 7 opposite contact. Frame 8 recovery into frame 1 without foot sliding. |
| 금지 | 모든 프레임에서 같은 다리 위치, 공중에 떠서 이동, 발 미끄러짐, 왼쪽 방향 별도 제작 |

### struggle

| 항목 | 값 |
| --- | --- |
| frames | 6 |
| contact sheet | 2×3 |
| stateGoal | Startled struggling reaction with lateral shake and flailing limbs. |
| motionPlan | Frame 1 startled recoil. Frame 2 body shakes to one side. Frame 3 body shakes to the other side. Frame 4 strongest flail with small shake marks. Frame 5 rebound. Frame 6 tense loop-back pose. |
| 금지 | 의도적인 타격 자세, 밝은 축하 효과, `attack`과 같은 전방 strike |

## 12. 상태 혼동 방지 문장

상태가 비슷하게 나올 때 아래 문장을 추가한다.

| 혼동 | 추가 문장 |
| --- | --- |
| `sad`가 `hungry`처럼 보임 | Sadness is shown through face, lowered head, and drooped shoulders. Do not make the belly the focus. |
| `hungry`가 `sad`처럼 보임 | Hunger is shown through hands touching the belly and weak posture. The belly must be the main visual cue. |
| `sleepy`가 `sleep`처럼 보임 | The character is drowsy but still upright or sitting. Do not make the character lie down. |
| `sleep`이 `sleepy`처럼 보임 | The character is fully asleep, lying down or curled up with closed eyes. Do not keep the character upright. |
| `attack`이 `struggle`처럼 보임 | Attack is an intentional forward strike with a determined expression. Do not show fear or random lateral shaking. |
| `struggle`이 `attack`처럼 보임 | Struggle motion shakes left and right with a startled expression. Do not show a controlled forward strike. |
| `walk`가 미끄러짐 | The feet must alternate contact with the ground. Do not slide the same pose across frames. |

## 13. 수정 요청 프롬프트

결과물이 아쉽다면 전체 프롬프트를 다시 쓰지 말고 아래 수정 요청을 짧게 붙인다.

### 캐릭터가 달라졌을 때

```text
Revise this image while preserving the exact character identity from the attached reference.
The face, proportions, colors, silhouette, and fixed props changed too much.
Keep the animation state, but make every frame look like the same original character.
```

### 상태 차이가 약할 때

```text
Revise this image so the state reads clearly from the body pose.
Do not rely only on facial expression or small effects.
Make the silhouette clearly different from idle.
```

### 프레임이 복제처럼 보일 때

```text
Revise the contact sheet so each frame has a visible motion change.
Do not duplicate adjacent frames.
Keep the loop smooth from the last frame back to the first.
```

### 캐릭터가 잘렸을 때

```text
Revise the contact sheet so the full character is visible in every cell.
Keep enough transparent padding around the character.
Do not crop hair, ears, tail, props, hands, or feet.
```

### walk가 부자연스러울 때

```text
Revise the walk cycle so the near leg and far leg alternate naturally.
Show clear contact, passing, opposite contact, and recovery poses.
The feet should not slide.
The body should have a small rhythmic up-and-down motion.
```

### 그림체가 다를 때

```text
Revise this image to match the Tokimon house style.
Use cute 2D pixel-art chibi monster proportions: large head, small body, short limbs, rounded readable silhouette, thick dark pixel outline, clean pixel clusters, simple cel-shaded highlights and shadows, saturated limited palette, and one clear accent color.
Do not use photorealistic, 3D render, clay, watercolor, oil painting, vector illustration, or realistic humanoid proportions.
```

## 14. 생성 결과 반려 기준

아래 중 하나라도 해당하면 다시 생성한다.

- 프레임마다 다른 캐릭터처럼 보인다.
- 토키몬 공통 그림체와 다르게 보인다.
- 큰 머리, 작은 몸, 짧은 팔다리의 chibi 비율이 아니다.
- 실사, 3D, 회화, 벡터 일러스트처럼 다른 매체 스타일로 보인다.
- 상태가 `idle`과 구분되지 않는다.
- `sad`, `hungry`, `sleepy`, `sleep`이 서로 비슷하다.
- `attack`과 `struggle`이 같은 행동처럼 보인다.
- `walk`에서 다리가 번갈아 움직이지 않는다.
- 프레임 안에서 캐릭터가 잘렸다.
- 셀마다 캐릭터 크기가 크게 다르다.
- 배경, 바닥, 텍스트, 번호, 테두리가 들어갔다.
- 상태 효과가 캐릭터보다 눈에 띈다.
- 80px 축소 시 캐릭터 고유 요소가 보이지 않는다.

## 15. 최종 파일 규격으로 변환

GPT 결과물을 그대로 앱에 넣지 않는다.

후처리에서 아래 작업을 수행한다.

1. contact sheet를 셀 단위로 자른다.
2. 각 프레임을 투명 512×512 캔버스에 정렬한다.
3. 캐릭터 중심과 바닥선을 맞춘다.
4. 모든 프레임을 왼쪽에서 오른쪽 순서로 가로 한 줄에 길게 이어 붙인다.
5. `tokimon-asset-spec.md`의 상태별 프레임 수와 크기를 검수한다.

최종 PNG는 반드시 `width = 512 × frames`, `height = 512`인 가로 1줄 sprite sheet다. 세로 배열, 격자 배열, 여러 줄 배열은 앱에 넣지 않는다.

최종 규격은 `docs/tokimon-asset-spec.md`를 따른다.
