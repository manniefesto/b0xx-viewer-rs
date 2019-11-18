mod app;
mod gui;
mod support;

use self::support::*;
use crate::config::ViewerOptions;

use crate::serial_probe::*;

use conrod_core::widget_ids;
use conrod_glium::Renderer;
use glium::Surface;

const NOTO_FONT: &[u8] = include_bytes!("../../assets/fonts/NotoSans-Regular.ttf");

#[allow(unused_imports)]
use conrod_winit::{
    convert_event, convert_key, convert_mouse_button, convert_mouse_cursor, convert_window_event,
};

conrod_winit::conversion_fns!();

widget_ids! {
    pub struct Ids {
        frame,
        start_btn,
        y_btn,
        x_btn,
        b_btn,
        a_btn,
        l_btn,
        r_btn,
        z_btn,
        up_btn,
        down_btn,
        right_btn,
        left_btn,
        mod_x_btn,
        mod_y_btn,
        c_left_btn,
        c_right_btn,
        c_up_btn,
        c_down_btn,
        start_label,
        y_label,
        x_label,
        b_label,
        a_label,
        l_label,
        r_label,
        z_label,
        up_label,
        down_label,
        right_label,
        left_label,
        mod_x_label,
        mod_y_label,
        c_left_label,
        c_right_label,
        c_up_label,
        c_down_label,
        fps_counter,
    }
}

pub fn start_gui(mut rx: crossbeam_channel::Receiver<B0xxMessage>, options: ViewerOptions) {
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        //.with_decorations(false)
        .with_title(WIN_TITLE)
        .with_resizable(false)
        .with_dimensions((WIN_W, WIN_H).into());

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_gl_robustness(if cfg!(profile = "release") {
            glium::glutin::Robustness::NoError
        } else {
            glium::glutin::Robustness::TryRobustLoseContextOnReset
        })
        .with_multisampling(4);

    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = GliumDisplayWinitWrapper(display);

    // Construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64])
        .theme(gui::theme())
        .build();

    let noto_font = ui
        .fonts
        .insert(rusttype::Font::from_bytes(NOTO_FONT).unwrap());

    ui.theme.font_id = Some(noto_font);

    let ids = Ids::new(ui.widget_id_generator());

    let image_map: conrod_core::image::Map<glium::texture::CompressedSrgbTexture2d> =
        conrod_core::image::Map::new();

    let mut app = app::ViewerApp::default();

    let mut renderer = Renderer::new(&display).unwrap();

    let mut pending_events = Vec::new();

    'main: loop {
        let mut maybe_state = match rx.try_iter().last() {
            Some(message) => match message {
                B0xxMessage::State(state) => {
                    debug!("{:#?}", state);
                    Some(state)
                }
                B0xxMessage::Error(e) => {
                    error!("{}", e);
                    drop(rx);
                    rx = reconnect();
                    None
                }
                B0xxMessage::Quit => {
                    break 'main;
                }
                B0xxMessage::Reconnect => {
                    drop(rx);
                    rx = reconnect();

                    None
                }
            },
            None => None,
        };

        if let Some(new_state) = maybe_state.take() {
            let changed = app.update_state(new_state);
            if changed {
                ui.handle_event(conrod_core::event::Input::Redraw);
            }
        }

        // Collect all pending events.
        events_loop.poll_events(|event| pending_events.push(event));

        // Handle all events.
        for event in pending_events.drain(..) {
            match &event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate a GUI demonstrating every widget type provided by conrod.
        gui::render_gui(&mut ui.set_widgets(), &ids, &mut app, &options);

        // Draw the `Ui`.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut frame = display.0.draw();
            frame.clear_color(0., 0., 0., 1.);
            renderer.draw(&display.0, &mut frame, &image_map).unwrap();
            frame.finish().unwrap();
        }
    }
}
