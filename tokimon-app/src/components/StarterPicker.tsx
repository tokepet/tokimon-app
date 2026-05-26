import { useState } from "react";
import { PetView } from "./PetView";
import { PETS } from "../domain/petCatalog";

type Props = {
  onPick: (petId: string) => void;
};

export function StarterPicker({ onPick }: Props) {
  const [hovered, setHovered] = useState<string | null>(null);

  return (
    <div className="selection-root">
      <header className="selection-header">
        <h1>토키몬을 선택해주세요</h1>
        <p>앞으로 함께할 첫 번째 펫을 골라주세요</p>
      </header>

      <div className="pet-grid">
        {PETS.map((pet) => (
          <button
            key={pet.id}
            type="button"
            className={`pet-card ${hovered === pet.id ? "is-hovered" : ""}`}
            onMouseEnter={() => setHovered(pet.id)}
            onMouseLeave={() => setHovered((curr) => (curr === pet.id ? null : curr))}
            onClick={() => onPick(pet.id)}
          >
            <div className="pet-card__sprite">
              <PetView pet={pet} size={120} />
            </div>
            <div className="pet-card__name">{pet.name}</div>
            <div className="pet-card__desc">{pet.description}</div>
          </button>
        ))}
      </div>

      <footer className="selection-footer">
        선택한 펫은 메뉴바 아이콘에서 다시 변경할 수 있습니다
      </footer>
    </div>
  );
}
