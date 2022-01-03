use std::process;
use iced::{button,Button,Color,Column,Container,Element,HorizontalAlignment,Length,pick_list,PickList,Row,Sandbox,Text};
use utau_rs::*;
use super::{even_scale::*,style::*};

#[derive(Default)]
pub struct State{
    selected_scale: Option<Scale>,
    uta_sections: UtaSections,
    pick_list: pick_list::State<Scale>,
    run: button::State,
}

#[derive(Clone,Debug)]
pub enum Message{
    ScaleSelect(Scale),
    Run,
}

impl Sandbox for State{
    type Message=Message;

    fn new()->Self{
        Self::default()
    }

    fn title(&self)->String{
        String::from("even scale")
    }

    fn update(&mut self,message: Message){
        match message{
            Message::ScaleSelect(scale)=>self.selected_scale=Some(scale),
            Message::Run=>{
                if let Err(err)=even_scale(&mut self.uta_sections,self.selected_scale.unwrap()){
                    eprint!("Error：{}\n",err);
                    process::exit(1);
                }

                if let Err(err)=self.uta_sections.write(){
                    eprint!("Error：{}\n",err);
                    process::exit(1);
                }
                process::exit(0);
            }
        }
    }

    fn view(&mut self)->Element<Message>{
        let scale_list=PickList::new(
            &mut self.pick_list,
            &Scale::ALL[..],
            self.selected_scale,
            Message::ScaleSelect,
        )
            .text_size(20)
            .style(PickList);

        let run=Button::new(
            &mut self.run,
            Text::new("Run")
                .size(30)
                .horizontal_alignment(HorizontalAlignment::Center)
        )
            .width(Length::Shrink)
            .height(Length::Shrink)    
            .min_width(80)
            .style(Button)
            .on_press(Message::Run);

        let contents=Column::new()
            .push(scale_list)
            .spacing(160)
            .push(
                Row::new()
                    .spacing(120)
            )
            .push(run);

        Container::new(contents)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(Container)
            .into()
    }

    fn background_color(&self)->Color{
        Color::TRANSPARENT
    }
}