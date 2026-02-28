use egui::ColorImage;

#[derive(Default)]
pub struct AppIcons {
    pub trash_icon: Option<egui::TextureHandle>,
}

pub enum Icons {
    TrashIcon,
}

impl Icons {
    pub fn get_icon(self, ctx: &egui::Context) -> egui::TextureHandle {
        match self {
            Icons::TrashIcon => {
                self.load_texture_from_bytes(ctx, include_bytes!("../assets/trash.png"))
            }
        }
    }

    fn load_texture_from_bytes(self, ctx: &egui::Context, bytes: &[u8]) -> egui::TextureHandle {
        let image = image::load_from_memory(bytes).expect("Failed to decode image");
        let image = image.to_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.as_flat_samples();

        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        return ctx.load_texture(self, color_image, egui::TextureOptions::default());
    }
}

impl Into<String> for Icons {
    fn into(self) -> String {
        match self {
            Icons::TrashIcon => "TrashIcon".to_string(),
        }
    }
}
