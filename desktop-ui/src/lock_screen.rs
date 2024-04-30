use std::sync::Arc;
use druid::widget::ZStack;
use druid::{
    im,
    kurbo::{Affine, BezPath, Circle, Point},
    piet::{FixedLinearGradient, GradientStop, ImageBuf, InterpolationMode},
    widget::{
        prelude::*, Button, Checkbox, FillStrat, Container, Flex, Image, Label, List, Painter, ProgressBar,
        RadioGroup, Scroll, Slider, Spinner, Stepper, Switch, TextBox, LineBreaking
    },
    AppLauncher, Color, Data, Lens, Widget, WidgetExt, WidgetPod, WindowDesc, UnitPoint, Vec2
};
use druid::Insets;

use crate::HelloState;
use crate::get_time;
use druid::DelegateCtx;

const VERTICAL_WIDGET_SPACING: f64 = 0.0;

// ctx: &mut DelegateCtx

pub fn lock_screen() -> impl Widget<HelloState> {
    // let new_win = WindowDesc::new(ui_builder())
    //     .menu(make_menu)
    //     .window_size((data.selected as f64 * 100.0 + 300.0, 500.0));
    // ctx.new_window(new_win);
    
    let some_else_is_about_to_text = Label::new(
        format!("Someone else is about to control this device, not you.")
    )
    .with_line_break_mode(LineBreaking::WordWrap)
    // .with_font(UI_FONT_BOLD)
    .with_text_size(24.0);

    let if_this_is_your_personal_device_text = Label::new(format!("If this is your personal device, do not continue. If you continue, admin@example.com will control this device. Without your admin's full compliance, it's almost impossible to remove their control without wiping your computer.\n\nRover is intended for corporate-owned devices. If this is your personal device, do not continue."))
    .with_line_break_mode(LineBreaking::WordWrap)
    .with_text_size(14.0);

    let continue_and_lose_control_button = Button::new("Continue & lose control").padding(10.0).on_click(move |_event, data: &mut i64, _env| {
        *data = 1;
    })
    .lens(HelloState::current_view).fix_height(50.0);

    let cancel_button = Button::new("Cancel").padding(10.0).on_click(move |_event, data: &mut i64, _env| {
        *data = 1;
    })
    .lens(HelloState::current_view).fix_height(50.0);

    let some_else_is_about_to_text_flex = Flex::column()
    .with_child(some_else_is_about_to_text)
    .with_spacer(4.0)
    .with_child(if_this_is_your_personal_device_text)
    .fix_height(100.0);

    let buttons_flex = Flex::row()
    .with_child(continue_and_lose_control_button)
    .with_spacer(0.0)
    .with_child(cancel_button);

    let flex = Flex::column()
    .with_child(some_else_is_about_to_text_flex)
    .with_spacer(60.0)
    .with_child(buttons_flex)
    .align_horizontal(UnitPoint::CENTER)
    .align_vertical(UnitPoint::CENTER);

    flex
}