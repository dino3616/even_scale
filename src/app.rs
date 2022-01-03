use std::process;
use iced::{Align,Application,button,Button,Clipboard,Color,Column,Command,Container,Element,executor,Font,HorizontalAlignment,image,Image,Length,pick_list,PickList,Text};
use utau_rs::*;
use super::{even_scale::*,style::*};

#[derive(Default)]
pub struct State{
    selected_scale: Option<Scale>,
    uta_sections: UtaSections,
    pick_list: pick_list::State<Scale>,
    run: button::State,
    exit: bool,
}

#[derive(Clone,Debug)]
pub enum Message{
    ScaleSelect(Scale),
    Run,
}

impl Application for State{
    type Executor=executor::Default;
    type Message=Message;
    type Flags=();

    fn new(_flags: ())->(Self,Command<Self::Message>){
        (Self::default(),Command::none())
    }

    fn title(&self)->String{
        String::from("even scale")
    }

    fn update(&mut self,message: Self::Message,_clipboard: &mut Clipboard)->Command<Self::Message>{
        match message{
            Message::ScaleSelect(scale)=>self.selected_scale=Some(scale),
            Message::Run=>{
                let selected_scale=match self.selected_scale{
                    Some(some)=>some,
                    None=>{
                        self.exit=true;
                        return Command::none();
                    }
                };
                if let Err(err)=even_scale(&mut self.uta_sections,selected_scale){
                    eprint!("Error：{}\n",err);
                    process::exit(1);
                }

                if let Err(err)=self.uta_sections.write(){
                    eprint!("Error：{}\n",err);
                    process::exit(1);
                }
                self.exit=true;
            }
        }

        Command::none()
    }

    fn view(&mut self)->Element<Message>{
        let comment_text=Text::new("↓choose scale that you want↓")
            .font(Font::External{name: "BRADHITC",bytes: include_bytes!("../fonts/karakaze-R.otf")})
            .size(30)
            .horizontal_alignment(HorizontalAlignment::Center);

        let scale_list=PickList::new(
            &mut self.pick_list,
            &Scale::ALL[..],
            self.selected_scale,
            Message::ScaleSelect,
        )
            .text_size(20)
            .style(PickList);

        let image=Container::new(Image::new(image::Handle::from_path("resource/background0.png")))
            .align_x(Align::Center)
            .align_y(Align::Center);

        let run_button=Button::new(
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
            .align_items(Align::Center)
            .push(comment_text)
            .spacing(10)
            .push(scale_list)
            .push(image)
            .push(run_button);

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

    fn should_exit(&self)->bool{
        self.exit
    }
}