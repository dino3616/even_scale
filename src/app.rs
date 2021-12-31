use iced::{Align,button,Button,Column,Container,Element,HorizontalAlignment,Length,pick_list,PickList,Row,Sandbox,Text};
use super::{even_scale::*,style::*};

#[derive(Default)]
pub struct State{
    pub scale: Scale,
    pub run: button::State,
    pub cancel: button::State,
    pub should_run: bool,
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
            Message::ScaleSelect(scale)=>self.scale=scale,
            Message::Run=>self.should_run=true,
        }
    }

    fn view(&mut self)->Element<Message>{
        let run=Button::new(
            &mut self.run,
            Text::new("Run")
                .horizontal_alignment(HorizontalAlignment::Center)
        )
            .min_width(80)
            .on_press(Message::Run);

        let contents=Column::new()
            .spacing(160)
            .push(
                Row::new()
                    .spacing(120)
            )
            .push(run)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center);

        Container::new(contents)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(Container)
            .into()
    }
}