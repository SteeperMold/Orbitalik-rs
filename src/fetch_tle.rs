use serde::Deserialize;
use tokio::io::AsyncWriteExt;

#[derive(Debug, thiserror::Error)]
pub enum TleFetchingError {
    #[error("Tle request failed with error")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to save tle file")]
    FileSavingError(#[from] tokio::io::Error),
}

#[derive(Deserialize, Debug)]
pub struct FetchingSettings {
    tle_urls: Vec<String>,
    pub delay_seconds: u64,
    do_track_everything: bool,
    satellites_to_track: Vec<String>,
}

pub fn read_settings(file_path: &str) -> FetchingSettings {
    let file = std::fs::File::open(file_path).expect("Tle fetching settings not found");
    serde_json::from_reader(file).expect("Json was not well-formatted")
}

pub async fn fetch_tle(settings: &FetchingSettings) -> Result<(), TleFetchingError> {
    let mut all_tle = String::new();

    for url in &settings.tle_urls {
        let response_text = reqwest::get(url).await?.text().await?;
        all_tle.push_str(&format!("{}\n", response_text));
    }

    let mut filtered_tle = String::new();

    if settings.do_track_everything {
        filtered_tle = all_tle;
    } else {
        let lines: Vec<&str> = all_tle.lines().collect();

        for i in (0..lines.len()).step_by(3) {
            let satellite_name = lines[i].trim().to_string();

            if settings.satellites_to_track.contains(&satellite_name) {
                for shift in 0..=2 {
                    filtered_tle.push_str(&format!("{}\n", lines[i + shift]));
                }
            }
        }
    }

    let mut tmp_file = tokio::fs::File::create("data/tle.tmp").await?;
    tmp_file.write_all(filtered_tle.as_bytes()).await?;
    tmp_file.sync_all().await?;

    while let Err(_) = tokio::fs::rename("data/tle.tmp", "data/tle.txt").await {
        log::warn!("Tle file is opened, trying again...");
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }

    Ok(())
}