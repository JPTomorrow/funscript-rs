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
pub struct OFSMetadata {
    bookmarks: Vec<i32>,
    chapters: Vec<String>,
    creator: String,
    description: String,
    duration: i32,
    license: String,
    notes: String,
    performers: Vec<String>,
    #[serde(rename = "script_url")]
    script_url: String,
    tags: Vec<String>,
    title: String,
    #[serde(rename = "type")]
    ofs_type: String,
    #[serde(rename = "video_url")]
    video_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase", default)]
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
    pub metadata: OFSMetadata,
}

impl Default for FScript {
    fn default() -> Self {
        Self {
            version: "".to_string(),
            inverted: false,
            range: -1,
            bookmark: -1,
            last_position: -1,
            graph_duration: -1,
            speed_ratio: -1.0,
            injection_speed: -1,
            injection_bias: -1.0,
            scripting_mode: -1,
            simulator_presets: Vec::new(),
            active_simulator: -1,
            reduction_tolerance: -1.0,
            reduction_stretch: -1.0,
            clips: Vec::new(),
            actions: Vec::new(),
            raw_actions: Vec::new(),
            metadata: OFSMetadata {
                bookmarks: Vec::new(),
                chapters: Vec::new(),
                creator: "".to_string(),
                description: "".to_string(),
                duration: -1,
                license: "".to_string(),
                notes: "".to_string(),
                performers: Vec::new(),
                script_url: "".to_string(),
                tags: Vec::new(),
                title: "".to_string(),
                ofs_type: "".to_string(),
                video_url: "".to_string(),
            },
        }
    }
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
    use super::*;

    #[test]
    fn test_jfs_save_load_funscript() {
        let path = "./test-scripts/joyfunscripter.funscript";
        let save_path = "./test-scripts/out/joyfunscripter.funscript";

        let mut s = load_funscript(path).unwrap();
        assert!(s.last_position == 6388388382, "file has defaulted");
        s.bookmark = 100000;
        save_funscript(save_path, &s).unwrap();
        let check = load_funscript(save_path).unwrap();
        assert_eq!(check.bookmark, 100000);
    }

    #[test]
    fn test_ofs_save_load_funscript() {
        let path = "./test-scripts/openfunscripter.funscript";
        let save_path = "./test-scripts/out/openfunscripter.funscript";

        let mut s = load_funscript(path).unwrap();
        assert!(s.metadata.duration == 2610, "file has defaulted");
        s.bookmark = 100000;
        save_funscript(save_path, &s).unwrap();
        let check = load_funscript(save_path).unwrap();
        assert_eq!(check.bookmark, 100000);
    }
}
