use color_eyre::Result;
use crossterm::event::{Event, EventStream};
use futures_util::StreamExt;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::Duration;
use tokio::sync::mpsc;

use neonmtui::{
    AppState, AppStateUpdater, WifiEvent, WifiState, get_wifi_connection, get_wifi_networks,
    ui::render,
};

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
    let (tx, mut rx) = mpsc::channel::<Box<dyn AppStateUpdater + Send + Sync>>(32);
    let mut crossterm_events = EventStream::new();

    let con_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            let res: Result<()> = async {
                let connected_ssid = get_wifi_connection().await?;
                let event = Box::new(WifiEvent::ConnectionUpdated(connected_ssid));
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
                let event = Box::new(WifiEvent::NetworkListUpdated(networks));
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
                    app_state.handle(Box::new(key));
                }
            }

            Some(event) = rx.recv() => {
                app_state.handle(event);
            }
        }
    }
    Ok(())
}
