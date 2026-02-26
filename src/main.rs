// #![windows_subsystem = "windows"]
mod codec;
mod connection;
mod connection_manager_ui;
mod handlers;
mod word_list;

use std::env;
use std::path::Path;

use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopWindowTarget};
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{Menu, MenuId, MenuItemBuilder, SubmenuBuilder},
};

use crate::handlers::{
    UserEvent, handle_clipboard_event, handle_connection_manager_event, handle_file_picker_event,
};

pub const APP_ID: &str = "DTRAN.APP.Test.V2";

enum EventIds {
    FilePicker,
    Clipboard,
    Connections,
    Exit,
}

impl From<String> for EventIds {
    fn from(s: String) -> Self {
        match s.as_str() {
            "FilePicker" => EventIds::FilePicker,
            "Clipboard" => EventIds::Clipboard,
            "Connections" => EventIds::Connections,
            "Exit" => EventIds::Exit,
            _ => panic!("Invalid id"),
        }
    }
}

impl EventIds {
    fn to_str(&self) -> &str {
        match self {
            EventIds::FilePicker => "FilePicker",
            EventIds::Clipboard => "Clipboard",
            EventIds::Connections => "Connections",
            EventIds::Exit => "Exit",
        }
    }
}

fn main() {
    let execute_path = env::current_exe().unwrap();

    #[cfg(windows)]
    register_windows_appid("D TRAN", execute_path.parent().unwrap());

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let tray_menu = Menu::new();

    let file_picker_tab = MenuItemBuilder::new()
        .id(MenuId::new(EventIds::FilePicker.to_str()))
        .enabled(true)
        .text("Select File")
        .build();

    let clipboard_tab = MenuItemBuilder::new()
        .id(MenuId::new(EventIds::Clipboard.to_str()))
        .enabled(true)
        .text("Send Clipboard")
        .build();

    let send_tab = SubmenuBuilder::new()
        .enabled(true)
        .text("Transfer")
        .items(&[&clipboard_tab, &file_picker_tab])
        .build()
        .expect("Failed ot build SubMenu");

    let connections_tab = MenuItemBuilder::new()
        .id(MenuId::new(EventIds::Connections.to_str()))
        .enabled(true)
        .text("Connection Manager")
        .build();

    let exit = MenuItemBuilder::new()
        .id(MenuId::new(EventIds::Exit.to_str()))
        .enabled(true)
        .text("Exit")
        .build();

    tray_menu
        .insert_items(&[&send_tab, &connections_tab, &exit], 0)
        .unwrap();

    let icon = Icon::from_path(execute_path.parent().unwrap().join("d_tran.ico"), None).unwrap();
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("D Tran")
        .with_icon(icon)
        .build()
        .unwrap();

    event_loop.run(move |event, window_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            tao::event::Event::UserEvent(UserEvent::MenuEvent(event)) => {
                handle_menu_event(event, control_flow, window_target);
            }
            _ => {}
        }
    });
}

fn handle_menu_event(
    event: tray_icon::menu::MenuEvent,
    control_flow: &mut ControlFlow,
    window_target: &EventLoopWindowTarget<UserEvent>,
) {
    match EventIds::from(event.id.0) {
        EventIds::FilePicker => handle_file_picker_event(),
        EventIds::Clipboard => handle_clipboard_event(),
        EventIds::Connections => handle_connection_manager_event(window_target),
        EventIds::Exit => {
            println!("Exiting");
            *control_flow = ControlFlow::ExitWithCode(0);
        }
    }
}

#[cfg(windows)]
fn register_windows_appid(display_name: &str, executing_dir: &Path) {
    use windows::Win32::System::WinRT::{RO_INIT_SINGLETHREADED, RoInitialize};
    use winreg::RegKey;
    use winreg::enums::*;

    unsafe {
        let _ = RoInitialize(RO_INIT_SINGLETHREADED);
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!(r"Software\Classes\AppUserModelId\{}", APP_ID);

    if let Ok((key, _)) = hkcu.create_subkey(&path) {
        let _ = key.set_value("DisplayName", &display_name);
        let _ = key.set_value(
            "IconUri",
            &executing_dir
                .join("d_tran_notification.png")
                .to_str()
                .unwrap(),
        );
    }
}
