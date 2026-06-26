import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

type ProviderStats = {
  eventsToday: number;
  tokensToday: number;
  lastEventAt: string | null;
};

type CollectorSnapshot = {
  todayTokens: number;
  status: string;
  activeSourceCount: number;
  claudeEnabled: boolean;
  codexEnabled: boolean;
  claudeStats: ProviderStats;
  codexStats: ProviderStats;
};

type DashboardSnapshot = {
  collector: CollectorSnapshot;
};

type SourceKey = "claude" | "codex";

const NUMBER_FORMATTER = new Intl.NumberFormat("ko-KR");

const SOURCES: { key: SourceKey; label: string; note: string }[] = [
  { key: "claude", label: "Claude Code", note: "Anthropic · ~/.claude/projects" },
  { key: "codex", label: "Codex CLI", note: "OpenAI · ~/.codex/sessions" },
];

export function SettingsPanel() {
  const [snapshot, setSnapshot] = useState<CollectorSnapshot | null>(null);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  const refresh = async () => {
    try {
      const dashboard = await invoke<DashboardSnapshot>("dashboard_snapshot");
      setSnapshot(dashboard.collector);
    } catch (err) {
      console.error("설정 화면 상태 조회 실패", err);
      setMessage("상태를 불러오지 못했습니다");
    }
  };

  useEffect(() => {
    void refresh();
    const timer = window.setInterval(refresh, 5000);
    return () => window.clearInterval(timer);
  }, []);

  const enabledFor = (key: SourceKey): boolean => {
    if (!snapshot) return true;
    return key === "claude" ? snapshot.claudeEnabled : snapshot.codexEnabled;
  };

  const statsFor = (key: SourceKey): ProviderStats | null => {
    if (!snapshot) return null;
    return key === "claude" ? snapshot.claudeStats : snapshot.codexStats;
  };

  const toggleSource = async (key: SourceKey, next: boolean) => {
    setBusy(true);
    setMessage(null);
    try {
      await invoke("set_source_enabled", { source: key, enabled: next });
      await refresh();
    } catch (err) {
      console.error("수집 소스 토글 실패", err);
      setMessage("설정 변경에 실패했습니다");
    } finally {
      setBusy(false);
    }
  };

  const pollNow = async () => {
    setBusy(true);
    setMessage(null);
    try {
      await invoke("poll_now");
      await refresh();
      setMessage("수집을 실행했습니다");
    } catch (err) {
      console.error("수동 수집 실패", err);
      setMessage("수집 실행에 실패했습니다");
    } finally {
      setBusy(false);
    }
  };

  const resetData = async () => {
    if (!window.confirm("수집된 모든 토큰 데이터를 삭제할까요? 펫 성장 기록은 유지됩니다.")) {
      return;
    }
    setBusy(true);
    setMessage(null);
    try {
      await invoke("reset_collected_data");
      await refresh();
      setMessage("수집 데이터를 초기화했습니다");
    } catch (err) {
      console.error("데이터 초기화 실패", err);
      setMessage("초기화에 실패했습니다");
    } finally {
      setBusy(false);
    }
  };

  const reselectStarter = async () => {
    // The pet selection lock lives in the main window's localStorage; clearing
    // it and reloading the main window drops it back into selection mode.
    try {
      await invoke("reset_starter_selection");
      await getCurrentWindow().close();
    } catch (err) {
      console.error("스타터 재선택 실패", err);
      setMessage("스타터 재선택에 실패했습니다");
    }
  };

  return (
    <div className="settings-root">
      <header className="settings-header">
        <h1>설정</h1>
        <p>토키몬이 어떤 도구의 토큰을 먹을지 고르세요</p>
      </header>

      <section className="settings-section">
        <h2>수집 소스</h2>
        {SOURCES.map((source) => {
          const stats = statsFor(source.key);
          return (
            <label key={source.key} className="settings-row">
              <span className="settings-row__text">
                <span className="settings-row__label">{source.label}</span>
                <span className="settings-row__note">{source.note}</span>
                {stats ? (
                  <span className="settings-row__stat">
                    오늘 {NUMBER_FORMATTER.format(stats.tokensToday)} 토큰 ·{" "}
                    {NUMBER_FORMATTER.format(stats.eventsToday)}건
                  </span>
                ) : null}
              </span>
              <input
                type="checkbox"
                checked={enabledFor(source.key)}
                disabled={busy || !snapshot}
                onChange={(event) => void toggleSource(source.key, event.target.checked)}
              />
            </label>
          );
        })}
      </section>

      <section className="settings-section">
        <h2>펫</h2>
        <button type="button" className="settings-button" onClick={() => void reselectStarter()}>
          스타터 펫 다시 선택
        </button>
      </section>

      <section className="settings-section">
        <h2>데이터</h2>
        <div className="settings-button-row">
          <button
            type="button"
            className="settings-button"
            disabled={busy}
            onClick={() => void pollNow()}
          >
            지금 수집
          </button>
          <button
            type="button"
            className="settings-button is-danger"
            disabled={busy}
            onClick={() => void resetData()}
          >
            수집 데이터 초기화
          </button>
        </div>
      </section>

      <footer className="settings-footer">
        {message ?? (snapshot ? `상태: ${snapshot.status}` : "상태 확인 중…")}
      </footer>
    </div>
  );
}
