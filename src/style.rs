use iced::{Background,button,Color,container,pick_list};

const CONTAINER_R: u8=0x36;
const CONTAINER_G: u8=0x39;
const CONTAINER_B: u8=0x3F;
const BUTTON_R: u8=0xFF;
const BUTTON_G: u8=0x65;
const BUTTON_B: u8=0x1C;
const TEXT_R: u8=0xDE;
const TEXT_G: u8=0xDE;
const TEXT_B: u8=0xDE;

pub struct PickList;
impl pick_list::StyleSheet for PickList{
    fn menu(&self)->pick_list::Menu{
        pick_list::Menu::default()
    }

    fn active(&self)->pick_list::Style{
        pick_list::Style{
            text_color: Color::from_rgb8(CONTAINER_R,CONTAINER_G,CONTAINER_B),
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

pub struct Button;
impl button::StyleSheet for Button{
    fn active(&self)->button::Style{
        button::Style{
            background: Some(Background::Color(Color::from_rgb8(BUTTON_R,BUTTON_G,BUTTON_B))),
            border_radius: 10.0,
            text_color: Color::from_rgb8(CONTAINER_R,CONTAINER_G,CONTAINER_B),
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
        let hovered=self.hovered();

        button::Style{
            background: Some(Background::Color(active.text_color)),
            border_width: hovered.border_width,
            border_color: hovered.border_color,
            text_color: match active.background{
                Some(Background::Color(some))=>some,
                None=>panic!("Error<{}:{}>：不明なエラーが発生しました.\n",file!(),line!()),
            },
            ..active
        }
    }
}

pub struct Container;
impl container::StyleSheet for Container{
    fn style(&self)->container::Style{
        container::Style{
            text_color: Some(Color::from_rgb8(TEXT_R,TEXT_G,TEXT_B)),
            background: Some(Background::Color(Color::from_rgb8(CONTAINER_R,CONTAINER_G,CONTAINER_B))),
            border_radius: 10.0,
            ..container::Style::default()
        }
    }
}