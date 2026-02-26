use std::net::UdpSocket;

use arboard::Clipboard;
use egui::Pos2;
use local_ip_address::local_ip;
use notify_rust::Notification;
use rand::RngExt;
use rfd::FileDialog;
use stunclient::StunClient;
use tao::event_loop::EventLoopWindowTarget;

use crate::{
    APP_ID,
    codec::{self, IpAddrWrapper},
    connection::{UdpConnection, init_and_send},
    connection_manager_ui::{AppState, show_connection_manager},
};

pub enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

const POPUP_WIDTH: f32 = 200.0;
const POPUP_HEIGHT: f32 = 300.0;

//Move this out to just send to selected active connections
pub fn handle_file_picker_event() {
    if let Some(file_path) = FileDialog::new().pick_file() {
        let pth_str = file_path.as_path();

        let udp = UdpSocket::bind("0.0.0.0:0").expect("udp failure");
        let local_ip = local_ip().unwrap();
        let public_ip = StunClient::with_google_stun_server() //Might want to remove at some point. This pulls in TONS of packages
            .query_external_address(&udp)
            .expect("Failed to get Public IP"); //TCP hole punching (self implement or use quinn QUIC libray)

        let code = codec::encode(
            IpAddrWrapper::new(public_ip.ip()),
            IpAddrWrapper::new(local_ip),
            public_ip.port(),
            rand::rng().random(),
        );

        Notification::new()
            .app_id(APP_ID)
            .appname(APP_ID)
            .summary("File Selected")
            .body(&code)
            .show()
            .expect("Notification failed");

        let choice = rfd::MessageDialog::new()
            .set_title("Ready to Send?")
            .set_description(format!(
                "Send file {}?",
                pth_str.file_name().unwrap().display()
            ))
            .set_buttons(rfd::MessageButtons::OkCancel)
            .show();

        match choice {
            rfd::MessageDialogResult::Ok => init_and_send(UdpConnection { socket: udp }),
            _ => println!("NOOOO"),
        }
    }
}

pub fn handle_clipboard_event() {
    let mut clipboard = Clipboard::new().unwrap();

    if let Ok(text) = clipboard.get_text() {
        println!("Text Copied: {}", text);
        return;
    }

    if let Ok(image) = clipboard.get_image() {
        println!(
            "image data - Width: {} | Height: {} | Size: {}",
            image.width,
            image.height,
            image.bytes.len()
        );
        return;
    }
}

pub fn handle_connection_manager_event(window_target: &EventLoopWindowTarget<UserEvent>) {
    show_connection_manager(
        get_window_position(window_target),
        [POPUP_WIDTH, POPUP_HEIGHT],
        AppState {
            active_connections: vec![
                (false, format!("test.penguin")),
                (true, format!("monkey.apple")),
            ],
            ..Default::default()
        },
    )
}

fn get_window_position(window_target: &EventLoopWindowTarget<UserEvent>) -> impl Into<Pos2> {
    const POPUP_PADDING: f32 = 10.0;
    // Save to local config and just load into memory at app start (if not there then get)

    let (screen_w, screen_h) = window_target
        .available_monitors()
        .next()
        .map(|m| {
            let size = m.size();
            (size.width as f32, size.height as f32)
        })
        .unwrap_or((1920.0, 1080.0));

    let x = screen_w - POPUP_WIDTH - POPUP_PADDING;
    let y = screen_h - POPUP_HEIGHT - POPUP_PADDING;

    return [x, y];
}
