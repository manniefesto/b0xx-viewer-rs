use super::{app::ViewerApp, Ids};
use crate::config::ViewerOptions;
use crate::ui::support::{BTN_RADIUS, WIN_H, WIN_W};

/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "B0XX theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

pub fn render_gui(
    ui: &mut conrod_core::UiCell,
    ids: &Ids,
    app: &mut ViewerApp,
    options: &ViewerOptions,
) {
    use conrod_core::{widget, Colorable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(options.background_color.into())
        .w_h(WIN_W.into(), WIN_H.into())
        .x_y(0., 0.)
        .crop_kids()
        .set(ids.frame, ui);

    let (btn, mut m_text) = make_button(
        app.state.start,
        ids.frame,
        options.button_active_colors.start,
        options.button_inactive_colors.start,
        options.display_labels,
    );

    btn
        .x_y(0., 40.)
        .set(ids.start_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Start")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.right,
        ids.frame,
        options.button_active_colors.right,
        options.button_inactive_colors.right,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.start_btn, -100., 5.)
        .set(ids.right_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Right")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.down,
        ids.frame,
        options.button_active_colors.down,
        options.button_inactive_colors.down,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.right_btn, -45., 15.)
        .set(ids.down_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Down")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.left,
        ids.frame,
        options.button_active_colors.left,
        options.button_inactive_colors.left,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.down_btn, -45., -5.)
        .set(ids.left_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Left")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.l,
        ids.frame,
        options.button_active_colors.l,
        options.button_inactive_colors.l,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.left_btn, -45., -15.)
        .set(ids.l_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("L")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.mod_x,
        ids.frame,
        options.button_active_colors.mod_x,
        options.button_inactive_colors.mod_x,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.right_btn, 10., -120.)
        .set(ids.mod_x_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("MX")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.mod_y,
        ids.frame,
        options.button_active_colors.mod_y,
        options.button_inactive_colors.mod_y,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.mod_x_btn, 40., -20.)
        .set(ids.mod_y_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("MY")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.b,
        ids.frame,
        options.button_active_colors.b,
        options.button_inactive_colors.b,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.start_btn, 100., 5.)
        .set(ids.b_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("B")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.x,
        ids.frame,
        options.button_active_colors.x,
        options.button_inactive_colors.x,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.b_btn, 45., 15.)
        .set(ids.x_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("X")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.z,
        ids.frame,
        options.button_active_colors.z,
        options.button_inactive_colors.z,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.x_btn, 45., -5.)
        .set(ids.z_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Z")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.up,
        ids.frame,
        options.button_active_colors.up,
        options.button_inactive_colors.up,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.z_btn, 45., -15.)
        .set(ids.up_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Up")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.y,
        ids.frame,
        options.button_active_colors.y,
        options.button_inactive_colors.y,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.x_btn, 2., 45.)
        .set(ids.y_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Y")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.r,
        ids.frame,
        options.button_active_colors.r,
        options.button_inactive_colors.r,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.b_btn, 2., 45.)
        .set(ids.r_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("R")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.a,
        ids.frame,
        options.button_active_colors.a,
        options.button_inactive_colors.a,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.b_btn, -10., -120.)
        .set(ids.a_btn, ui);


    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("A")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.c_up,
        ids.frame,
        options.button_active_colors.c_up,
        options.button_inactive_colors.c_up,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.a_btn, 1., 48.)
        .set(ids.c_up_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CU")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }


    let (btn, mut m_text) = make_button(
        app.state.c_left,
        ids.frame,
        options.button_active_colors.c_left,
        options.button_inactive_colors.c_left,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.c_up_btn, -34., -24.)
        .set(ids.c_left_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CL")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.c_right,
        ids.frame,
        options.button_active_colors.c_right,
        options.button_inactive_colors.c_right,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.c_up_btn, 34., -24.)
        .set(ids.c_right_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CR")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.c_down,
        ids.frame,
        options.button_active_colors.c_down,
        options.button_inactive_colors.c_down,
        options.display_labels,
    );

    btn
        .x_y_relative_to(ids.c_left_btn, 0., -48.)
        .set(ids.c_down_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CD")
            .color(text_color)
            .middle_of(ids.c_down_btn)
            .set(ids.c_down_label, ui);
    }

    fps_counter(ui, ids, app);
}

#[cfg(not(feature = "fps"))]
fn fps_counter(_: &mut conrod_core::UiCell, _: &Ids, _: &mut ViewerApp) {}

#[cfg(feature = "fps")]
fn fps_counter(ui: &mut conrod_core::UiCell, ids: &Ids, app: &mut ViewerApp) {
    use conrod_core::{color, widget, Colorable, Positionable, Widget};

    let fps = app.fps.tick();
    widget::Text::new(&fps.to_string())
        .color(color::YELLOW)
        .top_right_with_margin_on(ids.frame, 10.)
        .floating(true)
        .set(ids.fps_counter, ui);
}

#[inline(always)]
fn make_button(
    state: bool,
    parent: conrod_core::widget::Id,
    active_color: crate::config::ViewerColor,
    inactive_color: crate::config::ViewerColor,
    display_labels: bool,
) -> (conrod_core::widget::Oval<conrod_core::widget::primitive::shape::oval::Full>, Option<conrod_core::Color>) {
    use conrod_core::{widget, Colorable, Sizeable, Widget};

    let color = if state { active_color } else { inactive_color };
    let text_color = if display_labels {
        let tmp: conrod_core::Color = color.clone().into();
        Some(tmp.plain_contrast())
    } else {
        None
    };

    (
        widget::Circle::fill(BTN_RADIUS)
            .color(color.into())
            .parent(parent)
            .graphics_for(parent)
            .w_h(BTN_RADIUS, BTN_RADIUS),
        text_color
    )
}
