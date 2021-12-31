use iced::{Background,Color,container};

const BACKGROUND_R: u8=0x36;
const BACKGROUND_G: u8=0x39;
const BACKGROUND_B: u8=0x3F;

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