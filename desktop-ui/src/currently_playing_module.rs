use std::sync::Arc;
use druid::widget::ZStack;
use druid::{
    im,
    kurbo::{Affine, BezPath, Circle, Point},
    piet::{FixedLinearGradient, GradientStop, ImageBuf, InterpolationMode},
    widget::{
        prelude::*, Button, Checkbox, FillStrat, Container, Flex, Image, Label, List, Painter, ProgressBar, Svg, SvgData,
        RadioGroup, Scroll, Slider, Spinner, Stepper, Switch, TextBox, RangeSlider
    },
    AppLauncher, Color, Data, Lens, Widget, WidgetExt, WidgetPod, WindowDesc, UnitPoint, Vec2
};
use druid::theme::UI_FONT_BOLD;
use druid::widget::CrossAxisAlignment;
use druid::Insets;

use crate::{HelloState, HomeState};
use crate::structs::CurrentlyPlaying;

pub fn currently_playing_module() -> impl Widget<CurrentlyPlaying> {
    let label = Label::new(|data: &CurrentlyPlaying, _env: &Env| {
        data.title.clone()
    })
    .with_font(UI_FONT_BOLD)
    .with_text_size(11.0);

    let label_content = Label::new(|data: &CurrentlyPlaying, _env: &Env| {
        data.artist.clone()
    })
    .with_text_size(11.0)
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap);

    // let image_buf = load_image_from_file("xv9z9br6iwyb1.png");
    // let image_widget = Image::new(image_buf).fill_mode(FillStrat::Fill);

    let png_data = ImageBuf::from_data(include_bytes!("./placeholders/album-placeholder4.jpg")).unwrap();
    let mut app_icon = Image::new(png_data).fill_mode(FillStrat::Cover).fix_width(30.0).fix_height(30.0);

    let move_backward = include_str!("./assets/icons/media/previous.svg").parse::<SvgData>().unwrap();
    let play_svg = include_str!("./assets/icons/media/play.svg").parse::<SvgData>().unwrap();
    let move_forward = include_str!("./assets/icons/media/skip.svg").parse::<SvgData>().unwrap();

    Container::new(
        Flex::column()
        .with_child(
            Flex::row()
            .with_child(app_icon)
            .with_spacer(6.0)
            .with_flex_child(Flex::column()
                .with_child(label)
                .with_spacer(2.0)
                .with_child(label_content)
                .cross_axis_alignment(CrossAxisAlignment::Start), 1.0)
        )
        .with_spacer(6.0)
        .with_child(
            Flex::row()
            .with_child(
                Flex::row()
                .with_child(Svg::new(move_backward.clone()))
                .with_child(Svg::new(play_svg.clone()))
                .with_child(Svg::new(move_forward.clone()))
            )
            .with_child(
                Flex::row()
                .with_child(Slider::new()
                    .with_range(0.0, 100.0)
                    .with_step(0.10)
                    .lens(CurrentlyPlaying::progress).fix_width(280.0))
                .expand_width().padding(Insets::new(6.0, 0.0, 6.0, 0.0))
            )
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .expand_width()
        )
    )
    .align_vertical(UnitPoint::TOP)
    .align_horizontal(UnitPoint::LEFT)
    // .background(Color::rgb8(255, 0, 0))
    .padding(Insets::new(8.0, 8.0, 8.0, 8.0))
    .expand_width()
    .background(Color::rgb8(33, 33, 33))
    // .with_child(image_widget)
}