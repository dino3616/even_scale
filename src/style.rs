use iced::{Background,button,Color,container,pick_list};

const BACKGROUND_R: u8=0x36;
const BACKGROUND_G: u8=0x39;
const BACKGROUND_B: u8=0x3F;
const BUTTON_R: u8=0xFF;
const BUTTON_G: u8=0x65;
const BUTTON_B: u8=0x1C;

pub struct Container;
impl container::StyleSheet for Container{
    fn style(&self)->container::Style{
        container::Style{
            background: Some(Background::Color(Color::from_rgb8(BACKGROUND_R,BACKGROUND_G,BACKGROUND_B))),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct Button;
impl button::StyleSheet for Button{
    fn active(&self)->button::Style{
        button::Style{
            background: Some(Background::Color(Color::from_rgb8(BUTTON_R,BUTTON_G,BUTTON_B))),
            border_radius: 10.0,
            text_color: Color::from_rgb8(BACKGROUND_R,BACKGROUND_G,BACKGROUND_B),
            ..button::Style::default()
        }
    }

    fn hovered(&self)->button::Style{
        button::Style{
            border_width: 1.0,
            border_color: Color::WHITE,
            ..self.active()
        }
    }

    fn pressed(&self)->button::Style{
        let active=self.active();

        button::Style{
            background: Some(Background::Color(active.text_color)),
            text_color: match active.background{
                Some(Background::Color(some))=>some,
                None=>panic!("Error<{}:{}>：不明なエラーが発生しました.\n",file!(),line!()),
            },
            ..active
        }
    }
}

pub struct PickList;
impl pick_list::StyleSheet for PickList{
    fn menu(&self)->pick_list::Menu{
        pick_list::Menu::default()
    }

    fn active(&self)->pick_list::Style{
        pick_list::Style{
            text_color: Color::from_rgb8(BACKGROUND_R,BACKGROUND_G,BACKGROUND_B),
            icon_size: 0.5,
            ..pick_list::Style::default()
        }
    }

    fn hovered(&self)->pick_list::Style{
        pick_list::Style{
            border_color: Color::BLACK,
            ..self.active()
        }
    }
}