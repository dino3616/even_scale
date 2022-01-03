mod even_scale;
mod app;
mod style;

use std::process;
use utau_rs::*;
use iced::{Sandbox,Settings};

fn main(){
    let mut uta_sections=UtaSections::default();
    if let Err(err)=uta_sections.write(){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }

    // 主要処理
    let window_setting=iced::window::Settings{
        size: (640,480),    // (320,240)
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

    if let Err(err)=uta_sections.write(){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }
}

fn _display_uta_sections(uta_sections: &UtaSections){
    for section in &uta_sections.sections{
        print!("[#{}]\n",section.section_name);
        print!("Length={}\n",section.length);
        print!("Lyric={}\n",section.lyric);
        print!("NoteNum={}\n",section.note_num);
    }
}