mod home_screen;
mod lock_screen;
mod notification_component;
mod app_library_list_item;
mod currently_playing_module;
mod header_topbar_small;
mod header_topbar_expanded;
mod structs;
mod app;
mod monitored_by_rover_desktop_notice;

use chrono::{Local, Timelike, Datelike};
use std::{sync::Arc, borrow::BorrowMut};
use druid::{
    im,
    kurbo::{Affine, BezPath, Circle, Point},
    piet::{FixedLinearGradient, GradientStop, ImageBuf, InterpolationMode},
    widget::{
        prelude::*, Button, Checkbox, FillStrat, Container, Flex, Image, Label, List, Painter, ProgressBar,
        RadioGroup, Scroll, Slider, Spinner, Stepper, Switch, TextBox, ViewSwitcher
    },
    AppLauncher, Color, Data, Lens, Widget, WidgetExt, WidgetPod, WindowDesc, UnitPoint, Vec2, WindowLevel
};
use header_topbar_small::header_topbar_small;
use std::time::{SystemTime, UNIX_EPOCH};
use lock_screen::lock_screen;
use home_screen::home_screen;
use app::app_screen;
use monitored_by_rover_desktop_notice::monitored_by_rover_desktop_notice;
use header_topbar_expanded::header_topbar_expanded;
use structs::{ Notification, CurrentlyPlaying };

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    name: String,
    currently_playing: Arc<Vec<CurrentlyPlaying>>,
    list: Arc<Vec<String>>,
    notifications: Arc<Vec<Notification>>,
    current_view: i64,
    home_state: HomeState
}

#[derive(Clone, Data, Lens)]
pub struct HomeState {
    search: String
}

#[derive(Clone, Data, Lens)]
pub struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32
}

#[derive(Clone, Data, Lens)]
pub struct TimeOutput {
    twenty_four_hour: Time,
    twelve_hour: Time,
    user_preference: Time,
    period_of_day: String,
    day: u32,
    month: u32,
    month_name: String,
    month_short: String,
    weekday: String,
    weekday_short: String,
    year: i32
}

pub fn main() {
    // create the initial app state
    let initial_state: HelloState = HelloState {
        name: "".into(),
        currently_playing: Arc::new(vec![ CurrentlyPlaying { title: "Keep Driving".into(), artist: "Harry Styles".into(), artwork: "./placeholders/album-placeholder4.jpg".into(), pause_enabled: true, move_forward: true, move_backward: true, seek: true, progress: 2.0 }]),
        // list: Arc::new(vec!["1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into(), "1".into(), "2".into(), "3".into()]),
        list: Arc::new(vec!["1".into()]),
        notifications: Arc::new(vec![Notification { title: "Hayden".to_string(), description: "Unlock to view".to_string() }, Notification { title: "Hayden".to_string(), description: "Unlock to view".to_string() }]),
        current_view: 0,
        home_state: HomeState { search: "".to_string() }
    };

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget(&initial_state))
        .title("Rover")
        .window_size((400.0, 600.0));

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state.clone())
        .expect("Failed to launch application");

    // // describe the main window
    // let main_window = WindowDesc::new(build_root_widget(&initial_state))
    //     .transparent(true)
    //     .window_size((340.0, 200.0))
    //     .resizable(false)
    //     .show_titlebar(false)
    //     .set_always_on_top(false);

    // // start the application. Here we pass in the application state.
    // AppLauncher::with_window(main_window)
    //     .log_to_console()
    //     .launch(initial_state.clone())
    //     .expect("Failed to launch application");
}

fn get_time() -> TimeOutput {
    // Get the current local time
    let local_time = Local::now();

    // Extract hours, minutes, and seconds
    let hours = local_time.hour();
    let minutes = local_time.minute();
    let seconds = local_time.second();

    let day = local_time.day();
    let month = local_time.month();
    let year = local_time.year();
    let weekday = local_time.weekday().to_string();
    let weekday_short = weekday.get(0..3).unwrap_or(&weekday).to_string();

    let months = Arc::new(vec!["January", "Feburary", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]);
    let current_month = months[month as usize - 1].to_string();
    let month_short = current_month.get(0..3).unwrap_or(&current_month).to_string();

    TimeOutput {
        twenty_four_hour: Time {
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        },
        twelve_hour: Time {
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        },
        user_preference: Time {
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        },
        day: day,
        month: month,
        month_name: current_month,
        month_short: month_short,
        year: year,
        weekday: weekday,
        weekday_short: weekday_short,
        period_of_day: "".into()
    }
}

fn base(ui: impl Widget<HelloState> + 'static, data: &HelloState) -> impl Widget<HelloState> {
    ui
}

fn build_root_widget(data: &HelloState) -> impl Widget<HelloState> {
    let view_switcher = ViewSwitcher::new(
        |data: &HelloState, _env| data.current_view,
        |selector, _data, _env| match selector {
            0 => Box::new(base(lock_screen(), _data)),
            1 => Box::new(base(home_screen(), _data)),
            2 => Box::new(base(app_screen(), _data)),
            3 => Box::new(base(monitored_by_rover_desktop_notice(), _data)),
            _ => Box::new(Label::new("Invalid viewer.").center()),
        },
    );

    Flex::column()
    .with_child(view_switcher)
}