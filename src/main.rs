#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod scales;
use iced::text_input::{self, TextInput};
use iced::window;
use iced::{
    pick_list, Align, Column, Container, Element, HorizontalAlignment, Length, PickList, Sandbox,
    Settings, Text,
};
use scales::Scales;

pub fn main() -> iced::Result {
    TempConverter::run(Settings {
        window: window::Settings {
            size: (400, 400),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct TempConverter {
    temperature: f32,
    input_state: String,
    input: text_input::State,
    scale: Scales,
    pick_scale: pick_list::State<Scales>,
}

#[derive(Debug, Clone)]
enum Message {
    TempChanged(String),
    ScaleSelected(Scales),
}

impl Sandbox for TempConverter {
    type Message = Message;

    fn new() -> Self {
        TempConverter {
            temperature: 100.0,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Temperature converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TempChanged(val) => {
                let temp: f32 = match val.trim().parse() {
                    Ok(num) => num,
                    Err(_) => self.temperature,
                };

                self.temperature = temp;
                self.input_state = val;
            }
            Message::ScaleSelected(scale) => {
                self.scale = scale;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Temperature converter").size(40);

        let temps = Text::new(self.scale.get_conversions(self.temperature))
            .horizontal_alignment(HorizontalAlignment::Center);

        let input = TextInput::new(
            &mut self.input,
            "Input temperature",
            &self.input_state,
            Message::TempChanged,
        )
        .width(Length::Fill)
        .padding(15);

        let scale_list = PickList::new(
            &mut self.pick_scale,
            &Scales::ALL[..],
            Some(self.scale),
            Message::ScaleSelected,
        );

        let content = Column::new()
            .width(Length::Units(700))
            .spacing(20)
            .align_items(Align::Center)
            .push(title)
            .push(temps)
            .push(input)
            .push(scale_list);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
