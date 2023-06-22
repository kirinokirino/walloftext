use confargenv::fusion;

use std::collections::HashMap;
use std::default::Default;
use std::error::Error;

pub struct Config {
    path: Option<String>,
    pub title: String,
    pub sleep_ms_per_frame: u64,
    pub window_width: u32,
    pub window_height: u32,
    pub grid_width: u32,
    pub grid_height: u32,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let mut config = Self {
            path: Some(path.to_string()),
            ..Default::default()
        };
        config = match config.reload() {
            Ok(config) => config,
            Err(err) => {
                eprintln!("{err}");
                Self {
                    path: Some(path.to_string()),
                    ..Default::default()
                }
            }
        };
        config
    }

    pub fn reload(self) -> Result<Self, Box<dyn Error>> {
        let defaults = HashMap::from([
            ("title", "FLOATING"),
            ("sleep_ms_per_frame", "5"),
            ("window_width", "640"),
            ("window_height", "480"),
            ("grid_width", "8"),
            ("grid_height", "16"),
        ]);
        let config_map = fusion(defaults, Some(self.path.clone().unwrap().as_str()));
        let title = config_map.get("title").unwrap().to_string();
        let sleep_ms_per_frame = config_map
            .get("sleep_ms_per_frame")
            .unwrap()
            .parse::<u64>()?;
        let window_width = config_map.get("window_width").unwrap().parse::<u32>()?;
        let window_height = config_map.get("window_height").unwrap().parse::<u32>()?;
        let grid_width = config_map.get("grid_width").unwrap().parse::<u32>()?;
        let grid_height = config_map.get("grid_height").unwrap().parse::<u32>()?;
        Ok(Self {
            path: self.path,
            title,
            sleep_ms_per_frame,
            window_height,
            window_width,
            grid_width,
            grid_height,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: None,
            title: "FLOATING".to_string(),
            sleep_ms_per_frame: 5,
            window_width: 640,
            window_height: 640,
            grid_width: 8,
            grid_height: 16,
        }
    }
}
