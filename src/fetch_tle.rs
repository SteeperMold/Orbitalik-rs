use serde::{Serialize, Deserialize};
use std::io::Read;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, thiserror::Error)]
pub enum TleFetchingError {
    #[error("Tle request failed with error")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to save tle file")]
    FileSavingError(#[from] tokio::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchingSettings {
    pub tle_urls: Vec<String>,
    pub delay_seconds: u64,
    pub do_track_everything: bool,
    pub satellites_to_track: Vec<String>,
}

pub async fn read_settings() -> FetchingSettings {
    let path = std::env::var("TLE_FETCHING_SETTINGS_PATH")
        .expect("TLE_FETCHING_SETTINGS_PATH env variable should be set");

    let mut file = tokio::fs::File::open(path).await
        .expect("Tle fetching settings should be created");

    let mut content = String::new();
    file.read_to_string(&mut content).await
        .expect("Tle fetching settings file shouldn't be corrupted");

    serde_json::from_str(&content).expect("Json should be well-formatted")
}

pub fn read_settings_sync() -> FetchingSettings {
    let path = std::env::var("TLE_FETCHING_SETTINGS_PATH")
        .expect("TLE_FETCHING_SETTINGS_PATH env variable should be set");

    let mut file = std::fs::File::open(path)
        .expect("Tle fetching settings should be created");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Tle fetching settings file shouldn't be corrupted");

    serde_json::from_str(&content).expect("Json should be well-formatted")
}

async fn write_settings(settings: &FetchingSettings) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(settings)
        .expect("Serializing of struct with simple types shouldn't fail");

    let path = std::env::var("TLE_FETCHING_SETTINGS_PATH")
        .expect("TLE_FETCHING_SETTINGS_PATH env variable should be set");

    let temporary_file_path = format!("{}.tmp", path.split('.').collect::<Vec<_>>()[0]);

    let mut temporary_file = tokio::fs::File::create(&temporary_file_path).await?;
    temporary_file.write_all(json.as_bytes()).await?;
    temporary_file.sync_all().await?;

    while let Err(error) = tokio::fs::rename(&temporary_file_path, &path).await {
        log::warn!("Failed to rewrite tle fetching settings\
                    file with this error: {}; trying again...", error);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    };

    Ok(())
}

pub async fn fetch_tle(settings: &mut FetchingSettings) -> Result<(), TleFetchingError> {
    let mut all_tle = String::new();

    for url in &settings.tle_urls {
        let response_text = reqwest::get(url).await?.text().await?;
        all_tle.push_str(&format!("{}\n", response_text));
    }

    let mut filtered_tle = String::new();

    if settings.do_track_everything {
        // Добавление списка спутников в файл с настройками, чтобы иметь возможность
        // получить список всех спутников в дальнейшем

        let lines: Vec<&str> = all_tle.lines().collect();
        let mut satellite_names = vec![];

        for i in (0..lines.len()).step_by(3) {
            let satellite_name = lines[i].trim().to_string();

            if !satellite_name.is_empty() {
                satellite_names.push(satellite_name);
            }
        }

        settings.satellites_to_track = satellite_names;
        write_settings(settings).await?;

        filtered_tle = all_tle;
    } else {
        let lines: Vec<&str> = all_tle.lines().collect();

        for i in (0..lines.len()).step_by(3) {
            let satellite_name = lines[i].trim().to_string();

            if !satellite_name.is_empty() &&
                settings.satellites_to_track.contains(&satellite_name) {
                for shift in 0..=2 {
                    filtered_tle.push_str(&format!("{}\n", lines[i + shift]));
                }
            }
        }
    }

    let tle_file_path = std::env::var("TLE_FILE_PATH")
        .expect("TLE_FILE_PATH env variable should be set");

    let temporary_file_path = format!("{}.tmp", tle_file_path.split('.').collect::<Vec<_>>()[0]);

    let mut temporary_file = tokio::fs::File::create(&temporary_file_path).await?;
    temporary_file.write_all(filtered_tle.as_bytes()).await?;
    temporary_file.sync_all().await?;

    while let Err(error) = tokio::fs::rename(&temporary_file_path, &tle_file_path).await {
        log::warn!("Failed to rewrite tle file with error: {}; trying again...", error);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Ok(())
}