//#![windows_subsystem="windows"]

mod even_scale;
mod app;
mod style;

use std::process;
use iced::{Sandbox,Settings};

fn main(){
    let window_setting=iced::window::Settings{
        size: (640,480),
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
        default_font: Some(include_bytes!("..\\fonts\\NotoSansJP-Regular.otf")),
        ..Settings::default()
    };

    if let Err(err)=app::State::run(settings){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }
}