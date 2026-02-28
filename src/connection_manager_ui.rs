use egui::{Button, Pos2, Vec2, load::SizedTexture};

use crate::{
    APP_ID,
    icons::{AppIcons, Icons},
};

#[derive(Default)]
pub struct AppState {
    pub active_connections: Vec<(bool, String)>,
    pub had_focus: bool,
    pub new_connection: String,
    pub icons: AppIcons,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let focused = ctx.input(|i| {
            let focused = i.viewport().focused;
            if Some(true) == focused {
                self.had_focus = true;
            }
            return focused;
        });

        let trash_icon = self
            .icons
            .trash_icon
            .get_or_insert_with(|| Icons::TrashIcon.get_icon(ctx));

        if focused == Some(false) && self.had_focus {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            for connection in &mut self.active_connections {
                ui.horizontal(|row| {
                    row.checkbox(&mut connection.0, &connection.1);
                    if row
                        .add(Button::image(SizedTexture::from_handle(trash_icon)).frame(false))
                        .clicked()
                    {
                        println!("Delete Connection");
                    }
                });
            }

            ui.add_space(10.0);
            ui.label("Add New Connection");
            let conn_input = ui.text_edit_singleline(&mut self.new_connection);
            if conn_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                println!("Enter Submit: {}", self.new_connection);
            }
        });
    }
}

pub fn show_connection_manager(
    window_position: impl Into<Pos2>,
    window_size: impl Into<Vec2>,
    state: AppState,
) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()
            .with_decorations(false)
            .with_has_shadow(true)
            .with_resizable(false)
            .with_inner_size(window_size)
            .with_position(window_position)
            .with_taskbar(false)
            .with_visible(true),
        ..Default::default()
    };

    eframe::run_native(APP_ID, options, Box::new(|_| Ok(Box::new(state)))).unwrap();
}
