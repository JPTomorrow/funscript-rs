use mp4::Result;
use std::{fs::File, io::BufReader};

pub fn get_video_sample_count(path: &str) -> Result<u32> {
    let f = File::open(path)?;
    let size = f.metadata()?.len();
    let reader = BufReader::new(f);

    let mp4 = mp4::Mp4Reader::read_header(reader, size)?;
    mp4.sample_count(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_video_sample_count() {
        let path = "./test-scripts/openfunscripter.mp4";
        let sample_count = get_video_sample_count(path).unwrap();
        assert_eq!(sample_count, 156446);
    }
}
