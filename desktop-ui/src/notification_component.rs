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
use druid::widget::CrossAxisAlignment;
use druid::{Insets};
use crate::Notification;

use crate::HelloState;
use crate::home_screen::home_screen;

#[cfg(feature = "svg")]
use druid::widget::{Svg, SvgData};

pub fn notification_component() -> impl Widget<Notification> {
    let label = Label::new(|data: &Notification, _env: &Env| {
        data.title.clone()
    })
    .with_font(UI_FONT_BOLD)
    .with_text_size(11.0);

    let label_content = Label::new(|data: &Notification, _env: &Env| {
        data.description.clone()
    })
    .with_text_size(11.0)
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap);

    // let image_buf = load_image_from_file("xv9z9br6iwyb1.png");
    // let image_widget = Image::new(image_buf).fill_mode(FillStrat::Fill);

    let png_data = ImageBuf::from_data(include_bytes!("./placeholders/signal-icon.png")).unwrap();
    let mut app_icon = Image::new(png_data).fill_mode(FillStrat::Cover).fix_width(30.0).fix_height(30.0);

    Container::new(
        Flex::row()
        .with_child(app_icon)
        .with_spacer(6.0)
        .with_flex_child(Flex::column()
            .with_child(label)
            .with_spacer(2.0)
            .with_child(label_content)
            .cross_axis_alignment(CrossAxisAlignment::Start), 1.0))
    .align_vertical(UnitPoint::TOP)
    .align_horizontal(UnitPoint::LEFT)
    // .background(Color::rgb8(255, 0, 0))
    .padding(Insets::new(8.0, 8.0, 8.0, 8.0))
    .expand_width()
    .background(Color::rgb8(33, 33, 33))
    // .with_child(image_widget)
}