# 코인몽 Identity Spec

## 기준 이미지

| 항목 | 값 |
| --- | --- |
| 기준 파일 | `idle.png` |
| 기준 프레임 | 1번째 프레임 |
| 용도 | 모든 상태 에셋의 얼굴, 비율, 색상, 실루엣 기준 |

## 고정 요소

| 항목 | 고정값 |
| --- | --- |
| 얼굴형 | 둥근 머리와 크고 부드러운 볼 |
| 눈 | 큰 갈색 눈, 밝은 하이라이트, 친근한 표정 |
| 머리 | 위로 솟은 진한 분홍색 머리털 실루엣 |
| 귀 | 머리 양옆의 큰 둥근 귀, 안쪽은 따뜻한 주황색 |
| 몸 비율 | 큰 머리, 작은 몸, 짧은 팔다리 |
| 몸 색상 | 진한 분홍색과 자주색 계열 |
| 배/얼굴 색상 | 밝은 크림색 계열 |
| 손발 | 밝은 크림색 손발, 작고 둥근 손가락과 발가락 |
| 고정 소품 | 꼬리 끝의 금색 코인 |
| 실루엣 | 머리털, 큰 귀, 긴 꼬리, 코인 끝장식이 항상 보여야 함 |

## 금지 요소

- 머리털 방향이나 개수를 크게 바꾸지 않는다.
- 귀 크기와 형태를 바꾸지 않는다.
- 꼬리 끝 코인을 제거하거나 다른 물체로 바꾸지 않는다.
- 몸 색상을 붉은 분홍/자주 계열 밖으로 바꾸지 않는다.
- 얼굴과 배의 밝은 크림색 영역을 없애지 않는다.
- 눈 크기, 눈 색, 하이라이트 형태를 크게 바꾸지 않는다.
- 체형을 길쭉하거나 성숙한 비율로 바꾸지 않는다.

## 허용 변화

- 상태에 맞는 표정 변화
- 손, 발, 꼬리, 몸 기울기 변화
- 먹기, 걷기, 잠, 클릭 반응에 필요한 일시적 포즈 변화
- 작은 감정 효과선 또는 상태 효과

## 제작 프롬프트 조각

```text
Keep the same cute coin-tailed character identity.
Preserve the round head, large round ears, pink-red hair tuft, cream face and belly, magenta body, large brown eyes, short limbs, long tail, and gold coin at the tail tip.
Do not redesign the face, ears, hair tuft, body proportions, color palette, or coin-tail prop.
Only change the pose and expression for the requested animation state.
```

## 검수 기준

- `idle.png` 첫 프레임과 같은 캐릭터로 보인다.
- 머리털, 큰 귀, 꼬리 끝 코인이 유지된다.
- 얼굴, 배, 몸의 색상 배치가 유지된다.
- 상태가 바뀌어도 눈과 얼굴 비율이 달라지지 않는다.
- 모든 프레임에서 캐릭터 크기와 중심이 안정적이다.
