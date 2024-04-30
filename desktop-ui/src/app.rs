use std::sync::Arc;
use druid::widget::ZStack;
use druid::{
    im,
    kurbo::{Affine, BezPath, Circle, Point},
    piet::{FixedLinearGradient, GradientStop, ImageBuf, InterpolationMode},
    widget::{
        prelude::*, Button, Checkbox, FillStrat, Container, Flex, Image, Label, List, Painter, ProgressBar,
        RadioGroup, Scroll, Slider, Spinner, Stepper, Switch, TextBox
    },
    AppLauncher, Color, Data, Lens, Widget, WidgetExt, WidgetPod, WindowDesc, UnitPoint, Vec2
};
use druid::theme::UI_FONT_BOLD;
use druid::Insets;
use crate::currently_playing_module::currently_playing_module;
use crate::notification_component::notification_component;

use crate::HelloState;
use crate::get_time;

const VERTICAL_WIDGET_SPACING: f64 = 0.0;

pub fn app_screen() -> impl Widget<HelloState> {
    let label = Label::new(
        ""
    )
    .with_text_size(12.0);

    let png_data = ImageBuf::from_data(include_bytes!("./placeholders/signal_img.png")).unwrap();
    let mut img = Image::new(png_data).fill_mode(FillStrat::Cover);

    let shader = Color::rgba8(15, 14, 14, 0);

    ZStack::new(label)
    // .with_child(date_time_flex, Vec2::new(1.0, 1.0),
    // Vec2::ZERO,
    // UnitPoint::BOTTOM,
    // Vec2::new(0.0, -140.0))
    .background(shader)
}