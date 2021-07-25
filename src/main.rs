use std::{thread, time};

use druid::{
    im,
    kurbo::{Affine, BezPath, Circle, Point},
    piet::{FixedLinearGradient, GradientStop, InterpolationMode},
    widget::{
        prelude::*, Button, Checkbox, FillStrat, Flex, Image, Label, List, Painter, ProgressBar,
        RadioGroup, Scroll, Slider, Spinner, Stepper, Switch, TextBox,
    },
    AppDelegate, AppLauncher, Code, Color, Data, ImageBuf, KeyEvent, Lens, Widget, WidgetExt,
    WidgetPod, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppData {
    input: String,
    items: im::Vector<String>,
}

impl AppData {
    fn new() -> Self {
        let input = String::new();
        let items = im::Vector::new();
        Self { input, items }
    }
}
fn render_items() -> impl Widget<AppData> {
    Scroll::new(
        List::new(|| Label::new(|data: &String, _: &_| format!("{}", data)).expand_width())
            .expand_height()
            .lens(AppData::items),
    )
    .vertical()
}
fn render_input() -> impl Widget<AppData> {
    TextBox::new().lens(AppData::input).expand_width()
}

fn ui_builder() -> impl Widget<AppData> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_flex_child(render_items(), 1.0)
        .with_child(render_input())
}

#[derive(Default)]
struct Delegate;

impl AppDelegate<AppData> for Delegate {
    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        window_id: druid::WindowId,
        event: Event,
        data: &mut AppData,
        env: &Env,
    ) -> Option<Event> {
        match event {
            Event::KeyDown(KeyEvent {
                code: Code::Enter, ..
            }) => {
                data.items.push_back(data.input.split_off(0));
                None
            }
            _ => Some(event),
        }
    }

    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppData,
        env: &Env,
    ) -> druid::Handled {
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        id: druid::WindowId,
        data: &mut AppData,
        env: &Env,
        ctx: &mut druid::DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        id: druid::WindowId,
        data: &mut AppData,
        env: &Env,
        ctx: &mut druid::DelegateCtx,
    ) {
    }
}

fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Widget Gallery");
    let data = AppData::new();
    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();
    thread::spawn(move || background_thread(event_sink));

    launcher
        .use_simple_logger()
        .delegate(Delegate::default())
        .launch(data)
        .expect("launch failed");
    println!("Hello, world!");
}

fn background_thread(_event_sink: druid::ExtEventSink) -> () {
    loop {
        thread::sleep(time::Duration::from_secs(10))
    }
}
