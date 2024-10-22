use imgui::*;
use imgui_rs_vulkan_renderer::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::window::Window;

use crate::application::core::error::ErrorCode;

use super::types::VulkanContext;

pub struct GuiWrapper {
    pub context: Context,
    pub platform: WinitPlatform,
    pub hidpi_factor: f64,
}

impl VulkanContext<'_> {
    pub fn init_gui(&mut self, window: &Window) -> Result<(), ErrorCode> {
        let mut context = Context::create();
        context.set_ini_filename(None);

        let mut platform = WinitPlatform::new(&mut context);
        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13. * hidpi_factor) as f32;

        context.fonts().add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
        ]);
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        platform.attach_window(context.io_mut(), &window, HiDpiMode::Rounded);
        todo!()
    }
}