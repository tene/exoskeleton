use std::{
    fs::File,
    io::{self, BufRead},
    iter::FromIterator,
    thread, time,
};

use dirs;
use druid::{
    im,
    kurbo::{Affine, BezPath, Circle, Point},
    piet::{FixedLinearGradient, GradientStop, InterpolationMode},
    widget::{
        prelude::*, Button, Checkbox, Controller, FillStrat, Flex, Image, Label, List, Painter,
        ProgressBar, RadioGroup, Scroll, Slider, Spinner, Stepper, Switch, TextBox,
    },
    AppDelegate, AppLauncher, Code, Color, Data, ImageBuf, KeyEvent, Lens, TextAlignment, Widget,
    WidgetExt, WidgetPod, WindowDesc,
};
use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng, Rng};

lazy_static! {
    static ref sample_lines: Vec<String> = {
        let mut file_name = dirs::home_dir().unwrap();
        file_name.push(".bash_history");
        io::BufReader::new(File::open(file_name.as_path()).unwrap())
            .lines()
            .filter_map(Result::ok)
            .collect()
    };
}

#[derive(Clone, Data, PartialEq, Eq)]
pub enum ActionStatus {
    Running,
    Failed,
    Success,
}

#[derive(Clone, Data, Lens)]
pub struct Action {
    pub command: String,
    pub output: im::Vector<String>,
    pub status: ActionStatus,
}

impl Action {
    pub fn new(command: String) -> Self {
        let output = im::Vector::new();
        let status = ActionStatus::Running;
        Self {
            command,
            output,
            status,
        }
    }
    pub fn make_widget() -> impl Widget<Action> {
        Flex::column()
            .with_child(
                Label::raw()
                    .with_text_alignment(TextAlignment::Start)
                    .background(Color::grey(0.2))
                    .expand_width()
                    .lens(Action::command),
            )
            .with_child(
                List::new(|| {
                    Label::raw()
                        .with_text_alignment(TextAlignment::Start)
                        .background(Color::grey(0.1))
                        .expand_width()
                })
                .lens(Action::output),
            )
            .border(Color::BLACK, 1.0)
            .expand_width()
    }
    pub fn gen_fake() -> Self {
        let rng = &mut thread_rng();
        let command = sample_lines.choose(rng).unwrap().clone();
        let count = rng.gen_range(1..10);
        let output = im::Vector::from_iter(
            sample_lines
                .choose_multiple(rng, count)
                .map(String::to_owned),
        );
        let status = ActionStatus::Success;
        Self {
            command,
            output,
            status,
        }
    }
}

#[derive(Clone, Data, Lens)]
struct ExoState {
    input: String,
    items: im::Vector<Action>,
}

impl ExoState {
    fn new() -> Self {
        let input = String::new();
        //let items = im::Vector::new();
        let items = im::Vector::from_iter((1..20).map(|_| Action::gen_fake()));
        Self { input, items }
    }
}
fn render_items() -> impl Widget<ExoState> {
    Scroll::new(List::new(Action::make_widget).lens(ExoState::items))
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
            data.items.push_back(Action::new(data.input.split_off(0)));
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
