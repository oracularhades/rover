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

use crate::HelloState;
use crate::get_time;

pub fn header_topbar_small() -> impl Widget<HelloState> {
    let time = get_time().user_preference;
    let label = Label::new(move |data: &HelloState, _env: &Env| {
        format!("{}:{}", time.hours.clone(), time.minutes.clone())
    })
    .with_font(UI_FONT_BOLD)
    .with_text_size(10.0);

    let icon_size = 12.0;
    // let mut camera_icon = Image::new(ImageBuf::from_data(include_bytes!("./assets/1f4f7_camera_3d.png")).unwrap()).fill_mode(FillStrat::Cover).fix_width(icon_size).fix_height(icon_size);
    let mut weather_icon = Image::new(ImageBuf::from_data(include_bytes!("./assets/1f325_sunbehindlargecloud_3d.png")).unwrap()).fill_mode(FillStrat::Cover).fix_width(icon_size).fix_height(icon_size);
    let mut album_artwork = Image::new(ImageBuf::from_data(include_bytes!("./placeholders/album-placeholder4.jpg")).unwrap()).fill_mode(FillStrat::Cover).fix_width(icon_size).fix_height(icon_size);

    let spacing = 8.0;
    let topbar_right = Flex::row()
    // .with_child(camera_icon)
    // .with_spacer(spacing)
    .with_child(weather_icon)
    .with_spacer(spacing)
    .with_child(album_artwork);

    let header_topbar = Flex::row()
    .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
    .with_child(label)
    // .with_child(label2)
    .with_child(topbar_right)
    .expand_width()
    .padding(Insets::new(6.0, 4.0, 6.0, 4.0))
    .background(Color::rgb8(15, 14, 14)).on_click(move |_event, _ctx, _env| {
        // Modify the mutable integer variable when the widget is clicked
        _ctx.current_view = 1;
    });

    header_topbar
}