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

pub fn app_library_list_item() -> impl Widget<String> {
    let label = Label::new(|data: &String, _env: &Env| {
        "Signal".to_string()
    })
    .with_font(UI_FONT_BOLD)
    .with_text_size(12.0);

    // let image_buf = load_image_from_file("xv9z9br6iwyb1.png");
    // let image_widget = Image::new(image_buf).fill_mode(FillStrat::Fill);

    let png_data = ImageBuf::from_data(include_bytes!("./placeholders/signal-icon.png")).unwrap();
    let mut app_icon = Image::new(png_data).fill_mode(FillStrat::Cover).fix_width(28.0).fix_height(28.0);

    Container::new(
        Flex::row()
        .with_child(app_icon)
        .with_spacer(4.0)
        .with_child(label)
        .padding(Insets::new(6.0, 8.0, 6.0, 8.0))
    ).align_vertical(UnitPoint::LEFT)
    .background(Color::rgb8(15, 14, 14))
    .expand_width()
    // .with_child(image_widget)
}