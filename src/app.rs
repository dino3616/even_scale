use iced::{Align,button,Button,Column,Container,Element,HorizontalAlignment,Length,pick_list,PickList,Row,Sandbox,Text};
use super::{even_scale::*,style::*};

#[derive(Default)]
pub struct State{
    pub pick_list: pick_list::State<Scale>,
    pub selected_scale: Option<Scale>,
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
            Message::ScaleSelect(scale)=>self.selected_scale=Some(scale),
            Message::Run=>self.should_run=true,
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
}