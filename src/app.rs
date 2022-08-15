use crate::fan_data::FanData;
use crate::sqlite::{Database, Fan};
use chrono::{DateTime, Utc};
pub struct App {
    // pub titles: Vec<&'a str>,
    pub index: usize,
    pub database: Database,
    pub fan_data: Option<FanData>,
    pub fans: Vec<Fan>,
    last_change: DateTime<Utc>,
}

impl App {
    pub fn new(database_path: String) -> Result<App, rusqlite::Error> {
        let database = Database::new(&database_path)?;
        let fans = match database.get_fans() {
            Ok(fans) => fans,
            Err(error) => {
                let error_string = error.to_string();
                println!("Error: {}", { error_string });
                panic!("Failed fetching fans")
            }
        };
        let fan_data = match database.get_last_fan_data(1) {
            Ok(data) => data,
            Err(error) => {
                println!("{error:?}");
                None
            }
        };
        Ok(App {
            fans,
            index: 0,
            database,
            fan_data,
            last_change: chrono::offset::Utc::now(),
        })
    }

    fn on_change(&mut self) {
        self.last_change = chrono::offset::Utc::now();
        self.update_fan_data();
    }

    pub fn next(&mut self) {
        // Limit to 1 tab change per second
        let now = chrono::offset::Utc::now();
        if now.timestamp_millis() - 500 > self.last_change.timestamp_millis() {
            self.index = (self.index + 1) % self.fans.len();
            self.on_change();
        }
    }

    pub fn previous(&mut self) {
        // Limit to 1 tab change per second
        let now = chrono::offset::Utc::now();
        if now.timestamp_millis() - 500 > self.last_change.timestamp_millis() {
            if self.index > 0 {
                self.index -= 1;
            } else {
                self.index = self.fans.len() - 1;
            }
            self.on_change();
        }
    }

    pub fn update_fan_data(&mut self) {
        self.fan_data = match self.database.get_last_fan_data(self.index + 1) {
            Ok(data) => data,
            Err(error) => {
                println!("{error:?}");
                None
            }
        };
    }

    // #[cfg(target_os = "linux")]
    // pub fn restart_homebase_service() {
    //     let output = Command::new("sudo systemctl restart rpi-homebase")
    //         .args([])
    //         .output();
    // }
}
