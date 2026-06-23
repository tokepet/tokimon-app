// 펫 성장 경계.
//
// 토큰 수집기(`collector` 모듈)는 usage_events 테이블에 사용량만 적재한다.
// 펫 성장 로직은 앱이 소유한다: 새로 들어온 usage_events를 읽어 펫의
// EXP/레벨/스탯으로 변환하고 pet_state에 저장한다. 수집기 DB와 같은 SQLite
// 파일을 공유하되, pet_state/growth_meta 테이블은 앱이 직접 만든다.

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

const CURRENT_PET_ID: &str = "current";
const LAST_EVENT_KEY: &str = "last_processed_event_id";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum StarterSpecies {
    DragonMint,
    RobotSpark,
    CoinMonkey,
}

impl StarterSpecies {
    fn as_str(&self) -> &'static str {
        match self {
            Self::DragonMint => "dragon-mint",
            Self::RobotSpark => "robot-spark",
            Self::CoinMonkey => "coin-monkey",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Self::DragonMint => "민트 드래곤",
            Self::RobotSpark => "스파크봇",
            Self::CoinMonkey => "코인몽",
        }
    }
}

impl TryFrom<&str> for StarterSpecies {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dragon-mint" => Ok(Self::DragonMint),
            "robot-spark" => Ok(Self::RobotSpark),
            "coin-monkey" => Ok(Self::CoinMonkey),
            _ => Err(format!("unknown starter species: {value}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PetState {
    pub species: StarterSpecies,
    pub name: String,
    pub level: i64,
    pub exp: i64,
    pub energy: i64,
    pub mood: i64,
    pub evolution_stage: i64,
    pub wisdom: i64,
    pub curiosity: i64,
    pub craft: i64,
    pub last_fed_at: Option<String>,
}

/// The token fields of a single usage_events row needed to feed the pet.
struct UsageRow {
    id: i64,
    input_tokens: i64,
    reasoning_tokens: i64,
    thoughts_tokens: i64,
    tool_tokens: i64,
    total_tokens: i64,
    timestamp: String,
}

/// Create the tables the app owns (pet_state + growth_meta). The collector owns
/// usage_events/collector_cursors and migrates those separately.
pub fn apply_migrations(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS pet_state (
            id TEXT PRIMARY KEY,
            species TEXT NOT NULL,
            name TEXT NOT NULL,
            level INTEGER NOT NULL,
            exp INTEGER NOT NULL,
            updated_at TEXT NOT NULL,
            state_json TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS growth_meta (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#,
    )
    .map_err(|error| error.to_string())?;
    let pet = load_or_create_pet(conn)?;
    save_pet(conn, &pet)
}

/// Apply every usage_events row newer than the last processed one to the pet.
/// Returns the number of events fed and the resulting pet state.
pub fn process_new_usage(conn: &Connection) -> Result<(i64, PetState), String> {
    let mut pet = load_or_create_pet(conn)?;
    let last_id = load_last_event_id(conn)?;

    let mut stmt = conn
        .prepare(
            "SELECT id, input_tokens, reasoning_tokens, thoughts_tokens, tool_tokens, total_tokens, timestamp
             FROM usage_events
             WHERE id > ?1
             ORDER BY id ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = stmt
        .query_map([last_id], |row| {
            Ok(UsageRow {
                id: row.get(0)?,
                input_tokens: row.get(1)?,
                reasoning_tokens: row.get(2)?,
                thoughts_tokens: row.get(3)?,
                tool_tokens: row.get(4)?,
                total_tokens: row.get(5)?,
                timestamp: row.get(6)?,
            })
        })
        .map_err(|error| error.to_string())?;

    let mut fed = 0;
    let mut max_id = last_id;
    for row in rows {
        let row = row.map_err(|error| error.to_string())?;
        pet = apply_usage_to_pet(&pet, &row);
        max_id = max_id.max(row.id);
        fed += 1;
    }

    if fed > 0 {
        save_pet(conn, &pet)?;
        save_last_event_id(conn, max_id)?;
    }
    Ok((fed, pet))
}

pub fn load_pet(conn: &Connection) -> Result<PetState, String> {
    load_or_create_pet(conn)
}

/// Switch the active starter, preserving each species' own progress in its
/// dedicated row so switching back restores prior growth.
pub fn select_starter(conn: &Connection, species: StarterSpecies) -> Result<PetState, String> {
    let existing = load_or_create_pet(conn)?;
    save_pet_for_species(conn, &existing)?;

    let mut pet = load_pet_for_species(conn, &species)?
        .unwrap_or_else(|| starter_pet(species.clone(), species.display_name()));
    if pet.name.is_empty() {
        pet.name = species.display_name().to_string();
    }
    save_pet(conn, &pet)?;
    Ok(pet)
}

// ---------------------------------------------------------------------------
// Growth math
// ---------------------------------------------------------------------------

fn starter_pet(species: StarterSpecies, name: &str) -> PetState {
    let (wisdom, curiosity, craft) = match species {
        StarterSpecies::DragonMint => (5, 2, 2),
        StarterSpecies::RobotSpark => (2, 5, 2),
        StarterSpecies::CoinMonkey => (2, 2, 5),
    };
    PetState {
        species,
        name: name.to_string(),
        level: 1,
        exp: 0,
        energy: 50,
        mood: 50,
        evolution_stage: 1,
        wisdom,
        curiosity,
        craft,
        last_fed_at: None,
    }
}

fn exp_from_usage(total_tokens: i64) -> i64 {
    (total_tokens.max(0) as f64).sqrt().floor() as i64
}

fn level_for_exp(exp: i64) -> i64 {
    let mut level = 1;
    let mut next_threshold = 100;
    while exp >= next_threshold {
        level += 1;
        next_threshold += level * 100;
    }
    level
}

fn clamp(value: i64, min: i64, max: i64) -> i64 {
    value.min(max).max(min)
}

fn apply_usage_to_pet(pet: &PetState, usage: &UsageRow) -> PetState {
    let exp = pet.exp + exp_from_usage(usage.total_tokens);
    let reasoning_boost = usage.reasoning_tokens / 500;
    let curiosity_boost = usage.thoughts_tokens / 500;
    let craft_boost = usage.tool_tokens / 500;
    PetState {
        exp,
        level: level_for_exp(exp),
        energy: clamp(pet.energy + usage.input_tokens / 1000, 0, 100),
        mood: clamp(
            pet.mood + reasoning_boost + curiosity_boost + craft_boost,
            0,
            100,
        ),
        wisdom: pet.wisdom + reasoning_boost,
        curiosity: pet.curiosity + curiosity_boost,
        craft: pet.craft + craft_boost,
        last_fed_at: Some(usage.timestamp.clone()),
        ..pet.clone()
    }
}

// ---------------------------------------------------------------------------
// Persistence
// ---------------------------------------------------------------------------

fn load_or_create_pet(conn: &Connection) -> Result<PetState, String> {
    load_pet_by_id(conn, CURRENT_PET_ID)
        .map(|pet| pet.unwrap_or_else(|| starter_pet(StarterSpecies::DragonMint, "민트 드래곤")))
}

fn load_pet_for_species(
    conn: &Connection,
    species: &StarterSpecies,
) -> Result<Option<PetState>, String> {
    load_pet_by_id(conn, species.as_str())
}

fn load_pet_by_id(conn: &Connection, id: &str) -> Result<Option<PetState>, String> {
    let state_json: Option<String> = conn
        .query_row(
            "SELECT state_json FROM pet_state WHERE id = ?1",
            [id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;

    state_json
        .map(|json| serde_json::from_str(&json).map_err(|error| error.to_string()))
        .transpose()
}

fn save_pet(conn: &Connection, pet: &PetState) -> Result<(), String> {
    save_pet_with_id(conn, CURRENT_PET_ID, pet)?;
    save_pet_for_species(conn, pet)
}

fn save_pet_for_species(conn: &Connection, pet: &PetState) -> Result<(), String> {
    save_pet_with_id(conn, pet.species.as_str(), pet)
}

fn save_pet_with_id(conn: &Connection, id: &str, pet: &PetState) -> Result<(), String> {
    let state_json = serde_json::to_string(pet).map_err(|error| error.to_string())?;
    conn.execute(
        "INSERT INTO pet_state (id, species, name, level, exp, updated_at, state_json)
         VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP, ?6)
         ON CONFLICT(id) DO UPDATE SET
            species = excluded.species,
            name = excluded.name,
            level = excluded.level,
            exp = excluded.exp,
            updated_at = excluded.updated_at,
            state_json = excluded.state_json",
        params![
            id,
            pet.species.as_str(),
            pet.name,
            pet.level,
            pet.exp,
            state_json
        ],
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

fn load_last_event_id(conn: &Connection) -> Result<i64, String> {
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM growth_meta WHERE key = ?1",
            [LAST_EVENT_KEY],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    Ok(value.and_then(|v| v.parse().ok()).unwrap_or(0))
}

fn save_last_event_id(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "INSERT INTO growth_meta (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![LAST_EVENT_KEY, id.to_string()],
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::Collector;

    fn open(db: &std::path::Path) -> Connection {
        Connection::open(db).unwrap()
    }

    #[test]
    fn new_usage_feeds_pet_and_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("tokimon.sqlite3");

        // Collector owns usage_events; seed one Gemini event via its public API.
        let telemetry = dir.path().join("telemetry.log");
        std::fs::write(
            &telemetry,
            "{\"timestamp\":\"2026-05-17T14:21:00.000Z\",\"name\":\"gemini_cli.api_response\",\"attributes\":{\"model\":\"gemini-2.5-pro\",\"prompt_id\":\"p1\",\"input_token_count\":2000,\"output_token_count\":50,\"thoughts_token_count\":1000,\"tool_token_count\":500,\"total_token_count\":10000}}\n",
        )
        .unwrap();
        let collector =
            Collector::with_sources(db.clone(), Some(telemetry), None, None).unwrap();
        collector.poll_gemini_once().unwrap();

        let conn = open(&db);
        apply_migrations(&conn).unwrap();

        let (fed, pet) = process_new_usage(&conn).unwrap();
        assert_eq!(fed, 1);
        assert!(pet.exp > 0);
        assert!(pet.last_fed_at.is_some());

        // Re-processing must not double-feed.
        let (fed_again, _) = process_new_usage(&conn).unwrap();
        assert_eq!(fed_again, 0);
    }

    #[test]
    fn switching_starters_preserves_per_species_progress() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("tokimon.sqlite3");
        let conn = open(&db);
        apply_migrations(&conn).unwrap();

        // Grow the default dragon a bit.
        let dragon = apply_usage_to_pet(
            &load_or_create_pet(&conn).unwrap(),
            &UsageRow {
                id: 1,
                input_tokens: 1000,
                reasoning_tokens: 0,
                thoughts_tokens: 0,
                tool_tokens: 0,
                total_tokens: 1350,
                timestamp: "2026-03-15T16:49:25.074Z".to_string(),
            },
        );
        save_pet(&conn, &dragon).unwrap();

        let robot = select_starter(&conn, StarterSpecies::RobotSpark).unwrap();
        let restored = select_starter(&conn, StarterSpecies::DragonMint).unwrap();

        assert_eq!(robot.exp, 0);
        assert_eq!(restored.exp, dragon.exp);
        assert_eq!(restored.level, dragon.level);
    }
}
