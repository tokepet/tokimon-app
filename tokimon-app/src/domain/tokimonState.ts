import type { PetDef, PetState } from "./petCatalog";

export type TokimonFacing = "idle" | "left" | "right";

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

export type TokimonReactionState = "feeding" | "attacking" | "struggling";

export type TokimonReaction = {
  state: TokimonReactionState;
  until: number;
};

export type TokimonPersistentState = {
  species: string;
  name: string;
  level: number;
  exp: number;
  energy: number;
  mood: number;
  evolutionStage: number;
  wisdom: number;
  curiosity: number;
  craft: number;
  lastFedAt: string | null;
};

export const TOKIMON_FEEDING_MS = 2_000;
export const TOKIMON_ATTACK_MS = 3_000;
export const TOKIMON_STRUGGLE_MS = 1_500;
export const TOKIMON_BIG_TOKEN_THRESHOLD = 10_000;

const HUNGRY_ENERGY_THRESHOLD = 25;
const SAD_MOOD_THRESHOLD = 25;
const SLEEPY_AFTER_MS = 30 * 60 * 1_000;
const SLEEPING_AFTER_MS = 2 * 60 * 60 * 1_000;

const VISUAL_FALLBACKS: Record<PetState, PetState[]> = {
  idle: ["idle"],
  walk: ["walk", "idle"],
  sleep: ["sleep", "idle"],
  sleepy: ["sleepy", "sleep", "idle"],
  eat: ["eat", "idle"],
  attack: ["attack", "struggle", "eat", "idle"],
  sad: ["sad", "idle"],
  hungry: ["hungry", "sad", "idle"],
  struggle: ["struggle", "sad", "idle"],
};

export function resolveTokimonBehaviorState(input: {
  pet: TokimonPersistentState | null | undefined;
  facing: TokimonFacing;
  reaction: TokimonReaction | null;
  now?: number;
}): TokimonBehaviorState {
  const now = input.now ?? Date.now();

  if (input.reaction && input.reaction.until > now) {
    return input.reaction.state;
  }

  if (input.facing !== "idle") {
    return "walking";
  }

  const pet = input.pet;
  if (!pet) return "idle";

  if (pet.mood <= SAD_MOOD_THRESHOLD) {
    return "sad";
  }

  if (pet.energy <= HUNGRY_ENERGY_THRESHOLD) {
    return "hungry";
  }

  const lastFedAt = pet.lastFedAt ? Date.parse(pet.lastFedAt) : Number.NaN;
  if (!Number.isFinite(lastFedAt) || lastFedAt > now) {
    return "idle";
  }

  const inactiveFor = now - lastFedAt;
  if (inactiveFor >= SLEEPING_AFTER_MS) {
    return "sleeping";
  }
  if (inactiveFor >= SLEEPY_AFTER_MS) {
    return "sleepy";
  }

  return "idle";
}

export function resolveTokimonVisualState(input: {
  pet: PetDef;
  behavior: TokimonBehaviorState;
  facing: TokimonFacing;
}): PetState {
  const desired = desiredVisualState(input.behavior);
  return firstAvailableState(input.pet, desired);
}

function desiredVisualState(behavior: TokimonBehaviorState): PetState {
  switch (behavior) {
    case "walking":
      return "walk";
    case "feeding":
      return "eat";
    case "attacking":
      return "attack";
    case "hungry":
      return "hungry";
    case "sad":
      return "sad";
    case "sleepy":
      return "sleepy";
    case "sleeping":
      return "sleep";
    case "struggling":
      return "struggle";
    case "idle":
      return "idle";
  }
}

function firstAvailableState(pet: PetDef, desired: PetState): PetState {
  for (const state of VISUAL_FALLBACKS[desired]) {
    if (pet.states[state]) return state;
  }
  return "idle";
}
