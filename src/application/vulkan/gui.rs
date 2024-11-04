use std::{cmp::Ordering, sync::Arc, time::Duration};

use ash::vk::{
    AttachmentLoadOp, AttachmentStoreOp, DescriptorPool, DescriptorPoolCreateFlags,
    DescriptorPoolCreateInfo, DescriptorPoolSize, DescriptorType, ImageLayout, Offset2D, Rect2D,
    RenderingAttachmentInfo, RenderingInfo,
};
use imgui::*;
use imgui_rs_vulkan_renderer::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use log::error;
use winit::{
    event::{DeviceEvent, ElementState, MouseScrollDelta, TouchPhase, WindowEvent},
    window::Window,
};

use crate::application::{
    core::error::ErrorCode,
    scene::{bvh::BvhType, Scene},
    window::key_map::winit_character_to_imgui_key,
};

use super::{setup::frame_data::FRAME_OVERLAP, types::VulkanContext};

#[derive(Default)]
pub struct GuiWrapper {
    pub descriptor_pool: DescriptorPool,
    pub context: Option<Context>,
    pub platform: Option<WinitPlatform>,
    pub renderer: Option<Renderer>,
}

impl VulkanContext<'_> {
    fn init_gui_descriptor_pool(&self) -> Result<DescriptorPool, ErrorCode> {
        // Create descriptor pool for IMGUI
        let descriptor_pool_sizes = [
            DescriptorPoolSize::default()
                .ty(DescriptorType::SAMPLER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::SAMPLED_IMAGE)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::UNIFORM_TEXEL_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_TEXEL_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_BUFFER_DYNAMIC)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::INPUT_ATTACHMENT)
                .descriptor_count(1000),
        ];

        let descriptor_pool_create_info = DescriptorPoolCreateInfo::default()
            .flags(DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
            .max_sets(1000)
            .pool_sizes(&descriptor_pool_sizes);

        let device = self.get_device()?;
        let allocation_callback = self.get_allocation_callback()?;
        let descriptor_pool = match unsafe {
            device.create_descriptor_pool(&descriptor_pool_create_info, allocation_callback)
        } {
            Ok(pool) => pool,
            Err(err) => {
                error!("Failed to create a descriptor pool for the gui: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        Ok(descriptor_pool)
    }

    pub fn clean_gui(&mut self) -> Result<(), ErrorCode> {
        let device = self.get_device()?;
        let allocation_callback = self.get_allocation_callback()?;
        let gui = self.get_gui()?;
        unsafe {
            device.destroy_descriptor_pool(gui.descriptor_pool, allocation_callback);
        }
        self.gui.context = None;
        self.gui.platform = None;
        self.gui.renderer = None;
        Ok(())
    }

    pub fn get_gui(&self) -> Result<&GuiWrapper, ErrorCode> {
        Ok(&self.gui)
    }

    pub fn init_gui(&mut self, window: &Window) -> Result<(), ErrorCode> {
        let descriptor_pool = self.init_gui_descriptor_pool()?;

        let mut context = Context::create();
        context.set_ini_filename(None);

        let mut platform = WinitPlatform::new(&mut context);
        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13. * hidpi_factor) as f32;

        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        }]);
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        platform.attach_window(context.io_mut(), window, HiDpiMode::Rounded);

        platform.attach_window(
            context.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Rounded,
        );

        let image_format = self.get_swapchain_handler()?.surface_format.format;
        let dynamic_rendering = DynamicRendering {
            color_attachment_format: image_format,
            depth_attachment_format: None,
        };

        let allocator = self.get_allocator()?;
        let device = self.get_device()?;
        let graphics_queue = self.get_queues()?.graphics_queue.unwrap();

        let immediate = &self.immediate_submit;
        let renderer = Renderer::with_vk_mem_allocator(
            Arc::clone(&allocator.allocator),
            device.clone(),
            graphics_queue,
            immediate.command_pool,
            dynamic_rendering,
            &mut context,
            Some(imgui_rs_vulkan_renderer::Options {
                in_flight_frames: 1,
                ..Default::default()
            }),
        )
        .unwrap();

        let gui_wrapper = GuiWrapper {
            descriptor_pool,
            context: Some(context),
            platform: Some(platform),
            renderer: Some(renderer),
        };

        self.gui = gui_wrapper;

        Ok(())
    }

    fn prepare_gui_draw_cmd(&self, swapchain_next_index: usize) -> Result<(), ErrorCode> {
        let swapchain = self.get_swapchain_handler()?;
        let image_view = self.get_swapchain_handler()?.image_views[swapchain_next_index];
        let rendering_attachement_info = [RenderingAttachmentInfo::default()
            .image_view(image_view)
            .image_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(AttachmentLoadOp::LOAD)
            .store_op(AttachmentStoreOp::STORE)];

        let render_area = Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: swapchain.extent,
        };

        let rendering_info = RenderingInfo::default()
            .render_area(render_area)
            .color_attachments(&rendering_attachement_info)
            .layer_count(1);

        let device = self.get_device()?;
        let main_command_buffer = self.get_current_frame()?.main_command_buffer;
        unsafe { device.cmd_begin_rendering(main_command_buffer, &rendering_info) };
        Ok(())
    }

    pub fn draw_gui(
        &mut self,
        window: &Window,
        swapchain_next_index: usize,
        scene: &mut Scene,
    ) -> Result<(), ErrorCode> {
        self.prepare_gui_draw_cmd(swapchain_next_index)?;

        // Generate UI
        if let Err(err) = self
            .gui
            .platform
            .as_mut()
            .unwrap()
            .prepare_frame(self.gui.context.as_mut().unwrap().io_mut(), window)
        {
            error!("Failed to prepare the gui frame: {:?}", err);
            return Err(ErrorCode::Unknown);
        }
        let ui = self.gui.context.as_mut().unwrap().frame();

        // TODO: Create the GUI window
        ui.window("Raytracing Parameters")
            .size([300.0, 110.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.checkbox("Toogl wireframe mode", &mut scene.is_wireframe_on);
                ui.new_line();
                ui.text("BVH type");
                ui.radio_button("None", &mut scene.bvh_type, BvhType::None);
                ui.same_line();
                ui.radio_button(
                    "Default Top Down",
                    &mut scene.bvh_type,
                    BvhType::DefaultTopDown,
                );
            });

        self.gui
            .platform
            .as_mut()
            .unwrap()
            .prepare_render(ui, window);

        let draw_data = self.gui.context.as_mut().unwrap().render();

        let main_command_buffer = self.frames[self.frame_index % FRAME_OVERLAP].main_command_buffer;
        if let Err(err) = self
            .gui
            .renderer
            .as_mut()
            .unwrap()
            .cmd_draw(main_command_buffer, draw_data)
        {
            error!(
                "Failed to send a draw command from the gui renderer: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        let device = self.get_device()?;
        unsafe { device.cmd_end_rendering(main_command_buffer) };

        Ok(())
    }

    pub fn on_new_event_gui(&mut self, delta_time: Duration) -> Result<(), ErrorCode> {
        let io = self.gui.context.as_mut().unwrap().io_mut();
        io.update_delta_time(delta_time);
        Ok(())
    }

    pub fn on_device_event_gui(&mut self, event: &DeviceEvent) -> Result<(), ErrorCode> {
        let io = self.gui.context.as_mut().unwrap().io_mut();
        // Track key release events outside our window. If we don't do this,
        // we might never see the release event if some other window gets focus.
        if let DeviceEvent::Key(raw_key_event) = event {
            if let winit::keyboard::PhysicalKey::Code(key_code) = raw_key_event.physical_key {
                io.keys_down[key_code as usize] = false;
            }
        }
        Ok(())
    }

    pub fn on_window_event_gui(
        &mut self,
        window: &Window,
        event: &WindowEvent,
    ) -> Result<(), ErrorCode> {
        let io = self.gui.context.as_mut().unwrap().io_mut();
        if let WindowEvent::ModifiersChanged(modifiers) = event {
            io.key_shift = modifiers.state().shift_key();
            io.key_ctrl = modifiers.state().control_key();
            io.key_alt = modifiers.state().alt_key();
            io.key_super = modifiers.state().super_key();
        }

        let platform = self.gui.platform.as_mut().unwrap();
        match event {
            WindowEvent::Resized(physical_size) => {
                let logical_size = physical_size.to_logical(window.scale_factor());
                let logical_size = platform.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let hidpi_factor = scale_factor.round();

                // Mouse position needs to be changed while we still have both the old and the new
                // values
                if io.mouse_pos[0].is_finite() && io.mouse_pos[1].is_finite() {
                    io.mouse_pos = [
                        io.mouse_pos[0] * (hidpi_factor / platform.hidpi_factor()) as f32,
                        io.mouse_pos[1] * (hidpi_factor / platform.hidpi_factor()) as f32,
                    ];
                }

                io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
                // Window size might change too if we are using DPI rounding
                let logical_size = window.inner_size().to_logical(*scale_factor);
                let logical_size = platform.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let state = event.state;
                if let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key {
                    let pressed = state == ElementState::Pressed;
                    io.keys_down[key_code as usize] = pressed;
                    match key_code {
                        // This is a bit redundant here, but we'll leave it in. The OS occasionally
                        // fails to send modifiers keys, but it doesn't seem to send false-positives,
                        // so double checking isn't terrible in case some system *doesn't* send
                        // device events sometimes
                        winit::keyboard::KeyCode::ControlLeft
                        | winit::keyboard::KeyCode::ControlRight => io.key_ctrl = pressed,
                        winit::keyboard::KeyCode::ShiftLeft
                        | winit::keyboard::KeyCode::ShiftRight => io.key_shift = pressed,
                        winit::keyboard::KeyCode::AltLeft | winit::keyboard::KeyCode::AltRight => {
                            io.key_alt = pressed
                        }
                        winit::keyboard::KeyCode::SuperLeft
                        | winit::keyboard::KeyCode::SuperRight => io.key_super = pressed,
                        _ => (),
                    }
                }
                if let winit::keyboard::Key::Character(ch) = &event.logical_key {
                    // Exclude the backspace key ('\u{7f}'). Otherwise we will insert this char and then
                    // delete it
                    if let Some(ch) = winit_character_to_imgui_key(ch.clone()) {
                        let ch = (ch as u8) as char;
                        if ch != '\u{7f}' {
                            io.add_input_character(ch)
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical(window.scale_factor());
                let position = platform.scale_pos_from_winit(window, position);
                io.mouse_pos = [position.x as f32, position.y as f32];
            }
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => match delta {
                MouseScrollDelta::LineDelta(h, v) => {
                    io.mouse_wheel_h = *h;
                    io.mouse_wheel = *v;
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    let pos = pos.to_logical::<f64>(platform.hidpi_factor());
                    match pos.x.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel_h += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel_h -= 1.0,
                        _ => (),
                    }
                    match pos.y.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel -= 1.0,
                        _ => (),
                    }
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = *state == ElementState::Pressed;
                match button {
                    winit::event::MouseButton::Left => {
                        io.add_mouse_button_event(MouseButton::Left, pressed)
                    }
                    winit::event::MouseButton::Right => {
                        io.add_mouse_button_event(MouseButton::Right, pressed)
                    }
                    winit::event::MouseButton::Middle => {
                        io.add_mouse_button_event(MouseButton::Middle, pressed)
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        Ok(())
    }
}
