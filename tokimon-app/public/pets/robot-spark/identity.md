# 스파크봇 Identity Spec

## 기준 이미지

| 항목 | 값 |
| --- | --- |
| 기준 파일 | `idle.png` |
| 기준 프레임 | 1번째 프레임 |
| 용도 | 모든 상태 에셋의 얼굴, 비율, 색상, 실루엣 기준 |

## 고정 요소

| 항목 | 고정값 |
| --- | --- |
| 얼굴 | 둥근 사각형 모니터 얼굴 |
| 화면 | 검은 화면, 주황색 빛 반사 또는 눈 형태 |
| 머리 | 둥근 금속 헬멧형 머리 |
| 안테나 | 머리 위의 두 개 원뿔형 안테나, 끝은 주황색 발광 |
| 귀/측면 장치 | 머리 양옆의 둥근 발광 부품 |
| 몸 비율 | 큰 머리, 작은 몸, 짧은 기계 팔과 다리 |
| 몸 색상 | 청록색 금속과 회색 금속 계열 |
| 포인트 색 | 주황색 발광 포인트 |
| 배/몸판 | 밝은 회색 전면 패널 |
| 꼬리 | 케이블형 꼬리와 끝의 작은 불꽃 또는 스파크 |
| 실루엣 | 모니터 얼굴, 두 안테나, 둥근 몸통, 케이블 꼬리가 항상 보여야 함 |

## 금지 요소

- 모니터 얼굴을 일반 생물 얼굴로 바꾸지 않는다.
- 두 안테나를 제거하거나 개수를 바꾸지 않는다.
- 주황색 발광 포인트를 다른 색으로 크게 바꾸지 않는다.
- 청록/회색 금속 팔레트를 다른 주색으로 바꾸지 않는다.
- 케이블 꼬리와 끝 스파크를 제거하지 않는다.
- 몸을 날씬하거나 인간형 비율로 바꾸지 않는다.
- 화면 눈의 위치와 형태를 크게 바꾸지 않는다.

## 허용 변화

- 화면 안 표정 변화
- 안테나와 꼬리의 작은 흔들림
- 팔, 다리, 몸 기울기 변화
- 상태 표현을 위한 작은 스파크 효과

## 제작 프롬프트 조각

```text
Keep the same small spark robot identity.
Preserve the rounded monitor face, black screen with orange glow, teal-gray metal body, cream front panel, two cone antennas with orange glowing tips, side glowing pods, short mechanical limbs, cable tail, and small spark at the tail tip.
Do not redesign the monitor face, antennas, glowing orange accents, robot proportions, metal palette, or cable-tail prop.
Only change the pose and expression for the requested animation state.
```

## 검수 기준

- `idle.png` 첫 프레임과 같은 캐릭터로 보인다.
- 모니터 얼굴, 두 안테나, 케이블 꼬리가 유지된다.
- 청록/회색 금속 팔레트와 주황색 발광 포인트가 유지된다.
- 상태가 바뀌어도 로봇의 머리와 몸 비율이 달라지지 않는다.
- 모든 프레임에서 캐릭터 크기와 중심이 안정적이다.
