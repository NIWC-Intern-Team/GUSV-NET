use std::path::PathBuf;

use ratatui::widgets::ListState;

use crate::{sniffer::*, NodeTable};

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Edit,
    Push,
    Delete,
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    InterfaceView,
    NodeView(Mode),
    Help,
    Home,
    Main,
    Exiting,
}

#[derive(PartialEq)]
pub enum PingStatus {
    Halt,
    Running(u32),
}

#[derive(Debug)]
pub struct IpConnection {
    pub _ip: String,
    pub _conn_status: String,
}

impl IpConnection {
    fn new(ip: &str, conn_status: &str) -> Self {
        Self {
            _ip: ip.to_string(),
            _conn_status: conn_status.to_string(),
        }
    }
}

pub struct IpList {
    pub _items: Vec<IpConnection>,
    pub _state: ListState,
}

#[derive(PartialEq)]
pub enum IpInputStatus {
    Normal,
    Error,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub metrics: Vec<String>,
    pub analyzer_code: String,
    pub options_idx: u32,
    pub options: [String; 3],
    pub interface: String,
    pub if_options_idx: u32,
    pub interfaces: Vec<String>,
    pub ping_status: PingStatus,
    pub node_table: NodeTable,
    pub ip_input_status: IpInputStatus,
    pub ip_input: String,
    pub filepath: PathBuf,

    // Help
    pub help_scroll: u16,
}

impl FromIterator<(&'static str, &'static str)> for IpList {
    fn from_iter<T: IntoIterator<Item = (&'static str, &'static str)>>(iter: T) -> Self {
        let _items = iter
            .into_iter()
            .map(|(ip, conn_status)| IpConnection::new(ip, conn_status))
            .collect();
        let _state = ListState::default();
        Self { _items, _state }
    }
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Home,
            metrics: (0..6).map(|_| String::new()).collect(),
            analyzer_code: scapy_analyzer_import(),
            options_idx: 0,
            if_options_idx: 0,
            options: [
                "Connection test".into(),
                "Network monitor".into(),
                "Help".into(),
            ],
            node_table: NodeTable::default(),
            interfaces: list_interfaces(),
            interface: String::new(),
            ping_status: PingStatus::Halt,
            ip_input: String::from(""),
            ip_input_status: IpInputStatus::Normal,
            filepath: PathBuf::new(),
            help_scroll: 0,
        }
    }
}
