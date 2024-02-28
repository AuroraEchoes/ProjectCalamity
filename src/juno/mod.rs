pub mod directions;
pub mod grid;
pub mod renderer;

use cgmath::Vector2;
use renderer::{renderer::Renderer, testing::TextureAtlasHandle};
use winit::{
    dpi::PhysicalSize,
    event::{
        DeviceEvent, ElementState, Event, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent,
    },
    event_loop::{EventLoop, EventLoopBuilder},
    keyboard::{KeyCode, PhysicalKey},
    platform::{run_on_demand::EventLoopExtRunOnDemand, wayland::EventLoopBuilderExtWayland},
};

use crate::interaction::ButtonInput;

pub struct JunoApp {
    renderer: Renderer,
    event_loop: EventLoop<()>,
    window_size: PhysicalSize<u32>,
    input_state: InputState,
}

impl JunoApp {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
            .unwrap();

        let window_size = PhysicalSize { width, height };
        let renderer = Renderer::new(&event_loop, window_size);
        Self {
            renderer,
            event_loop,
            window_size,
            input_state: InputState::new(),
        }
    }

    pub fn update(&mut self) -> bool {
        // Should continue
        let mut should_continue = true;
        self.input_state.clear();
        self.event_loop
            .run_on_demand(|event, event_loop| {
                match event {
                    Event::WindowEvent { window_id, event } => {
                        if window_id == self.renderer.interface().window().id() {
                            match event {
                                WindowEvent::CloseRequested => {
                                    event_loop.exit();
                                    should_continue = false
                                }
                                WindowEvent::Resized(size) => {
                                    self.renderer.interface().resize(size);
                                    self.window_size = size
                                }
                                WindowEvent::RedrawRequested => {
                                    match self.renderer.render() {
                                        Ok(_) => {}
                                        // Reconfigure the surface if lost
                                        Err(wgpu::SurfaceError::Lost) => self
                                            .renderer
                                            .interface()
                                            .resize(winit::dpi::PhysicalSize {
                                                width: self.window_size.width,
                                                height: self.window_size.height,
                                            }),
                                        // The system is out of memory, we should probably quit
                                        Err(wgpu::SurfaceError::OutOfMemory) => {
                                            event_loop.exit();
                                        }
                                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                                        Err(e) => eprintln!("{:?}", e),
                                    }
                                }
                                WindowEvent::KeyboardInput {
                                    device_id: _,
                                    event: key_event,
                                    is_synthetic: _,
                                } => {
                                    if key_event.state == ElementState::Pressed {
                                        match key_event.physical_key {
                                            PhysicalKey::Code(key_code) => {
                                                self.input_state.add_key(key_code)
                                            }
                                            PhysicalKey::Unidentified(_) => {}
                                        }
                                    }
                                }
                                WindowEvent::MouseInput {
                                    device_id: _,
                                    state,
                                    button,
                                } => {
                                    if state == ElementState::Pressed {
                                        self.input_state.add_button(button)
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Event::DeviceEvent {
                        device_id: _,
                        event: device_event,
                    } => match device_event {
                        DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                            let delta_clip = Vector2::new(
                                dx as f32 / self.window_size.width as f32,
                                dy as f32 / self.window_size.height as f32,
                            );
                            self.input_state.modify_mouse_delta(delta_clip)
                        }
                        DeviceEvent::MouseWheel { delta } => {
                            let wheel_delta = match delta {
                                MouseScrollDelta::LineDelta(dx, _) => dx,
                                MouseScrollDelta::PixelDelta(physical_delta) => {
                                    physical_delta.x as f32
                                }
                            };
                            self.input_state.modify_wheel_delta(wheel_delta)
                        }
                        _ => {}
                    },
                    _ => event_loop.exit(),
                }
            })
            .unwrap();
        should_continue
    }

    pub fn load_texture_atlas(
        &mut self,
        path: &str,
        pixel_size: Vector2<u32>,
    ) -> Option<TextureAtlasHandle> {
        self.renderer.load_atlas(path, pixel_size)
    }

    pub fn render(&mut self) {
        let _ = self.renderer.render();
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn input_state(&self) -> &InputState {
        &self.input_state
    }
}

impl Default for JunoApp {
    fn default() -> Self {
        Self::new(1280, 720)
    }
}

#[derive(Debug)]
pub struct InputState {
    button_presses: Vec<ButtonInput>,
    mouse_delta: Vector2<f32>,
    wheel_delta: f32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            button_presses: Vec::new(),
            mouse_delta: Vector2::new(0., 0.),
            wheel_delta: 0.,
        }
    }
    pub fn button_presses(&self) -> &Vec<ButtonInput> {
        &self.button_presses
    }
    pub fn mouse_delta(&self) -> Vector2<f32> {
        self.mouse_delta
    }
    pub fn wheel_delta(&self) -> f32 {
        self.wheel_delta
    }
    pub fn add_key(&mut self, key: KeyCode) {
        self.button_presses.push(ButtonInput::Key(key));
    }
    pub fn add_button(&mut self, button: MouseButton) {
        self.button_presses.push(ButtonInput::Mouse(button));
    }
    pub fn modify_mouse_delta(&mut self, distance: Vector2<f32>) {
        self.mouse_delta += distance;
    }
    pub fn modify_wheel_delta(&mut self, distance: f32) {
        self.wheel_delta += distance;
    }
    pub fn clear(&mut self) {
        self.button_presses.clear();
        self.mouse_delta = Vector2::new(0., 0.);
        self.wheel_delta = 0.;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}
