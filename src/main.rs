#![windows_subsystem="windows"]

mod even_scale;
mod app;
mod style;

use std::process;
use iced::{Application,Settings};

const WINDOW_WIDTH: u32=640;
const WINDOW_HEIGHT: u32=480;

fn main(){
    let window_setting=iced::window::Settings{
        size: (WINDOW_WIDTH,WINDOW_HEIGHT),
        min_size: None,
        max_size: None,
        resizable: false,
        decorations: false,
        transparent: true,
        always_on_top: false,
        icon: None,
    };
    let settings=Settings{
        window: window_setting,
        default_font: Some(include_bytes!("../fonts/NotoSansJP-Bold.otf")),
        ..Settings::default()
    };

    if let Err(err)=app::State::run(settings){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }
}