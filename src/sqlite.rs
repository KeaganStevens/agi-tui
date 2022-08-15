use crate::fan_data::FanData;
use rusqlite::{Connection, Error, OpenFlags, Result};
#[derive(Clone, Debug)]
pub struct Fan {
    pub name: String,
    pub serial_number: String,
    pub mac: Option<String>,
}
// #[derive(Clone, Debug)]
// pub enum ControlMethods {
//     Manual = 0,
//     TemperatureControl = 1,
//     Auto = 2,
//     ManualOn = 3,
//     ManualOff = 4,
// }

// #[derive(Clone, Debug)]
// pub struct FanData {
//     // Panel Switch & Door Status
//     pub main_panel_open: bool,
//     // control_door: bool,
//     pub main_switch: bool,

//     // Fan Status
//     pub vsd_error: bool,
//     pub vsd_running: bool,
//     pub vsd_command: bool,

//     // Temperature Data
//     pub temperature_top: f32,
//     pub temperature_bottom: f32,
//     pub temperature_far: f32,

//     // Fan Information
//     pub signal_strength: f32,
//     pub control_method: u8,
//     // pub last_update: DateTime<FixedOffset>,
//     pub last_update: String,

//     // Fan Data
//     pub motor_vibration: f32,
//     pub motor_current: f32,
//     // main_state: bool,

//     // Environmental Data
//     pub humidity: f32,
//     pub wind_speed: f32,
//     pub wind_direction: i16,
// }

pub struct Database {
    database_path: String,
}

impl Database {
    pub fn new(database_path: &str) -> Result<Self, rusqlite::Error> {
        Connection::open_with_flags(database_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        // conn.close();
        let path = database_path.to_string();
        Ok(Self {
            database_path: path,
        })
    }

    fn get_connection(&self) -> Result<Connection, Error> {
        let conn =
            Connection::open_with_flags(&self.database_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        Ok(conn)
    }

    pub fn get_fans(&self) -> Result<Vec<Fan>, rusqlite::Error> {
        let connection = self.get_connection()?;
        let mut stmt = connection.prepare("SELECT devices.sn,name,mac FROM devices LEFT JOIN fan_sn_mac ON devices.sn=fan_sn_mac.sn WHERE type=1")?;
        let fans_iter = stmt.query_map([], |row| {
            Ok(Fan {
                serial_number: row.get("sn")?,
                name: row.get("name")?,
                mac: row.get("mac")?,
            })
        })?;
        let mut fans: Vec<Fan> = Vec::new();
        for fan in fans_iter {
            match fan {
                Ok(f) => fans.push(f),
                Err(_error) => println!("Something wrong with result"),
            }
        }
        Ok(fans)
    }

    pub fn get_last_fan_data(&self, fan_id: usize) -> Result<Option<FanData>, rusqlite::Error> {
        let connection = self.get_connection()?;
        let table_name = format!("Fan{}", fan_id);
        let sql = format!(
            "SELECT * FROM {} ORDER BY datetime DESC LIMIT 1",
            table_name
        );
        let mut stmt = connection.prepare(&sql)?;
        let has_data = stmt.exists([])?;
        if has_data == false {
            return Ok(None);
        }
        // let col_names = stmt.column_names();
        // println!("{col_names:?}");
        let fan_data = stmt.query_row([], |row| {
            Ok(FanData {
                // last_update: parse_datetime(row.get("datetime")?)?,
                temperature_top: row.get("th")?,
                temperature_bottom: row.get("tl")?,
                temperature_far: row.get("tf")?,
                humidity: row.get("rh")?,
                wind_speed: row.get("ws")?,
                wind_direction: row.get("wd")?,
                battery_voltage: row.get("bv")?,
                control_door: row.get("cmpd")?,
                main_panel_open: row.get("mpd")?,
                motor_vibration: row.get("vs")?,
                control_method: row.get("om")?,
                // main_state: row.get("ip")?,
                rain_meter: row.get("ass")?,
                main_switch: row.get("ms")?,
                motor_current: row.get("mc")?,
                vsd_error: row.get("ves")?,
                vsd_command: row.get("vcmd")?,
                vsd_running: row.get("vrs")?,
                signal_strength: row.get("rssi")?,
                last_update: row.get("datetime")?,
            })
        })?;
        Ok(Some(fan_data))
    }
}

impl Fan {
    pub fn get_name(&self) -> &str {
        let name = &self.name;
        name
    }
}

// impl FanData {
//     pub fn get_temperature_top_string(&self) -> String {
//         let value = self.temperature_top;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}℃", value.to_string()),
//         }
//     }

//     pub fn get_temperature_bottom_string(&self) -> String {
//         let value = self.temperature_bottom;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}℃", value.to_string()),
//         }
//     }

//     pub fn get_temperature_far_string(&self) -> String {
//         let value = self.temperature_far;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}℃", value.to_string()),
//         }
//     }

//     pub fn get_motor_current_string(&self) -> String {
//         let value = self.motor_current;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}A", value.to_string()),
//         }
//     }

//     pub fn get_motor_vibration_string(&self) -> String {
//         let value = self.motor_vibration;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}mm/s", value.to_string()),
//         }
//     }

//     pub fn get_ready_string(&self) -> String {
//         let value = self.vsd_error;
//         match value {
//             true => String::from("Error"),
//             false => String::from("Ready"),
//         }
//     }

//     pub fn get_running_string(&self) -> String {
//         let value = self.vsd_running;
//         match value {
//             true => String::from("Running"),
//             false => String::from("Idle"),
//         }
//     }

//     pub fn get_command_string(&self) -> String {
//         let value = self.vsd_command;
//         match value {
//             true => String::from("On"),
//             false => String::from("Off"),
//         }
//     }

//     pub fn get_operating_mode_string(&self) -> String {
//         let value = self.control_method;
//         match value {
//             0 => String::from("Manual"),
//             1 => String::from("Temperature Control"),
//             2 => String::from("C&M Auto"),
//             3 => String::from("C&M Manual On"),
//             4 => String::from("C&M Manual Off"),
//             _ => String::from("Unknown"),
//         }
//     }

//     pub fn get_humidity_string(&self) -> String {
//         let value = self.humidity;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}%", value.to_string()),
//         }
//     }

//     pub fn get_wind_speed_string(&self) -> String {
//         let value = self.wind_speed;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}m/s", value.to_string()),
//         }
//     }

//     pub fn get_wind_direction_string(&self) -> String {
//         let value = self.wind_direction;
//         match value {
//             -49 => String::from("Disabled"),
//             -50 => String::from("Error"),
//             _ => format!("{}°", value.to_string()),
//         }
//     }

//     pub fn get_signal_strength_string(&self) -> String {
//         let value = self.signal_strength;
//         match value {
//             -49.0 => String::from("Disabled"),
//             -50.0 => String::from("Error"),
//             _ => format!("{}dBm", value.to_string()),
//         }
//     }

//     pub fn get_main_panel_string(&self) -> String {
//         let value = self.main_panel_open;
//         match value {
//             true => String::from("Closed"),
//             false => String::from("Open"),
//         }
//     }

//     pub fn get_main_switch_string(&self) -> String {
//         let value = self.main_switch;
//         match value {
//             true => String::from("On"),
//             false => String::from("Off"),
//         }
//     }

//     pub fn get_last_update_string(&self) -> String {
//         let datetime = &self.last_update;
//         let parsed_datetime_result =
//             chrono::NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S");
//         if parsed_datetime_result.is_err() {
//             return String::from("Parsing error");
//         }
//         let now = chrono::offset::Local::now().timestamp();
//         let parsed_datetime = parsed_datetime_result.unwrap().timestamp();
//         if now < parsed_datetime {
//             return String::from("Error: last update later than now");
//         }
//         let difference_seconds = now - parsed_datetime;
//         match difference_seconds {
//             seconds if seconds > 60 * 60 => {
//                 let hours = difference_seconds / (60 * 60);
//                 format!("{hours} Hours Ago")
//             }
//             seconds if seconds < 60 * 60 => {
//                 let minutes = difference_seconds / (60);
//                 format!("{minutes} Minutes Ago")
//             }
//             _ => String::from("Matching error"),
//         }
//     }
// }
