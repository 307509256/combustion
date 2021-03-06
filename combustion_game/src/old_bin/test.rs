extern crate engine;

use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::sync::{Arc, RwLock};

use glfw::{Glfw, Action, Context, Key, WindowHint, WindowEvent};

use backend::window::WindowBuilder;

use error::*;

use graphics::{RenderSignal, FullscreenToggle};

fn main() {
    common::log::init_global_logger("logs").expect("Could not initialize logging system!");

    let mut glfw = Arc::new(RwLock::new(glfw::init(glfw::FAIL_ON_ERRORS).expect_logged_box("Could not initialize GLFW!")));

    let (mut window, events) = WindowBuilder::new(glfw.clone())
        .try_modern_context_hints()
        .size(1280, 720)
        //.aspect_ratio(16, 9)
        .common_hints(&[
            WindowHint::Visible(true),
            //WindowHint::Samples(Some(2)),
            WindowHint::DoubleBuffer(true),
            //WindowHint::OpenGlDebugContext(true),
        ])
        .title("Combustion")
        .create()
        .expect_logged_box("Couldn't create window");

    info!("Window created");

    //Enable interactivity
    window.set_all_polling(true);

    //Load up all the OpenGL functions from the process
    backend::gl::bindings::load_all_with(|symbol| window.get_proc_address(symbol) as *const _);

    //Enable debugging of OpenGL messages
    //backend::gl::enable_debug(backend::gl::default_debug_callback, true).unwrap();

    //Create channel for forwarding events to the render thread
    let (tx, rx) = mpsc::channel();

    // Disconnect current context
    glfw::make_context_current(None);

    //Create Send-able context to send to render thread
    let context = window.render_context();

    let running = Arc::new(AtomicBool::new(true));

    let render_running = running.clone();

    //Start render thread
    let render_thread: thread::JoinHandle<_> = thread::Builder::new().name("Render thread".to_string()).spawn(move || {
        use graphics::render::RenderLoopState;

        info!("Render thread started...");

        //Make the OpenGL context active on the render thread
        glfw::make_context_current(Some(&context));

        let mut state: RenderLoopState = RenderLoopState::new(60.0);

        state.unpause();

        {
            let res = graphics::render::start(&mut state, context, &rx);

            render_running.store(false, Ordering::SeqCst);

            //Once rendering has ended, free the OpenGL context
            glfw::make_context_current(None);

            res
        }.expect_logged_box("Render thread crashed");

        info!("Finished after {} frames", state.total_frames());
    }).expect_logged_box("Could not create Render thread");

    //Create fullscreen toggle in primary thread
    let mut fullscreen = FullscreenToggle::new();

    macro_rules! send_and_unpark {
        ($event:expr) => ({
            let ret = tx.send($event);
            render_thread.thread().unpark();
            ret
        })
    }

    info!("Listening for events...");

    //Since the primary thread will do nothing but wait on events, do that
    'event_loop: while !window.should_close() {
        //Instead of polling, actively block the thread since nothing else is happening in it
        glfw.wait_events();

        //While most events are simply forwarded to the
        for (_, event) in glfw::flush_messages(&events) {
            // Do NOT send any events if the render thread cannot accept them.
            if running.load(Ordering::SeqCst) {
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true);
                    }
                    WindowEvent::Key(Key::F11, _, Action::Press, _) => {
                        fullscreen.toggle(&mut glfw, &mut window);
                    }
                    WindowEvent::FramebufferSize(width, height) |
                    WindowEvent::Size(width, height) if width > 0 && height > 0 => {
                        send_and_unpark!(RenderSignal::ViewportResize(width, height)).unwrap();
                    }
                    WindowEvent::Iconify(iconified) if iconified => {
                        send_and_unpark!(RenderSignal::Pause).unwrap();
                    }
                    WindowEvent::Focus(focus) => {
                        if focus {
                            send_and_unpark!(RenderSignal::Resume).unwrap();
                        } else {
                            send_and_unpark!(RenderSignal::Pause).unwrap();
                        }
                    }
                    _ => {
                        tx.send(RenderSignal::Event(event)).unwrap();
                    }
                }
            } else {
                window.set_should_close(true);
                break 'event_loop;
            }
        }
    }

    info!("Shutting down...");

    if running.swap(false, Ordering::SeqCst) {
        //Signal the render thread to close
        send_and_unpark!(RenderSignal::Stop).expect_logged("Failed to signal render task.");
    }

    render_thread.join().expect_logged("Failed to join render thread");

    info!("Goodbye");
}