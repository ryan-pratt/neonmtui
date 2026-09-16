use color_eyre::{Result, eyre::WrapErr};
use crossterm::event::{KeyCode, KeyEvent};
use nmrs::{Network, NetworkManager};

pub mod ui;

pub trait AppStateUpdater {
    fn update_app_state(self: Box<Self>, app_state: &mut AppState);
}

pub enum WifiEvent {
    NetworkListUpdated(Vec<Network>),
    ConnectionUpdated(Option<String>),
}

impl AppStateUpdater for WifiEvent {
    fn update_app_state(self: Box<Self>, app_state: &mut AppState) {
        match *self {
            Self::ConnectionUpdated(con) => {
                app_state.wifi.connected_ssid = con;
                app_state.status_text = String::from("WiFi connection updated");
            }

            Self::NetworkListUpdated(nets) => {
                app_state.wifi.networks = nets;
                app_state.status_text = String::from("WiFi available networks updated")
            }
        }
    }
}

impl AppStateUpdater for KeyEvent {
    fn update_app_state(self: Box<Self>, app_state: &mut AppState) {
        if self.is_press()
            && let KeyCode::Char('q') = self.code
        {
            app_state.is_running = false;
        }
    }
}

pub struct AppState {
    pub wifi: WifiState,
    pub status_text: String,
    pub is_running: bool,
}

impl AppState {
    pub fn handle(&mut self, event: Box<dyn AppStateUpdater>) {
        event.update_app_state(self);
    }
}

pub struct WifiState {
    pub networks: Vec<Network>,
    pub connected_ssid: Option<String>,
}

pub async fn get_wifi_networks() -> Result<Vec<Network>> {
    let nm = NetworkManager::new().await?;

    let networks = nm
        .list_networks(None)
        .await
        .wrap_err("Failed to list networks")?;

    Ok(networks)
}

pub async fn get_wifi_connection() -> Result<Option<String>> {
    let nm = NetworkManager::new().await?;

    let connected_ssid = nm.current_ssid().await;

    Ok(connected_ssid)
}
