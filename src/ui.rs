use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    // Top Level Layout ( splits Tabs & Tab Content)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
    f.render_widget(block, size);

    let mut titles: Vec<&str> = Vec::new();
    for fan in &app.fans {
        let fan_name = &fan.get_name();
        let title = fan_name.clone();
        titles.push(title);
    }
    // Tab Titles
    let titles = titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::White))))
        .collect();

    // Creates Tabs and changes selected tab style
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Fans"))
        .select(app.index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::UNDERLINED)
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightBlue)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    let (left_chunks, middle_chunks, right_chunks) = get_tab_content_chunks(chunks[1]);

    // Left Blocks
    let left_middle = Block::default().title("Middle Left").borders(Borders::ALL);
    f.render_widget(left_middle, left_chunks[1]);

    let left_bottom = Block::default().title("Bottom Left").borders(Borders::ALL);
    f.render_widget(left_bottom, left_chunks[2]);

    // Top Left
    // Information Block Container
    let left_top = Block::default()
        .title("General Information")
        .borders(Borders::ALL);
    f.render_widget(left_top, left_chunks[0]);
    let (battery_voltage, connection_status, last_update) = get_information_values(&app);
    let left_top_inner = get_block_content_chunks(left_chunks[0]);
    let battery_voltage = render_block_with_content("Battery Voltage", &battery_voltage);
    let connection_status = render_block_with_content("Connection Status", &connection_status);
    let last_update = render_block_with_content("Last Update", &last_update);
    f.render_widget(battery_voltage, left_top_inner[1]);
    f.render_widget(connection_status, left_top_inner[2]);
    f.render_widget(last_update, left_top_inner[3]);

    // Left Middle
    // Fan Status Block Container
    let left_top = Block::default().title("Fan Status").borders(Borders::ALL);
    f.render_widget(left_top, left_chunks[1]);
    let (running, error, command) = get_fan_status_values(&app);
    let left_middle_inner = get_block_content_chunks(left_chunks[1]);
    let last_update = render_block_with_content("Ready Status", &error);
    let signal_strength = render_block_with_content("Running Status", &running);
    let control_mode = render_block_with_content("C&M Command", &command);
    f.render_widget(last_update, left_middle_inner[1]);
    f.render_widget(signal_strength, left_middle_inner[2]);
    f.render_widget(control_mode, left_middle_inner[3]);

    // Left Bottom
    // Panel Switch & Door Status Block Container
    let left_top = Block::default()
        .title("Panel Switch & Door Status")
        .borders(Borders::ALL);
    f.render_widget(left_top, left_chunks[2]);
    let (main_switch, main_panel_door, cm_panel_door) = get_panel_door_values(&app);
    let left_bottom_inner = get_block_content_chunks(left_chunks[2]);
    let main_switch = render_block_with_content("Main Switch", &main_switch);
    let main_panel_door = render_block_with_content("Main Panel Door", &main_panel_door);
    let cm_panel_door = render_block_with_content("C&M Panel Door", &cm_panel_door);
    f.render_widget(main_switch, left_bottom_inner[1]);
    f.render_widget(main_panel_door, left_bottom_inner[2]);
    f.render_widget(cm_panel_door, left_bottom_inner[3]);

    // Top Middle
    // Temperature block
    let middle_top = render_block("Temperature");
    f.render_widget(middle_top, middle_chunks[0]);
    let (temperature_top, temperature_bottom, temperature_far) = get_temperature_values(&app);
    let middle_top_inner = get_block_content_chunks(middle_chunks[0]);
    let temperature_top = render_block_with_content("Temperature Top", &temperature_top);
    let temperature_bottom = render_block_with_content("Temperature Bottom", &temperature_bottom);
    let temperature_far = render_block_with_content("Temperature Far", &temperature_far);
    f.render_widget(temperature_top, middle_top_inner[1]);
    f.render_widget(temperature_bottom, middle_top_inner[2]);
    f.render_widget(temperature_far, middle_top_inner[3]);

    // Middle Middle
    // Fan Data
    let middle_middle = render_block("Fan Data");
    f.render_widget(middle_middle, middle_chunks[1]);
    let (motor_current, motor_vibration, main_state) = get_fan_data_values(&app);
    let middle_middle_inner = get_block_content_chunks(middle_chunks[1]);
    let motor_current = render_block_with_content("Motor Current", &motor_current);
    let motor_vibration = render_block_with_content("Motor Vibration", &motor_vibration);
    let main_state = render_block_with_content("Main State", &main_state);
    f.render_widget(motor_current, middle_middle_inner[1]);
    f.render_widget(motor_vibration, middle_middle_inner[2]);
    f.render_widget(main_state, middle_middle_inner[3]);

    // Middle Bottom
    // Main Panel Status
    let middle_bottom = render_block("Main Panel Status");
    f.render_widget(middle_bottom, middle_chunks[2]);
    let (operating_mode, running_status) = get_main_panel_values(&app);
    let middle_middle_inner = get_block_content_chunks(middle_chunks[2]);
    let operating_mode = render_block_with_content("Control Method", &operating_mode);
    let running_status = render_block_with_content("Fan Status", &running_status);
    f.render_widget(operating_mode, middle_middle_inner[1]);
    f.render_widget(running_status, middle_middle_inner[2]);

    // let middle_bottom = render_block("Middle Bottom");
    // f.render_widget(middle_bottom, middle_chunks[2]);

    // Right Top
    // Environmental Data
    let right_top = render_block("Environmental Data");
    f.render_widget(right_top, right_chunks[0]);
    let (humidity, wind_speed, wind_direction) = get_environment_values(&app);
    let middle_right_inner = get_block_content_chunks(right_chunks[0]);
    let humidity = render_block_with_content("Humidity", &humidity);
    let wind_speed = render_block_with_content("Wind Speed", &wind_speed);
    let wind_direction = render_block_with_content("Wind Direction", &wind_direction);
    f.render_widget(humidity, middle_right_inner[1]);
    f.render_widget(wind_speed, middle_right_inner[2]);
    f.render_widget(wind_direction, middle_right_inner[3]);

    // Right Middle
    // Fan Data
    let middle_right = render_block("Additional Data");
    f.render_widget(middle_right, right_chunks[1]);
    let (signal_strength, rain_meter) = get_additional_values(&app);
    let middle_right_inner = get_block_content_chunks(right_chunks[1]);
    let signal_strength = render_block_with_content("Signal Strength", &signal_strength);
    let rain_meter = render_block_with_content("Rain Meter", &rain_meter);
    f.render_widget(signal_strength, middle_right_inner[1]);
    f.render_widget(rain_meter, middle_right_inner[2]);

    // let right_bottom = render_block("Bottom Right");
    // f.render_widget(right_bottom, right_chunks[2]);
}

fn render_block(title: &str) -> Block {
    let block = Block::default().title(title).borders(Borders::ALL);
    block
}

fn render_block_with_content<'a>(title: &'a str, content: &'a str) -> Paragraph<'a> {
    let block = Block::default()
        .style(Style::default().fg(Color::Blue))
        .title(title)
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default())
        .alignment(Alignment::Center);
    paragraph
}

fn get_temperature_values(app: &App) -> (String, String, String) {
    let fan_data = &app.fan_data;
    let (temp_high, temp_low, temp_far) = match fan_data {
        Some(dta) => (
            dta.get_temperature_top_string(),
            dta.get_temperature_bottom_string(),
            dta.get_temperature_far_string(),
        ),
        None => (
            String::from("No Data"),
            String::from("No Data"),
            String::from("No Data"),
        ),
    };
    (temp_high, temp_low, temp_far)
}

fn get_information_values(app: &App) -> (String, String, String) {
    let fan_data = &app.fan_data;
    match fan_data {
        Some(dta) => (
            dta.get_voltage_string(),
            dta.get_connection_status_string(),
            dta.get_last_update_string(),
        ),
        None => (
            String::from("No Data"),
            String::from("No Data"),
            String::from("No Data"),
        ),
    }
}

fn get_fan_status_values(app: &App) -> (String, String, String) {
    let fan_data = &app.fan_data;
    match fan_data {
        Some(dta) => (
            dta.get_running_string(),
            dta.get_ready_string(),
            dta.get_command_string(),
        ),
        None => (
            String::from("No Data"),
            String::from("No Data"),
            String::from("No Data"),
        ),
    }
}

fn get_environment_values(app: &App) -> (String, String, String) {
    let fan_data = &app.fan_data;
    match fan_data {
        Some(dta) => (
            dta.get_humidity_string(),
            dta.get_wind_speed_string(),
            dta.get_wind_direction_string(),
        ),
        None => (
            String::from("No Data"),
            String::from("No Data"),
            String::from("No Data"),
        ),
    }
}

fn get_additional_values(app: &App) -> (String, String) {
    let fan_data = &app.fan_data;
    match fan_data {
        Some(dta) => (
            dta.get_signal_strength_string(),
            dta.get_rain_meter_string(),
        ),
        None => (String::from("No Data"), String::from("No Data")),
    }
}
// TODO Fix Main State
fn get_fan_data_values(app: &App) -> (String, String, String) {
    let fan_data = &app.fan_data;
    match fan_data {
        Some(dta) => (
            dta.get_motor_current_string(),
            dta.get_motor_vibration_string(),
            dta.get_main_switch_string(),
        ),
        None => (
            String::from("No Data"),
            String::from("No Data"),
            String::from("No Data"),
        ),
    }
}

fn get_panel_door_values(app: &App) -> (String, String, String) {
    let fan_data = &app.fan_data;
    match fan_data {
        Some(dta) => (
            dta.get_main_switch_string(),
            dta.get_main_panel_string(),
            dta.get_control_door_string(),
        ),
        None => (
            String::from("No Data"),
            String::from("No Data"),
            String::from("No Data"),
        ),
    }
}

fn get_main_panel_values(app: &App) -> (String, String) {
    let fan_data = &app.fan_data;
    match fan_data {
        Some(dta) => (
            dta.get_operating_mode_string(),
            dta.get_running_status_string(),
        ),
        None => (String::from("No Data"), String::from("No Data")),
    }
}

fn get_block_content_chunks(chunk: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(2),
                Constraint::Percentage(32),
                Constraint::Percentage(32),
                Constraint::Percentage(32),
                Constraint::Percentage(2),
            ]
            .as_ref(),
        )
        .split(chunk)
}

fn get_tab_content_chunks(chunk: Rect) -> (Vec<Rect>, Vec<Rect>, Vec<Rect>) {
    // Splits chunks into 3 equal vertical portions
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunk);

    // Left Layout
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(horizontal_chunks[0]);

    // Middle Layout
    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(horizontal_chunks[1]);

    // Right Layout
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(horizontal_chunks[2]);

    (left_chunks, middle_chunks, right_chunks)
}
