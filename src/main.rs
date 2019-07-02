// -------------------------------
// Default

// fn main() {
//     println!("Hello, world!");
// }

// ===============================
// https://github.com/paulrouget/servo-embedding-example

// -------------------------------
// Minimal code:

// extern crate servo;

// fn main() {
//     println!("Servo version: {}", servo::config::servo_version());
// }

// -------------------------------
// Glutin's event loop

// extern crate servo;
// extern crate glutin;

// fn main() {
//     println!("Servo version: {}", servo::config::servo_version());

//     let mut event_loop = glutin::EventsLoop::new();

//     let builder = glutin::WindowBuilder::new().with_dimensions(800, 600);
//     let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));
//     let context = glutin::ContextBuilder::new().with_gl(gl_version).with_vsync(true);
//     let window = glutin::GlWindow::new(builder, context, &event_loop).unwrap();
  
//     window.show();

//     event_loop.run_forever(|_event| {
//         glutin::ControlFlow::Continue
//     });
// }

// -------------------------------
// EventLoopWaker: mechanism to wake up the event loop (EDIT)

// # 参考
// - https://github.com/servo/servo/blob/973a3448a459464b79ea0ef5fb46141176cc7643/ports/glutin/headed_window.rs

//#[macro_use] extern crate log;

// // use glutin::GlContext;
// use glutin::{WindowedContext, NotCurrent, PossiblyCurrent};

// use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
// // 変更
// // use servo::compositing::compositor_thread::EventLoopWaker; // private
// use servo::compositing::windowing::{AnimationState, MouseWindowEvent, WindowEvent};
// use servo::compositing::windowing::{EmbedderMethods, WindowMethods, EmbedderCoordinates};
// use servo::embedder_traits::EventLoopWaker;
// use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D, TypedScale};
// use servo::gl;
// use servo::ipc_channel::ipc;
// // added
// // use servo::msg::constellation_msg::{Key, KeyModifiers};
// // use servo::net_traits::net_error_list::NetError;
// use servo::script_traits::LoadData;
// use servo::servo_geometry::DeviceIndependentPixel;
// use servo::servo_url::ServoUrl;
// use servo::style_traits::DevicePixel;
// // use servo::style_traits::cursor::Cursor;
// use servo::embedder_traits::Cursor;
// use servo::servo_config::opts;
// use servo::webrender_api::{DeviceIntRect, FramebufferIntSize};
// // 存在しない
// // use servo::servo_config::resource_files::set_resources_path;
// use std::cell::{Cell, RefCell};
// use std::env;
// use std::rc::Rc;
// use std::sync::Arc;



// pub struct GlutinEventLoopWaker {
//     pub proxy: Arc<glutin::EventsLoopProxy>
// }

// impl EventLoopWaker for GlutinEventLoopWaker {
//     // Use by servo to share the "event loop waker" across threads
//     fn clone(&self) -> Box<EventLoopWaker + Send> {
//         Box::new(GlutinEventLoopWaker { 
//             proxy: self.proxy.clone() 
//         })
//     }
//     // Called by servo when the main thread needs to wake up
//     fn wake(&self) {
//         self.proxy.wakeup().expect("wakeup eventloop failed");
//     }
// }

// impl EmbedderMethods for GlutinEventLoopWaker {
//     fn create_event_loop_waker(&mut self) -> Box<EventLoopWaker> {
//         Box::new(GlutinEventLoopWaker { 
//             proxy: self.proxy.clone() 
//         })
//     }
// }


// struct Window {
//     gl_context: RefCell<GlContext>,
//     screen_size: TypedSize2D<u32, DeviceIndependentPixel>,
//     inner_size: Cell<TypedSize2D<u32, DeviceIndependentPixel>>,
//     mouse_down_button: Cell<Option<glutin::MouseButton>>,
//     mouse_down_point: Cell<TypedPoint2D<i32, DevicePixel>>,
//     primary_monitor: glutin::MonitorId,
//     event_queue: RefCell<Vec<WindowEvent>>,
//     mouse_pos: Cell<TypedPoint2D<i32, DevicePixel>>,
//     last_pressed: Cell<Option<KeyboardEvent>>,
//     animation_state: Cell<AnimationState>,
//     fullscreen: Cell<bool>,
//     gl: Rc<gl::Gl>,
// }

// fn main() {
//     // added
//     println!("Servo version: {}", servo::config::servo_version());
 
//     // […]
//     let mut event_loop = glutin::EventsLoop::new();

//     // […]
//     // added
//     let builder = glutin::WindowBuilder::new().with_dimensions(800, 600);
//     let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));
//     let context = glutin::ContextBuilder::new()
//         .with_gl(gl_version)
//         .with_vsync(true);
//     let window = glutin::GlWindow::new(builder, context, &event_loop).unwrap();

//     window.show();

//     let gl = unsafe {
//         window
//             .context()
//             .make_current()
//             .expect("Couldn't make window current");
//         gl::GlFns::load_with(|s| window.context().get_proc_address(s) as *const _)
//     };

//     // Will be use used by Servo later
//     let event_loop_waker = Box::new(GlutinEventLoopWaker {
//         proxy: Arc::new(event_loop.create_proxy())
//     });

//     let path = env::current_dir().unwrap().join("resources");
//     let path = path.to_str().unwrap().to_string();
//     // set_resources_path(Some(path));
//     // 変更
//     opts::set_options(opts::default_opts());

//     let inner_size = TypedSize2D::new(width as u32, height as u32);

//     let window = Rc::new(Window {
//         gl_context: window,
//         screen_size: nil,
//         inner_size: nil,
//         mouse_down_button: Cell::new(None),
//         mouse_down_point: Cell::new(TypedPoint2D::new(0, 0)),
//         primary_monitor: event_loop.get_primary_monitor(),
//         event_queue: RefCell::new(vec![]),
//         mouse_pos: Cell::new(TypedPoint2D::new(0, 0)),
//         last_pressed: Cell::new(None),
//         animation_state: Cell::new(AnimationState::Idle),
//         fullscreen: Cell::new(false),
//         gl: gl.clone(),
//     });

//     let mut servo = servo::Servo::new(event_loop_waker, window.clone());

//     // let url = ServoUrl::parse("https://servo.org").unwrap();
//     // let (sender, receiver) = ipc::channel().unwrap();
//     // servo.handle_events(vec![WindowEvent::NewBrowser(url, sender)]);
//     // let browser_id = receiver.recv().unwrap();
//     // servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

//     event_loop.run_forever(|_event| {
//         glutin::ControlFlow::Continue
//     });
// }

// impl WindowMethods for Window {
//     // fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
//     fn prepare_for_composite(&self) -> bool {
//         // true
//     }

//     fn present(&self) {
//         self.gl_context.borrow().swap_buffers();
//         self.gl_context.borrow_mut().make_not_current();
//         }

//     // fn supports_clipboard(&self) -> bool {
//     //     false
//     // }

//     // fn create_event_loop_waker(&self) -> Box<EventLoopWaker> {
//     //     self.waker.clone()
//     // }

//     fn gl(&self) -> Rc<gl::Gl> {
//         self.gl.clone()
//     }

//     // fn hidpi_factor(&self) -> TypedScale<f32, DeviceIndependentPixel, DevicePixel> {
//     //     TypedScale::new(self.glutin_window.hidpi_factor())
//     // }

//     // fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
//     //     let (width, height) = self.glutin_window.get_inner_size().unwrap();
//     //     let scale_factor = self.glutin_window.hidpi_factor() as u32;
//     //     TypedSize2D::new(scale_factor * width, scale_factor * height)
//     // }

//     // fn window_rect(&self) -> TypedRect<u32, DevicePixel> {
//     //     TypedRect::new(TypedPoint2D::new(0, 0), self.framebuffer_size())
//     // }

//     // fn size(&self) -> TypedSize2D<f32, DeviceIndependentPixel> {
//     //     let (width, height) = self.glutin_window.get_inner_size().unwrap();
//     //     TypedSize2D::new(width as f32, height as f32)
//     // }

//     // fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
//     //     let (width, height) = self.glutin_window.get_inner_size().unwrap();
//     //     let (x, y) = self.glutin_window.get_position().unwrap();
//     //     (Size2D::new(width, height), Point2D::new(x as i32, y as i32))
//     // }

//     // fn set_inner_size(&self, _id: BrowserId, _size: Size2D<u32>) {}

//     // fn set_position(&self, _id: BrowserId, _point: Point2D<i32>) {}

//     // fn set_fullscreen_state(&self, _id: BrowserId, _state: bool) {}

//     // fn set_page_title(&self, _id: BrowserId, title: Option<String>) {
//     //     self.glutin_window.set_title(match title {
//     //         Some(ref title) => title,
//     //         None => "",
//     //     });
//     // }

//     // fn status(&self, _id: BrowserId, _status: Option<String>) {}

//     // fn allow_navigation(&self, _id: BrowserId, _url: ServoUrl, chan: ipc::IpcSender<bool>) {
//     //     chan.send(true).ok();
//     // }

//     // fn load_start(&self, _id: BrowserId) {}

//     // fn load_end(&self, _id: BrowserId) {}

//     // fn load_error(&self, _id: BrowserId, _: NetError, _url: String) {}

//     // fn head_parsed(&self, _id: BrowserId) {}

//     // fn history_changed(&self, _id: BrowserId, _entries: Vec<LoadData>, _current: usize) {}

//     // fn set_cursor(&self, cursor: CursorKind) {
//     //     let cursor = match cursor {
//     //         CursorKind::Pointer => glutin::MouseCursor::Hand,
//     //         _ => glutin::MouseCursor::Default,
//     //     };
//     //     self.glutin_window.set_cursor(cursor);
//     // }

//     // fn set_favicon(&self, _id: BrowserId, _url: ServoUrl) {}

//     // fn handle_key(
//     //     &self,
//     //     _id: Option<BrowserId>,
//     //     _ch: Option<char>,
//     //     _key: Key,
//     //     _mods: KeyModifiers,
//     // ) {
//     // }

//     // fn handle_panic(&self, _id: BrowserId, _reason: String, _backtrace: Option<String>) {}

//     // fn screen_avail_size(&self, _id: BrowserId) -> Size2D<u32> {
//     //     let monitor = self.glutin_window.get_current_monitor();
//     //     let (monitor_width, monitor_height) = monitor.get_dimensions();
//     //     Size2D::new(monitor_width, monitor_height)
//     // }

//     // fn screen_size(&self, _id: BrowserId) -> Size2D<u32> {
//     //     let monitor = self.glutin_window.get_current_monitor();
//     //     let (monitor_width, monitor_height) = monitor.get_dimensions();
//     //     Size2D::new(monitor_width, monitor_height)
//     // }
    
//     // add
//     fn get_coordinates(&self) -> EmbedderCoordinates {
//         let dpr = TypedScale::new(self.glutin_window.hidpi_factor());
//         let LogicalSize { width, height } = self
//             .gl_context
//             .borrow()
//             .window()
//             .get_outer_size()
//             .expect("Failed to get window outer size.");
//         let LogicalPosition { x, y } = self
//             .gl_context
//             .borrow()
//             .window()
//             .get_position()
//             .unwrap_or(LogicalPosition::new(0., 0.));
//         let screen = (self.screen_size.to_f32() * dpr).to_i32();
//         let win_size = (TypedSize2D::new(width as f32, height as f32) * dpr).to_i32();
//         let win_origin = (TypedPoint2D::new(x as f32, y as f32) * dpr).to_i32();
//         let LogicalSize { width, height } = self
//             .gl_context
//             .borrow()
//             .window()
//             .get_inner_size()
//             .expect("Failed to get window inner size.");
//         let inner_size = (TypedSize2D::new(width as f32, height as f32) * dpr).to_i32();
//         let viewport = DeviceIntRect::new(TypedPoint2D::zero(), inner_size);
//         let framebuffer = FramebufferIntSize::from_untyped(&viewport.size.to_untyped());
//         EmbedderCoordinates {
//             viewport,
//             framebuffer,
//             window: (win_size, win_origin),
//             screen: screen,
//             screen_avail: screen,
//             hidpi_factor: dpr,
//         }
//     }

//     fn set_animation_state(&self, state: AnimationState) {
//         self.animation_state.set(state);
//     }
// }


// pub enum GlContext {
//     Current(WindowedContext<PossiblyCurrent>),
//     NotCurrent(WindowedContext<NotCurrent>),
//     // Used a temporary value as we switch from Current to NotCurrent.
//     None,
// }

// impl GlContext {
//     pub fn window(&self) -> &glutin::Window {
//         match self {
//             GlContext::Current(c) => c.window(),
//             GlContext::NotCurrent(c) => c.window(),
//             GlContext::None => unreachable!(),
//         }
//     }
//     pub fn resize(&mut self, size: glutin::dpi::PhysicalSize) {
//         if let GlContext::NotCurrent(_) = self {
//             self.make_current();
//         }
//         match self {
//             GlContext::Current(c) => c.resize(size),
//             _ => unreachable!(),
//         }
//     }
//     pub fn make_current(&mut self) {
//         *self = match std::mem::replace(self, GlContext::None) {
//             GlContext::Current(c) => {
//                 println!("Making an already current context current");
//                 GlContext::Current(c)
//             },
//             GlContext::NotCurrent(c) => {
//                 let c = unsafe {
//                     c.make_current().expect("Couldn't make window current")
//                 };
//                 GlContext::Current(c)
//             }
//             GlContext::None => unreachable!(),
//         }
//     }
//     pub fn make_not_current(&mut self) {
//         *self = match std::mem::replace(self, GlContext::None) {
//             GlContext::Current(c) => {
//                 let c = unsafe {
//                     c.make_not_current().expect("Couldn't make window not current")
//                 };
//                 GlContext::NotCurrent(c)
//             },
//             GlContext::NotCurrent(c) => {
//                 println!("Making an already not current context not current");
//                 GlContext::NotCurrent(c)
//             }
//             GlContext::None => unreachable!(),
//         }
//     }
//     pub fn swap_buffers(&self) {
//         match self {
//             GlContext::Current(c) => {
//                 if let Err(err) = c.swap_buffers() {
//                     println!("Failed to swap window buffers ({}).", err);
//                 }
//             },
//             GlContext::NotCurrent(_) => {
//                 println!("Context is not current. Forgot to call prepare_for_composite?");
//             },
//             GlContext::None => unreachable!(),
//         };
//     }
// }

// --------------------------------------------------------------
// Glutin's event loop (2)
// https://github.com/antoyo/servo-gtk/blob/master/src/view.rs

extern crate servo;
extern crate glutin;

// use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use glutin::GlContext;
use servo::BrowserId;
use servo::compositing::windowing::{EmbedderMethods, WindowMethods, EmbedderCoordinates};
use servo::compositing::windowing::{AnimationState, WindowEvent};
use servo::embedder_traits::EventLoopWaker; // change
use servo::euclid::{TypedPoint2D, TypedScale, TypedSize2D, TypedVector2D, };
use servo::gl;
// use servo::ipc_channel::ipc;
use servo::script_traits::{TouchEventType};
use servo::servo_config::opts;
//use servo::servo_config::resource_files::set_resources_path;
// use servo::embedder_traits::resources::ResourceReaderMethods::
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
    // Use by servo to share the "event loop waker" across threads
    fn clone(&self) -> Box<dyn EventLoopWaker + Send> {
        Box::new(GlutinEventLoopWaker {
            proxy: self.proxy.clone(),
        })
    }
    // Called by servo when the main thread needs to wake up
    fn wake(&self) {
        self.proxy.wakeup().expect("wakeup eventloop failed");
    }
}

// https://doc.servo.org/servo/struct.Servo.html
impl EmbedderMethods for GlutinEventLoopWaker {
    fn create_event_loop_waker(&mut self) -> Box<dyn EventLoopWaker> {
        Box::new(GlutinEventLoopWaker { 
            proxy: self.proxy.clone() 
        })
    }
}

struct Window {
    glutin_window: glutin::GlWindow,
    // waker: Box<dyn EventLoopWaker>,
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

    // resources_dir_path();
    resources::set(Box::new(ResourceReader));

    opts::set_options(opts::default_opts()); // change

    let window = Rc::new(Window {
        glutin_window: window,
        // waker: event_loop_waker,
        animation_state: Cell::new(AnimationState::Idle),
        gl: gl,
    });

    let mut servo = servo::Servo::new(event_loop_waker, window.clone());

    let url = ServoUrl::parse("https://kansock.industries/").unwrap();
    // let (sender, receiver) = ipc::channel().unwrap();
    // let browser_id = receiver.recv().unwrap();
    let browser_id = BrowserId::new();
    servo.handle_events(vec![WindowEvent::NewBrowser(url, browser_id)]); // change
    servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

    let mut pointer = (0.0, 0.0);

    event_loop.run_forever(|event| {
        match event {
            glutin::Event::Awakened => {
                servo.handle_events(vec![]);
            }

            // Mousemove
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

            // reload when R is pressed
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

            // Scrolling
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
    // let path = path.to_str().unwrap().to_string();
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