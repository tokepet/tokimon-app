export type PetState =
  | "idle"
  | "walk"
  | "sleep"
  | "sleepy"
  | "eat"
  | "attack"
  | "sad"
  | "hungry"
  | "struggle";

export type PetSpriteDef = {
  src: string;
  frameCount: number;
  frameWidth: number;
  frameHeight: number;
  fps: number;
};

export type PetDef = {
  id: string;
  name: string;
  description: string;
  displaySize: number;
  states: Partial<Record<PetState, PetSpriteDef>>;
};

const SPRITE_256 = { frameWidth: 256, frameHeight: 256, frameCount: 2 } as const;
const STANDARD_PET_FRAME = { frameWidth: 887, frameHeight: 887 } as const;
const ANIMATION_FPS = 1;

export const PETS: PetDef[] = [
  {
    id: "dragon-mint",
    name: "민트 드래곤",
    description: "잠 많은 작은 불도마뱀",
    displaySize: 80,
    states: {
      idle: { src: "/pets/dragon-mint/idle.png", ...SPRITE_256, fps: ANIMATION_FPS },
      eat: { src: "/pets/dragon-mint/eat.png", ...SPRITE_256, fps: ANIMATION_FPS },
      attack: { src: "/pets/dragon-mint/eat.png", ...SPRITE_256, fps: ANIMATION_FPS },
      sad: { src: "/pets/dragon-mint/sad.png", ...SPRITE_256, fps: ANIMATION_FPS },
      hungry: { src: "/pets/dragon-mint/hungry.png", ...SPRITE_256, fps: ANIMATION_FPS },
      sleep: { src: "/pets/dragon-mint/sleep.png", ...SPRITE_256, fps: ANIMATION_FPS },
      walk: { src: "/pets/dragon-mint/walk.png", ...SPRITE_256, fps: ANIMATION_FPS },
    },
  },
  {
    id: "robot-spark",
    name: "스파크봇",
    description: "토큰 에너지를 좋아하는 작은 로봇 몬스터",
    displaySize: 80,
    states: {
      idle: { src: "/pets/robot-spark/idle.png", ...SPRITE_256, fps: ANIMATION_FPS },
      eat: { src: "/pets/robot-spark/eat.png", ...SPRITE_256, fps: ANIMATION_FPS },
      attack: { src: "/pets/robot-spark/attack.png", ...SPRITE_256, fps: ANIMATION_FPS },
      sad: { src: "/pets/robot-spark/sad.png", ...SPRITE_256, fps: ANIMATION_FPS },
      hungry: { src: "/pets/robot-spark/hungry.png", ...SPRITE_256, fps: ANIMATION_FPS },
      sleepy: { src: "/pets/robot-spark/sleepy.png", ...SPRITE_256, fps: ANIMATION_FPS },
      sleep: { src: "/pets/robot-spark/sleep.png", ...SPRITE_256, fps: ANIMATION_FPS },
      walk: { src: "/pets/robot-spark/walk.png", ...SPRITE_256, fps: ANIMATION_FPS },
    },
  },
  {
    id: "coin-monkey",
    name: "코인몽",
    description: "꼬리 끝이 토큰 코인인 장난꾸러기 원숭이형 토키몬",
    displaySize: 80,
    states: {
      idle: { src: "/pets/coin-monkey/idle.png", ...STANDARD_PET_FRAME, frameCount: 4, fps: 2 },
      eat: { src: "/pets/coin-monkey/eat.png", ...STANDARD_PET_FRAME, frameCount: 6, fps: 4 },
      attack: { src: "/pets/coin-monkey/attack.png", ...STANDARD_PET_FRAME, frameCount: 6, fps: 8 },
      sad: { src: "/pets/coin-monkey/sad.png", ...STANDARD_PET_FRAME, frameCount: 4, fps: 2 },
      hungry: { src: "/pets/coin-monkey/hungry.png", ...STANDARD_PET_FRAME, frameCount: 4, fps: 2 },
      sleepy: { src: "/pets/coin-monkey/sleepy.png", ...STANDARD_PET_FRAME, frameCount: 4, fps: 1 },
      sleep: { src: "/pets/coin-monkey/sleep.png", ...STANDARD_PET_FRAME, frameCount: 4, fps: 1 },
      walk: { src: "/pets/coin-monkey/walk.png", ...STANDARD_PET_FRAME, frameCount: 8, fps: 8 },
      struggle: { src: "/pets/coin-monkey/struggle.png", ...STANDARD_PET_FRAME, frameCount: 6, fps: 8 },
    },
  },
];

export function findPet(id: string): PetDef | undefined {
  return PETS.find((p) => p.id === id);
}
