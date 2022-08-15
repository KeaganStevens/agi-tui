// #[derive(Clone, Debug)]
// pub enum FieldState {
//     Okay = 0,
//     Error = -50,
//     Disabled = -49,
// }

#[derive(Clone, Debug)]
pub struct FanData {
    // Panel Switch & Door Status
    pub main_panel_open: bool,
    pub control_door: bool,
    pub main_switch: bool,

    // Fan Status
    pub vsd_error: bool,
    pub vsd_running: bool,
    pub vsd_command: bool,

    // Temperature Data
    pub temperature_top: f32,
    pub temperature_bottom: f32,
    pub temperature_far: f32,

    // Fan Information
    pub signal_strength: f32,
    pub control_method: u8,
    pub battery_voltage: f32,
    pub last_update: String,

    // Fan Data
    pub motor_vibration: f32,
    pub motor_current: f32,
    // main_state: bool,

    // Environmental Data
    pub humidity: f32,
    pub wind_speed: f32,
    pub wind_direction: i16,
    pub rain_meter: f32,
}

impl FanData {
    pub fn get_temperature_top_string(&self) -> String {
        let value = self.temperature_top;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}℃", value.to_string()),
        }
    }

    pub fn get_temperature_bottom_string(&self) -> String {
        let value = self.temperature_bottom;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}℃", value.to_string()),
        }
    }

    pub fn get_temperature_far_string(&self) -> String {
        let value = self.temperature_far;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}℃", value.to_string()),
        }
    }

    pub fn get_motor_current_string(&self) -> String {
        let value = self.motor_current;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}A", value.to_string()),
        }
    }

    pub fn get_motor_vibration_string(&self) -> String {
        let value = self.motor_vibration;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}mm/s", value.to_string()),
        }
    }

    pub fn get_ready_string(&self) -> String {
        let value = self.vsd_error;
        match value {
            true => String::from("Error"),
            false => String::from("Ready"),
        }
    }

    pub fn get_running_string(&self) -> String {
        let value = self.vsd_running;
        match value {
            true => String::from("Running"),
            false => String::from("Idle"),
        }
    }

    pub fn get_command_string(&self) -> String {
        let value = self.vsd_command;
        match value {
            true => String::from("On"),
            false => String::from("Off"),
        }
    }

    pub fn get_operating_mode_string(&self) -> String {
        let value = self.control_method;
        match value {
            0 => String::from("Manual"),
            1 => String::from("Temperature Control"),
            2 => String::from("C&M Auto"),
            3 => String::from("C&M Manual On"),
            4 => String::from("C&M Manual Off"),
            _ => String::from("Unknown"),
        }
    }

    pub fn get_humidity_string(&self) -> String {
        let value = self.humidity;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}%", value.to_string()),
        }
    }

    pub fn get_voltage_string(&self) -> String {
        let value = self.battery_voltage;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}V", value.to_string()),
        }
    }

    pub fn get_wind_speed_string(&self) -> String {
        let value = self.wind_speed;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}m/s", value.to_string()),
        }
    }

    pub fn get_wind_direction_string(&self) -> String {
        let value = self.wind_direction;
        match value {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}°", value.to_string()),
        }
    }

    pub fn get_signal_strength_string(&self) -> String {
        let value = self.signal_strength;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}dBm", value.to_string()),
        }
    }

    pub fn get_rain_meter_string(&self) -> String {
        let value = self.rain_meter;
        let int_value = value as i16;
        match int_value as i16 {
            -49 => String::from("Disabled"),
            -50 => String::from("Error"),
            _ => format!("{}mm", value.to_string()),
        }
    }

    pub fn get_main_panel_string(&self) -> String {
        let value = self.main_panel_open;
        match value {
            true => String::from("Closed"),
            false => String::from("Open"),
        }
    }

    pub fn get_control_door_string(&self) -> String {
        let value = self.control_door;
        match value {
            true => String::from("Closed"),
            false => String::from("Open"),
        }
    }

    pub fn get_main_switch_string(&self) -> String {
        let value = self.main_switch;
        match value {
            true => String::from("On"),
            false => String::from("Off"),
        }
    }

    pub fn get_last_update_string(&self) -> String {
        let seconds_since_last_result = self.get_seconds_since_last();
        if seconds_since_last_result.is_err() {
            return String::from("Parsing error");
        };
        let difference_seconds = seconds_since_last_result.unwrap();
        match difference_seconds {
            difference_seconds if difference_seconds > 60 * 60 * 24 => {
                let hours = difference_seconds / (60 * 60 * 24);
                format!("{hours} Days Ago")
            }
            difference_seconds if difference_seconds > 60 * 60 => {
                let hours = difference_seconds / (60 * 60);
                format!("{hours} Hours Ago")
            }
            difference_seconds if difference_seconds < 60 * 60 => {
                let minutes = difference_seconds / (60);
                format!("{minutes} Minutes Ago")
            }
            _ => String::from("Matching error"),
        }
    }

    pub fn get_connection_status_string(&self) -> String {
        let seconds_since_last_result = self.get_seconds_since_last();
        if seconds_since_last_result.is_err() {
            return String::from("Parsing error");
        };
        let seconds_since = seconds_since_last_result.unwrap();
        match seconds_since {
            seconds_since if seconds_since <= 60 * 10 => String::from("Online"),
            seconds_since if seconds_since > 60 * 10 => String::from("Offline"),
            _ => String::from("Matching error"),
        }
    }

    pub fn get_running_status_string(&self) -> String {
        if self.vsd_running == false {
            return String::from("Not Running");
        }
        let seconds_since_last_result = self.get_seconds_since_last();
        if seconds_since_last_result.is_err() {
            return String::from("Parsing error");
        };
        let seconds_since = seconds_since_last_result.unwrap();
        match seconds_since {
            seconds_since if seconds_since <= 60 * 10 => String::from("Running"),
            seconds_since if seconds_since > 60 * 10 => String::from("Not Running"),
            _ => String::from("Matching error"),
        }
    }

    fn get_seconds_since_last(&self) -> Result<i64, chrono::ParseError> {
        let datetime = &self.last_update;
        let parsed_datetime =
            chrono::NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S")?.timestamp();
        let now = chrono::offset::Local::now().timestamp();
        let difference_seconds = now - parsed_datetime;
        Ok(difference_seconds)
    }
}
