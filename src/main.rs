mod even_scale;
mod app;
mod style;

use std::process;
use utau_rs::*;
use iced::{Sandbox,Settings};

fn main(){
    let mut uta_io=match UtaIO::new(){
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error：{}\n",err);
            process::exit(1);
        }
    };
    if let Err(err)=uta_io.read(){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }

    print!("{}\n",uta_io.tmpfile);

    let mut uta_sections=UtaSections::new();
    if let Err(err)=uta_sections.read(&uta_io.file_data){
        eprint!("Error：{}\n",err);
        process::exit(1);
    }

    // 主要処理
    let window_setting=iced::window::Settings{
        size: (320,240),
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

    uta_io.file_data=match uta_sections.apply(){
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error：{}\n",err);
            process::exit(1);
        }
    };

    if let Err(err)=uta_io.write(){
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