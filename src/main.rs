extern crate servo;
extern crate glutin;

use glutin::GlContext;
use servo::BrowserId;
use servo::compositing::windowing::{EmbedderMethods, WindowMethods, EmbedderCoordinates};
use servo::compositing::windowing::{AnimationState, WindowEvent};
use servo::embedder_traits::EventLoopWaker;
use servo::euclid::{TypedPoint2D, TypedScale, TypedSize2D, TypedVector2D, };
use servo::gl;
use servo::script_traits::{TouchEventType};
use servo::servo_config::opts;
use servo::embedder_traits::resources::{self, Resource};
use servo::servo_url::ServoUrl;
use servo::webrender_api::{DeviceIntRect, FramebufferIntSize};
use std::cell::{Cell};
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

pub struct GlutinEventLoopWaker {
    proxy: Arc<glutin::EventsLoopProxy>,
}

impl EventLoopWaker for GlutinEventLoopWaker {
    fn clone(&self) -> Box<dyn EventLoopWaker + Send> {
        Box::new(GlutinEventLoopWaker {
            proxy: self.proxy.clone(),
        })
    }
    fn wake(&self) {
        self.proxy.wakeup().expect("wakeup eventloop failed");
    }
}

impl EmbedderMethods for GlutinEventLoopWaker {
    fn create_event_loop_waker(&mut self) -> Box<dyn EventLoopWaker> {
        Box::new(GlutinEventLoopWaker { 
            proxy: self.proxy.clone() 
        })
    }
}

struct Window {
    glutin_window: glutin::GlWindow,
    animation_state: Cell<AnimationState>,
    gl: Rc<dyn gl::Gl>,
}

impl WindowMethods for Window {
    fn present(&self) {
        self.glutin_window.swap_buffers().unwrap();
    }

    fn prepare_for_composite(&self) {

    }

    fn gl(&self) -> Rc<dyn gl::Gl> {
        self.gl.clone()
    }

    fn get_coordinates(&self) -> EmbedderCoordinates {
        let dpr = TypedScale::new(self.glutin_window.hidpi_factor());
        let (width, height) = self
            .glutin_window
            .get_outer_size()
            .unwrap();
        let ( x, y ) = self
            .glutin_window
            .get_position()
            .unwrap();
        let win_size = (TypedSize2D::new(width as f32, height as f32) * dpr).to_i32();
        let win_origin = (TypedPoint2D::new(x as f32, y as f32) * dpr).to_i32();
        let ( width, height ) = self
            .glutin_window
            .get_inner_size()
            .unwrap();
        let screen = TypedSize2D::new(width as i32, height as i32);
        let inner_size = (TypedSize2D::new(width as f32, height as f32) * dpr).to_i32();
        let viewport = DeviceIntRect::new(TypedPoint2D::zero(), inner_size);
        let framebuffer = FramebufferIntSize::from_untyped(&viewport.size.to_untyped());
        EmbedderCoordinates {
            viewport,
            framebuffer,
            window: (win_size, win_origin),
            screen: screen,
            screen_avail: screen,
            hidpi_factor: dpr,
        }
    }

    fn set_animation_state(&self, state: AnimationState) {
        self.animation_state.set(state);
    }
}

fn main() {
    println!("Servo version: {}", servo::config::servo_version());

    let mut event_loop = glutin::EventsLoop::new();

    let builder = glutin::WindowBuilder::new().with_dimensions(800, 600);
    let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));
    let context = glutin::ContextBuilder::new()
        .with_gl(gl_version)
        .with_vsync(true);
    let window = glutin::GlWindow::new(builder, context, &event_loop).unwrap();
  
    window.show();

    let gl = unsafe {
        window
            .context()
            .make_current()
            .expect("Couldn't make window current");
        gl::GlFns::load_with(|s| window.context().get_proc_address(s) as *const _)
    };

    let event_loop_waker = Box::new(GlutinEventLoopWaker {
        proxy: Arc::new(event_loop.create_proxy()),
    });

    resources::set(Box::new(ResourceReader));

    opts::set_options(opts::default_opts());

    let window = Rc::new(Window {
        glutin_window: window,
        animation_state: Cell::new(AnimationState::Idle),
        gl: gl,
    });

    let mut servo = servo::Servo::new(event_loop_waker, window.clone());

    let url = ServoUrl::parse("https://servo.org/").unwrap();
    let browser_id = BrowserId::new();
    servo.handle_events(vec![WindowEvent::NewBrowser(url, browser_id)]);
    servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

    let mut pointer = (0.0, 0.0);

    event_loop.run_forever(|event| {
        match event {
            glutin::Event::Awakened => {
                servo.handle_events(vec![]);
            }

            glutin::Event::WindowEvent {
                event:
                    glutin::WindowEvent::CursorMoved {
                        position: (x, y), ..
                    },
                ..
            } => {
                pointer = (x, y);
                let event =
                    WindowEvent::MouseWindowMoveEventClass(TypedPoint2D::new(x as f32, y as f32));
                servo.handle_events(vec![event]);
            }

            glutin::Event::WindowEvent {
                event:
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                state: glutin::ElementState::Pressed,
                                virtual_keycode: Some(glutin::VirtualKeyCode::R),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                let event = WindowEvent::Reload(browser_id);
                servo.handle_events(vec![event]);
            }

            glutin::Event::WindowEvent {
                event: glutin::WindowEvent::MouseWheel { delta, phase, .. },
                ..
            } => {
                let pointer = TypedPoint2D::new(pointer.0 as i32, pointer.1 as i32);
                let (dx, dy) = match delta {
                    glutin::MouseScrollDelta::LineDelta(dx, dy) => {
                        (dx, dy * 38.0 /*line height*/)
                    }
                    glutin::MouseScrollDelta::PixelDelta(dx, dy) => (dx, dy),
                };
                let scroll_location =
                    servo::webrender_api::ScrollLocation::Delta(TypedVector2D::new(dx, dy));
                let phase = match phase {
                    glutin::TouchPhase::Started => TouchEventType::Down,
                    glutin::TouchPhase::Moved => TouchEventType::Move,
                    glutin::TouchPhase::Ended => TouchEventType::Up,
                    glutin::TouchPhase::Cancelled => TouchEventType::Up,
                };
                let event = WindowEvent::Scroll(scroll_location, pointer, phase);
                servo.handle_events(vec![event]);
            }

            glutin::Event::WindowEvent {
                event: glutin::WindowEvent::Resized(width, height),
                ..
            } => {
                let event = WindowEvent::Resize;
                servo.handle_events(vec![event]);
                window.glutin_window.resize(width, height);
            }

            _ => {}
        }

        glutin::ControlFlow::Continue
    });
}

struct ResourceReader;

impl resources::ResourceReaderMethods for ResourceReader {
    fn read(&self, file: Resource) -> Vec<u8> {
        let file = filename(file);
        let mut path = resources_dir_path().expect("Can't find resources directory");
        path.push(file);
        fs::read(path).expect("Can't read file")
    }

    fn sandbox_access_files_dirs(&self) -> Vec<PathBuf> {
        vec![resources_dir_path().expect("Can't find resources directory")]
    }

    fn sandbox_access_files(&self) -> Vec<PathBuf> {
        vec![]
    }
}

fn resources_dir_path() -> io::Result<PathBuf> {
    let path = env::current_dir().unwrap().join("resources");
    Ok(path)
}

fn filename(file: Resource) -> &'static str {
    match file {
        Resource::Preferences => "prefs.json",
        Resource::BluetoothBlocklist => "gatt_blocklist.txt",
        Resource::DomainList => "public_domains.txt",
        Resource::HstsPreloadList => "hsts_preload.json",
        Resource::SSLCertificates => "certs",
        Resource::BadCertHTML => "badcert.html",
        Resource::NetErrorHTML => "neterror.html",
        Resource::UserAgentCSS => "user-agent.css",
        Resource::ServoCSS => "servo.css",
        Resource::PresentationalHintsCSS => "presentational-hints.css",
        Resource::QuirksModeCSS => "quirks-mode.css",
        Resource::RippyPNG => "rippy.png",
    }
}