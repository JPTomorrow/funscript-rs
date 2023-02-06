use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeError, Value};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FSPoint {
    pub pos: i32,
    pub at: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SimulatorPresets {
    pub name: String,
    pub full_range: bool,
    pub direction: i32,
    pub rotation: f32,
    pub length: f32,
    pub width: f32,
    pub offset: String,
    pub color: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FScript {
    pub version: String,
    pub inverted: bool,
    pub range: i32,
    pub bookmark: i32,
    pub last_position: i64,
    pub graph_duration: i32,
    pub speed_ratio: f32,
    pub injection_speed: i32,
    pub injection_bias: f32,
    pub scripting_mode: i32,
    pub simulator_presets: Vec<SimulatorPresets>,
    pub active_simulator: i32,
    pub reduction_tolerance: f32,
    pub reduction_stretch: f32,
    pub clips: Vec<Value>,
    pub actions: Vec<FSPoint>,
    pub raw_actions: Vec<FSPoint>,
}

#[derive(Error, Debug)]
pub enum FunscriptError {
    #[error("file read error {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("json error {0}")]
    JsonError(#[from] SerdeError),
}

pub fn load_funscript(path: &str) -> Result<FScript, FunscriptError> {
    let file = std::fs::read_to_string(path)?;
    let json = serde_json::from_str::<FScript>(&file)?;
    Ok(json)
}

pub fn save_funscript(path: &str, script: &FScript) -> Result<(), FunscriptError> {
    if !path.ends_with(".funscript") {
        return Err(FunscriptError::FileReadError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid file extension",
        )));
    }

    let json = serde_json::to_string_pretty(script)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn print_script(script: &FScript) {
    println!("{}", serde_json::to_string_pretty(script).unwrap());
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_load_funscript() {
        let path = "./test-scripts/joyfunscripter.funscript";
        let s = load_funscript(path);
        if s.is_err() {
            assert_matches!(s, Err(FunscriptError::FileReadError(_)));
        } else {
            assert_matches!(s, Ok(_));
        }
    }

    #[test]
    fn test_save_funscript() {
        let path = "./test-scripts/joyfunscripter.funscript";
        let save_path = "./test-scripts/out/joyfunscripter.funscript";

        let mut s = load_funscript(path).unwrap();
        s.bookmark = 100000;
        save_funscript(save_path, &s).unwrap();
        let check = load_funscript(save_path).unwrap();
        assert_eq!(check.bookmark, 100000);
    }
}
