use ratatui::{Frame, widgets::Paragraph};

use crate::AppState;

pub fn render(frame: &mut Frame, app_state: &AppState) {
    let wifi_text = app_state
        .wifi
        .connected_ssid
        .clone()
        .unwrap_or_else(|| String::from("Disconnected"));

    let mut text = format!(
        "{status}\n{wifi}\n",
        status = app_state.status_text,
        wifi = wifi_text
    );

    for network in &app_state.wifi.networks {
        let strength = network
            .strength
            .map(|s| format!("{}%", s))
            .unwrap_or_else(|| "N/A".into());
        text.push_str(&format!(
            "{ssid} ({strength})\n",
            ssid = network.ssid,
            strength = strength
        ));
    }

    frame.render_widget(Paragraph::new(text), frame.area());
}
