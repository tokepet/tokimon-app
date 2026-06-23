# 민트 드래곤 Identity Spec

## 기준 이미지

| 항목 | 값 |
| --- | --- |
| 기준 파일 | `idle.png` |
| 기준 프레임 | 1번째 프레임 |
| 용도 | 모든 상태 에셋의 얼굴, 비율, 색상, 실루엣 기준 |

## 고정 요소

| 항목 | 고정값 |
| --- | --- |
| 얼굴형 | 둥근 도마뱀형 머리와 짧은 주둥이 |
| 눈 | 큰 검은 눈, 흰 하이라이트, 주황색 홍채 포인트 |
| 머리/등 | 머리와 등 뒤쪽의 뾰족한 가시 실루엣 |
| 뿔 | 머리 위의 작은 주황색 뿔 |
| 몸 비율 | 큰 머리, 작은 몸, 짧은 팔과 다리 |
| 몸 색상 | 민트색과 청록색 계열 |
| 배 색상 | 밝은 크림색 배 |
| 포인트 색 | 주황색 뿔, 볼/몸 포인트, 꼬리 불꽃 |
| 꼬리 | 굵은 꼬리와 끝의 작은 불꽃 |
| 실루엣 | 뿔, 등가시, 꼬리 불꽃이 항상 유지되어야 함 |

## 금지 요소

- 민트/청록 계열 몸 색상을 다른 주색으로 바꾸지 않는다.
- 주황색 뿔과 꼬리 불꽃을 제거하지 않는다.
- 등가시 실루엣을 없애지 않는다.
- 얼굴을 길게 늘이거나 성체 드래곤 비율로 바꾸지 않는다.
- 눈 크기와 눈 위치를 크게 바꾸지 않는다.
- 꼬리를 짧게 만들거나 불꽃을 다른 장식으로 바꾸지 않는다.

## 허용 변화

- 상태에 맞는 표정 변화
- 꼬리, 팔, 다리, 몸 기울기 변화
- 불꽃의 작은 흔들림
- 잠, 배고픔, 슬픔 표현을 위한 자세 변화

## 제작 프롬프트 조각

```text
Keep the same small mint dragon identity.
Preserve the round lizard-like head, short snout, large black eyes with orange highlight, teal/mint body, cream belly, orange horns, orange accent marks, dorsal spikes, thick tail, and small flame at the tail tip.
Do not redesign the horns, dorsal spikes, tail flame, body proportions, color palette, or eye style.
Only change the pose and expression for the requested animation state.
```

## 검수 기준

- `idle.png` 첫 프레임과 같은 캐릭터로 보인다.
- 민트/청록 몸 색상과 밝은 배가 유지된다.
- 주황색 뿔, 등가시, 꼬리 불꽃이 유지된다.
- 상태가 바뀌어도 머리와 몸 비율이 달라지지 않는다.
- 모든 프레임에서 캐릭터 크기와 중심이 안정적이다.
