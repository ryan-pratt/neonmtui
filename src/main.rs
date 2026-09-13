use color_eyre::{Result, eyre::WrapErr};
use crossterm::event::{Event, EventStream, KeyEvent};
use futures_util::StreamExt;
use nmrs::{Network, NetworkManager};
use ratatui::{Frame, Terminal, backend::CrosstermBackend, widgets::Paragraph};
use std::time::Duration;
use tokio::sync::mpsc;

enum AppEvent {
    Wifi(WifiEvent),
    Key(KeyEvent),
}

enum WifiEvent {
    NetworkListUpdated(Vec<Network>),
    ConnectionUpdated(Option<String>),
}

struct AppState {
    wifi: WifiState,
    status_text: String,
    is_running: bool,
}

impl AppState {
    fn handle(&mut self, event: AppEvent) {
        match event {
            AppEvent::Key(_) => {
                self.is_running = false;
            }

            AppEvent::Wifi(WifiEvent::ConnectionUpdated(con)) => {
                self.wifi.connected_ssid = con;
                self.status_text = String::from("WiFi connection updated");
            }

            AppEvent::Wifi(WifiEvent::NetworkListUpdated(nets)) => {
                self.wifi.networks = nets;
                self.status_text = String::from("WiFi available networks updated")
            }
        }
    }
}

struct WifiState {
    networks: Vec<Network>,
    connected_ssid: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let wifi = WifiState {
        networks: Vec::new(),
        connected_ssid: None,
    };
    let mut app_state = AppState {
        wifi,
        status_text: String::from("Initializing..."),
        is_running: true,
    };
    let result = run(&mut terminal, &mut app_state).await;
    ratatui::restore();
    result
}

async fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app_state: &mut AppState,
) -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let mut crossterm_events = EventStream::new();

    let con_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            let res: Result<()> = async {
                let connected_ssid = get_wifi_connection().await?;
                let event = AppEvent::Wifi(WifiEvent::ConnectionUpdated(connected_ssid));
                con_tx.send(event).await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
                Ok(())
            }
            .await;

            if res.is_err() {
                break;
            }
        }
    });

    let net_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            let res: Result<()> = async {
                let networks = get_wifi_networks().await?;
                let event = AppEvent::Wifi(WifiEvent::NetworkListUpdated(networks));
                net_tx.send(event).await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
                Ok(())
            }
            .await;

            if res.is_err() {
                break;
            }
        }
    });

    while app_state.is_running {
        terminal.draw(|frame| render(frame, app_state))?;
        tokio::select! {
            Some(Ok(event)) = crossterm_events.next() => {
                if let Event::Key(key) = event {
                    app_state.handle(AppEvent::Key(key));
                }
            }

            Some(event) = rx.recv() => {
                app_state.handle(event);
            }
        }
    }
    Ok(())
}

async fn get_wifi_networks() -> Result<Vec<Network>> {
    let nm = NetworkManager::new().await?;

    let networks = nm
        .list_networks(None)
        .await
        .wrap_err("Failed to list networks")?;

    Ok(networks)
}

async fn get_wifi_connection() -> Result<Option<String>> {
    let nm = NetworkManager::new().await?;

    let connected_ssid = nm.current_ssid().await;

    Ok(connected_ssid)
}

fn render(frame: &mut Frame, app_state: &AppState) {
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
