use mint::Point2;
use ramer_douglas_peucker::rdp;
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeError, Value};
use thiserror::Error;

/// A .funscript action point
/// x = pos
/// y = at
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FSPoint {
    pub pos: i32,
    pub at: i32,
}

/// properties about a pressure simulator
/// that can be used to input points in a .funscript
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

/// extra metadata, specifically for OpenFunscripter (OFS)
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

/// a serializable and deserializable .funscript file
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

/// Error types for .funscript file operations
#[derive(Error, Debug)]
pub enum FunscriptError {
    #[error("file read error {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("json error {0}")]
    JsonError(#[from] SerdeError),
    #[error("failed to {0} point at index {1}")]
    PointError(String, usize),
}

/// loads a .funscript file using the provided path
pub fn load_funscript(path: &str) -> Result<FScript, FunscriptError> {
    let file = std::fs::read_to_string(path)?;
    let json = serde_json::from_str::<FScript>(&file)?;
    Ok(json)
}

/// saves a .funscript file using the provided path
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

/// adds an action point
pub fn get_pt(script: &mut FScript, idx: usize) -> Result<&mut FSPoint, FunscriptError> {
    if idx >= script.actions.len() {
        return Err(FunscriptError::PointError("get".to_string(), idx));
    }
    Ok(&mut script.actions[idx])
}

// runs the ramer-douglas-peucker algorithm on the script
pub fn apply_rdp(script: &mut FScript, epsilon: f64) {
    let mut points: Vec<Point2<i32>> = Vec::new();
    for pt in &script.actions {
        points.push(Point2 {
            x: pt.at,
            y: pt.pos,
        });
    }

    // keep points that are in idxs
    let idxs = rdp(points.as_slice(), epsilon);
    let mut reduced: Vec<Point2<i32>> = Vec::new();
    for idx in idxs {
        reduced.push(points[idx]);
    }

    script.actions.clear();
    for pt in reduced {
        script.actions.push(FSPoint {
            at: pt.x,
            pos: pt.y,
        });
    }
}

/// print the .funscript structure
pub fn print_script(script: &FScript) {
    println!("{}", serde_json::to_string_pretty(script).unwrap());
}

// print the .funscript structure
// fn print_script_diagnostics(s: &FScript) {
//     println!("# of points: {}", s.actions.len());
// }

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

    #[test]
    fn test_get_set_pt() {
        let path = "./test-scripts/openfunscripter.funscript";
        let mut s = load_funscript(path).unwrap();
        let pt = get_pt(&mut s, 0).unwrap();
        assert_eq!(pt.at, 218703);
        pt.at = 12345678;
        assert_eq!(pt.at, 12345678);
    }
}
