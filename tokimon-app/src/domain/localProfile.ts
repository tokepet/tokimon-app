const STORAGE_KEY = "tokimon.localProfile.v1";
const SCHEMA_VERSION = 1;

export type LocalPetInstance = {
  id: string;
  speciesId: string;
  nickname: string | null;
  source: "starter";
  createdAt: string;
};

export type LocalProfile = {
  schemaVersion: typeof SCHEMA_VERSION;
  localProfileId: string;
  activePetInstanceId: string;
  pets: LocalPetInstance[];
  createdAt: string;
  updatedAt: string;
};

export type ActivePetLock = {
  localProfileId: string;
  petInstanceId: string;
  speciesId: string;
};

export function loadActivePetLock(): ActivePetLock | null {
  const profile = loadLocalProfile();
  if (!profile) return null;

  const activePet = profile.pets.find((pet) => pet.id === profile.activePetInstanceId);
  if (!activePet) return null;

  return {
    localProfileId: profile.localProfileId,
    petInstanceId: activePet.id,
    speciesId: activePet.speciesId,
  };
}

export function lockStarterPet(speciesId: string): ActivePetLock {
  const existingLock = loadActivePetLock();
  if (existingLock) return existingLock;

  const now = new Date().toISOString();
  const activePet: LocalPetInstance = {
    id: createId(),
    speciesId,
    nickname: null,
    source: "starter",
    createdAt: now,
  };

  const existingProfile = loadLocalProfile();
  const profile: LocalProfile = {
    schemaVersion: SCHEMA_VERSION,
    localProfileId: existingProfile?.localProfileId ?? createId(),
    activePetInstanceId: activePet.id,
    pets: [...(existingProfile?.pets ?? []), activePet],
    createdAt: existingProfile?.createdAt ?? now,
    updatedAt: now,
  };

  saveLocalProfile(profile);

  return {
    localProfileId: profile.localProfileId,
    petInstanceId: activePet.id,
    speciesId: activePet.speciesId,
  };
}

function loadLocalProfile(): LocalProfile | null {
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;

    const parsed: unknown = JSON.parse(raw);
    if (!isLocalProfile(parsed)) return null;

    return parsed;
  } catch (err) {
    console.error("로컬 프로필 로드 실패", err);
    return null;
  }
}

function saveLocalProfile(profile: LocalProfile) {
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(profile));
  } catch (err) {
    console.error("로컬 프로필 저장 실패", err);
  }
}

function isLocalProfile(value: unknown): value is LocalProfile {
  if (!value || typeof value !== "object") return false;
  const profile = value as Partial<LocalProfile>;

  return (
    profile.schemaVersion === SCHEMA_VERSION &&
    typeof profile.localProfileId === "string" &&
    typeof profile.activePetInstanceId === "string" &&
    typeof profile.createdAt === "string" &&
    typeof profile.updatedAt === "string" &&
    Array.isArray(profile.pets) &&
    profile.pets.every(isLocalPetInstance)
  );
}

function isLocalPetInstance(value: unknown): value is LocalPetInstance {
  if (!value || typeof value !== "object") return false;
  const pet = value as Partial<LocalPetInstance>;

  return (
    typeof pet.id === "string" &&
    typeof pet.speciesId === "string" &&
    (typeof pet.nickname === "string" || pet.nickname === null) &&
    pet.source === "starter" &&
    typeof pet.createdAt === "string"
  );
}

function createId(): string {
  if (window.crypto?.randomUUID) return window.crypto.randomUUID();
  return `tokimon_${Date.now().toString(36)}_${Math.random().toString(36).slice(2, 10)}`;
}
