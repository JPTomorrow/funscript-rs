use serde_json::{Error as SerdeError, Value};
use thiserror::Error;

pub type FScript = Value; // empty named tuple for json

#[derive(Error, Debug)]
pub enum FunscriptError {
    #[error("file read error {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("json error {0}")]
    JsonError(#[from] SerdeError),
}

pub fn load_funscript(path: &str) -> Result<FScript, FunscriptError> {
    let file = std::fs::read_to_string(path)?;
    let json = serde_json::from_str::<Value>(&file)?;
    Ok(json)
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
}
