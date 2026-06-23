import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { StarterPicker } from "../components/StarterPicker";
import { Wanderer } from "../components/Wanderer";
import { loadActivePetLock, lockStarterPet } from "../domain/localProfile";
import { findPet, type PetSpriteDef } from "../domain/petCatalog";
import {
  TOKIMON_ATTACK_MS,
  TOKIMON_BIG_TOKEN_THRESHOLD,
  TOKIMON_FEEDING_MS,
  TOKIMON_STRUGGLE_MS,
  type TokimonPersistentState,
  type TokimonReaction,
} from "../domain/tokimonState";
import type { Point } from "../motion/randomTarget";

// macOS 메뉴바는 약 22pt에서 그려지지만 Retina를 고려해 2x인 64px로 보낸다.
// OS가 표시 크기로 자동 다운스케일한다.
const TRAY_PX = 64;

type AppState =
  | { phase: "selection" }
  | { phase: "wandering"; petId: string; startAt?: Point };

type TokenUsageSnapshot = {
  eventCount: number;
  inputTokens: number;
  reasoningTokens: number;
  thoughtsTokens: number;
  toolTokens: number;
  totalTokens: number;
  lastEventAt: string | null;
};

type ProviderStats = {
  eventsToday: number;
  tokensToday: number;
  lastEventAt: string | null;
};

type CollectorSnapshot = {
  todayTokens: number;
  status: string;
  activeSourceCount: number;
  claudeStats: ProviderStats;
  geminiStats: ProviderStats;
  codexStats: ProviderStats;
};

type DashboardSnapshot = {
  collector: CollectorSnapshot;
  pet: TokimonPersistentState;
};

type TokenCollectionPanelState = {
  usage: TokenUsageSnapshot;
  dashboard: DashboardSnapshot;
};

const TOKEN_FORMATTER = new Intl.NumberFormat("ko-KR");

export function App() {
  const [state, setState] = useState<AppState>(() => getInitialState());
  const [collectionState, setCollectionState] =
    useState<TokenCollectionPanelState | null>(null);
  const [reaction, setReaction] = useState<TokimonReaction | null>(null);
  const lastCollectionStateRef = useRef<TokenCollectionPanelState | null>(null);

  useEffect(() => {
    if (state.phase === "selection") {
      void enterSelectionMode();
      return;
    }

    void syncPetShell(state.petId);
  }, [state]);

  useEffect(() => {
    if (state.phase !== "wandering") {
      setCollectionState(null);
      setReaction(null);
      lastCollectionStateRef.current = null;
      return;
    }

    let disposed = false;
    const refresh = async (reason?: "fed") => {
      try {
        const [usage, dashboard] = await Promise.all([
          invoke<TokenUsageSnapshot>("token_usage"),
          invoke<DashboardSnapshot>("dashboard_snapshot"),
        ]);
        if (disposed) return;

        const nextState = { usage, dashboard };
        const nextReaction = resolveReaction(
          lastCollectionStateRef.current,
          nextState,
          reason,
        );
        lastCollectionStateRef.current = nextState;
        if (nextReaction) setReaction(nextReaction);
        setCollectionState(nextState);
      } catch (err) {
        console.error("토큰 수집 상태 조회 실패", err);
      }
    };

    void refresh();
    const timer = window.setInterval(refresh, 5000);
    const unlisten = listen("pet:fed", () => void refresh("fed"));

    return () => {
      disposed = true;
      window.clearInterval(timer);
      void unlisten.then((off) => off());
    };
  }, [state.phase]);

  useEffect(() => {
    if (!reaction) return;

    const timeout = window.setTimeout(
      () => setReaction((current) => (current === reaction ? null : current)),
      Math.max(0, reaction.until - Date.now()),
    );
    return () => window.clearTimeout(timeout);
  }, [reaction]);

  const handlePick = async (petId: string) => {
    const lockedPet = lockStarterPet(petId);
    if (!findPet(lockedPet.speciesId)) {
      console.error("저장된 스타터를 찾을 수 없습니다", lockedPet);
      return;
    }

    setState({ phase: "wandering", petId: lockedPet.speciesId });
  };

  const handlePetInteract = () => {
    setReaction({
      state: "struggling",
      until: Date.now() + TOKIMON_STRUGGLE_MS,
    });
  };

  return (
    <>
      {state.phase === "wandering" ? (
        <div
          data-tauri-drag-region
          style={{ position: "absolute", inset: 0, zIndex: 0 }}
        />
      ) : null}

      {state.phase === "wandering" ? (
        <TokenCollectionPanel state={collectionState} />
      ) : null}

      {state.phase === "selection" ? (
        <StarterPicker onPick={handlePick} />
      ) : (
        <Wanderer
          petId={state.petId}
          startAt={state.startAt}
          petState={collectionState?.dashboard.pet}
          reaction={reaction}
          onInteract={handlePetInteract}
        />
      )}
    </>
  );
}

function resolveReaction(
  previous: TokenCollectionPanelState | null,
  next: TokenCollectionPanelState,
  reason?: "fed",
): TokimonReaction | null {
  const now = Date.now();
  if (!previous) {
    return reason === "fed" ? { state: "feeding", until: now + TOKIMON_FEEDING_MS } : null;
  }

  const levelGained = next.dashboard.pet.level > previous.dashboard.pet.level;
  const tokenDelta = next.usage.totalTokens - previous.usage.totalTokens;
  const eventDelta = next.usage.eventCount - previous.usage.eventCount;

  if (levelGained || tokenDelta >= TOKIMON_BIG_TOKEN_THRESHOLD) {
    return { state: "attacking", until: now + TOKIMON_ATTACK_MS };
  }

  if (reason === "fed" || eventDelta > 0) {
    return { state: "feeding", until: now + TOKIMON_FEEDING_MS };
  }

  return null;
}

function TokenCollectionPanel({
  state,
}: {
  state: TokenCollectionPanelState | null;
}) {
  const usage = state?.usage;
  const collector = state?.dashboard.collector;
  const totalTokens = usage?.totalTokens ?? 0;
  const todayTokens = collector?.todayTokens ?? 0;
  const isCollecting = Boolean(
    collector &&
      collector.activeSourceCount > 0 &&
      !collector.status.toLowerCase().includes("not found") &&
      !collector.status.toLowerCase().includes("not configured"),
  );

  return (
    <div
      className="token-collection-panel"
      title={collectionTitle(state)}
    >
      <span
        className={
          isCollecting
            ? "token-collection-panel__dot is-active"
            : "token-collection-panel__dot"
        }
      />
      <span className="token-collection-panel__status">
        {isCollecting ? "수집중" : state ? "대기" : "확인중"}
      </span>
      <strong>{formatCompact(todayTokens)}</strong>
      <span className="token-collection-panel__meta">
        총 {formatCompact(totalTokens)}
      </span>
      <span className="token-collection-panel__meta">
        {TOKEN_FORMATTER.format(usage?.eventCount ?? 0)}건
      </span>
    </div>
  );
}

function collectionTitle(state: TokenCollectionPanelState | null) {
  if (!state) return "토큰 수집 상태를 불러오는 중";

  const { usage, dashboard } = state;
  const { collector } = dashboard;
  return [
    `상태: ${collector.status}`,
    `오늘: ${TOKEN_FORMATTER.format(collector.todayTokens)} 토큰`,
    `총합: ${TOKEN_FORMATTER.format(usage.totalTokens)} 토큰`,
    `이벤트: ${TOKEN_FORMATTER.format(usage.eventCount)}건`,
    `Claude 오늘: ${TOKEN_FORMATTER.format(collector.claudeStats.tokensToday)} 토큰`,
    `Codex 오늘: ${TOKEN_FORMATTER.format(collector.codexStats.tokensToday)} 토큰`,
    `Gemini 오늘: ${TOKEN_FORMATTER.format(collector.geminiStats.tokensToday)} 토큰`,
    usage.lastEventAt ? `마지막 수집: ${usage.lastEventAt}` : "마지막 수집: 없음",
  ].join("\n");
}

function formatCompact(value: number) {
  if (value >= 1_000_000_000) return `${(value / 1_000_000_000).toFixed(1)}B`;
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`;
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)}K`;
  return TOKEN_FORMATTER.format(value);
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
