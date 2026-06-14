import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { StarterPicker } from "../components/StarterPicker";
import { Wanderer } from "../components/Wanderer";
import { loadActivePetLock, lockStarterPet } from "../domain/localProfile";
import { findPet, type PetSpriteDef } from "../domain/petCatalog";
import type { Point } from "../motion/randomTarget";

// macOS 메뉴바는 약 22pt에서 그려지지만 Retina를 고려해 2x인 64px로 보낸다.
// OS가 표시 크기로 자동 다운스케일한다.
const TRAY_PX = 64;

type AppState =
  | { phase: "selection" }
  | { phase: "wandering"; petId: string; startAt?: Point };

export function App() {
  const [state, setState] = useState<AppState>(() => getInitialState());

  useEffect(() => {
    if (state.phase === "selection") {
      void enterSelectionMode();
      return;
    }

    void syncPetShell(state.petId);
  }, [state]);

  const handlePick = async (petId: string) => {
    const lockedPet = lockStarterPet(petId);
    if (!findPet(lockedPet.speciesId)) {
      console.error("저장된 스타터를 찾을 수 없습니다", lockedPet);
      return;
    }

    setState({ phase: "wandering", petId: lockedPet.speciesId });
  };

  return (
    <>
      {state.phase === "wandering" ? (
        <div
          data-tauri-drag-region
          style={{ position: "absolute", inset: 0, zIndex: 0 }}
        />
      ) : null}

      {state.phase === "selection" ? (
        <StarterPicker onPick={handlePick} />
      ) : (
        <Wanderer petId={state.petId} startAt={state.startAt} />
      )}
    </>
  );
}

function getInitialState(): AppState {
  const lockedPet = loadActivePetLock();
  if (lockedPet && findPet(lockedPet.speciesId)) {
    return { phase: "wandering", petId: lockedPet.speciesId };
  }

  return { phase: "selection" };
}

async function syncPetShell(petId: string) {
  await syncStarterWithBackend(petId);

  const pet = findPet(petId);
  const sprite = pet?.states.idle;

  if (sprite) {
    try {
      const rgba = await renderTrayRgba(sprite);
      await invoke("set_tray_icon", {
        rgba: Array.from(rgba),
        width: TRAY_PX,
        height: TRAY_PX,
      });
    } catch (err) {
      console.error("트레이 아이콘 설정 실패", err);
    }
  }

  await enterPetMode();
}

async function syncStarterWithBackend(petId: string) {
  // 선택한 종을 백엔드에 저장해 성장 로직이 같은 펫을 키우도록 한다.
  try {
    await invoke("select_starter", { species: petId });
  } catch (err) {
    console.error("스타터 저장 실패", err);
  }
}

async function enterPetMode() {
  try {
    await invoke("enter_pet_mode");
  } catch (err) {
    console.error("펫 창 전환 실패", err);
  }
}

async function enterSelectionMode() {
  try {
    await invoke("enter_selection_mode");
  } catch (err) {
    console.error("선택 창 전환 실패", err);
  }
}

async function renderTrayRgba(sprite: PetSpriteDef): Promise<Uint8ClampedArray> {
  const img = new Image();
  img.src = sprite.src;
  await new Promise<void>((resolve, reject) => {
    img.onload = () => resolve();
    img.onerror = () => reject(new Error(`스프라이트 로드 실패: ${sprite.src}`));
  });

  const canvas = document.createElement("canvas");
  canvas.width = TRAY_PX;
  canvas.height = TRAY_PX;
  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error("canvas 2D context를 만들 수 없습니다");

  // 첫 프레임(0, 0, frameWidth, frameHeight)만 잘라 트레이 사이즈로 그린다.
  ctx.drawImage(
    img,
    0,
    0,
    sprite.frameWidth,
    sprite.frameHeight,
    0,
    0,
    TRAY_PX,
    TRAY_PX,
  );

  return ctx.getImageData(0, 0, TRAY_PX, TRAY_PX).data;
}
