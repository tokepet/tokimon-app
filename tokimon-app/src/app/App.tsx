import { invoke } from "@tauri-apps/api/core";
import { StarterPicker } from "../components/StarterPicker";
import { findPet, type PetSpriteDef } from "../domain/petCatalog";

// macOS 메뉴바는 약 22pt에서 그려지지만 Retina를 고려해 2x인 64px로 보낸다.
// OS가 표시 크기로 자동 다운스케일한다.
const TRAY_PX = 64;

export function App() {
  const handlePick = async (petId: string) => {
    const pet = findPet(petId);
    const sprite = pet?.states.idle;
    if (!sprite) {
      await invoke("hide_to_tray");
      return;
    }

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

    await invoke("hide_to_tray");
  };

  return <StarterPicker onPick={handlePick} />;
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
