use color_eyre::{Result, eyre::{OptionExt, WrapErr}};
use nmrs::{Network, NetworkManager};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Frame, Terminal};

pub struct AppData {
    status_text: String,
    networks: Vec<Network>,
    connected_ssid: String,
    is_loading: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app_data = AppData {
        status_text: String::from("Initializing..."),
        networks: Vec::new(),
        connected_ssid: String::from("..."),
        is_loading: true,
    };
    let result = run(&mut terminal, &mut app_data).await;
    ratatui::restore();
    result
}

async fn run(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app_data: &mut AppData) -> Result<()> {
    let (networks, ssid) = get_nm_info().await?;
    app_data.networks = networks;
    app_data.connected_ssid = ssid;
    app_data.is_loading = false;

    loop {
        terminal.draw(|frame| render(frame, app_data))?;
        if crossterm::event::read()?.is_key_press() {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_data: &AppData) {
    let text = if app_data.is_loading {
        &app_data.status_text
    } else {
        &app_data.connected_ssid
    };

    frame.render_widget(Paragraph::new::<String>(text.to_string()), frame.area());
}

async fn get_nm_info() -> Result<(Vec<Network>, String)> {
    let nm = NetworkManager::new().await?;

    let networks = nm
        .list_networks(None)
        .await
        .wrap_err("Failed to list networks")?;

    let ssid = nm
        .current_ssid()
        .await
        .ok_or_eyre("Disconnected")
        .wrap_err("Failed to get current SSID")?;

    Ok((networks, ssid))
}
