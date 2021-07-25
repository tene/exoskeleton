use std::{thread, time};

use druid::{
    im,
    kurbo::{Affine, BezPath, Circle, Point},
    piet::{FixedLinearGradient, GradientStop, InterpolationMode},
    widget::{
        prelude::*, Button, Checkbox, Controller, FillStrat, Flex, Image, Label, List, Painter,
        ProgressBar, RadioGroup, Scroll, Slider, Spinner, Stepper, Switch, TextBox,
    },
    AppDelegate, AppLauncher, Code, Color, Data, ImageBuf, KeyEvent, Lens, Widget, WidgetExt,
    WidgetPod, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct ExoState {
    input: String,
    items: im::Vector<String>,
}

impl ExoState {
    fn new() -> Self {
        let input = String::new();
        let items = im::Vector::new();
        Self { input, items }
    }
}
fn render_items() -> impl Widget<ExoState> {
    Scroll::new(
        List::new(|| Label::new(|data: &String, _: &_| format!("{}", data)).expand_width())
            .lens(ExoState::items),
    )
    .vertical()
    .expand_height()
}
fn render_input() -> impl Widget<ExoState> {
    TextBox::new().lens(ExoState::input).expand_width()
}

fn ui_builder() -> impl Widget<ExoState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_flex_child(render_items(), 1.0)
        .with_child(render_input())
        .controller(ExoController)
}

#[derive(Default)]
struct ExoController;

impl<W: Widget<ExoState>> Controller<ExoState, W> for ExoController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut ExoState,
        env: &Env,
    ) {
        if let Event::KeyDown(KeyEvent {
            code: Code::Enter, ..
        }) = event
        {
            data.items.push_back(data.input.split_off(0));
        } else {
            child.event(ctx, event, data, env)
        }
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &ExoState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &ExoState,
        data: &ExoState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}
fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Widget Gallery");
    let data = ExoState::new();
    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();
    thread::spawn(move || background_thread(event_sink));

    launcher
        .use_simple_logger()
        //.delegate(Delegate::default())
        .launch(data)
        .expect("launch failed");
    println!("Hello, world!");
}

fn background_thread(_event_sink: druid::ExtEventSink) -> () {
    loop {
        thread::sleep(time::Duration::from_secs(10))
    }
}
