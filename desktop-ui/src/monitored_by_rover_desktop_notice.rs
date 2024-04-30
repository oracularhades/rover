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

pub fn monitored_by_rover_desktop_notice() -> impl Widget<HelloState> {
    let some_else_is_about_to_text = Label::new(
        format!("This device is controlled")
    )
    .with_line_break_mode(LineBreaking::WordWrap)
    // .with_font(UI_FONT_BOLD)
    .with_text_size(12.0)
    .align_horizontal(UnitPoint::LEFT);

    let if_this_is_your_personal_device_text = Label::new(format!("example.com is controlling this device, not you. Click here for more details."))
    .with_line_break_mode(LineBreaking::WordWrap)
    .with_text_size(12.0)
    .align_horizontal(UnitPoint::LEFT);

    let shader = Color::rgba8(0, 0, 0, 100);

    let flex = Flex::column()
    .with_child(some_else_is_about_to_text)
    .with_spacer(4.0)
    .with_child(if_this_is_your_personal_device_text)
    .align_horizontal(UnitPoint::LEFT)
    .align_vertical(UnitPoint::LEFT)
    .background(shader);

    flex
}