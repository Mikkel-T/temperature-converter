#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod scales;

use scales::Scales;

use iced::font::{Font, Weight};
use iced::widget::{column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};

fn main() -> iced::Result {
    TemperatureConverter::run(Settings {
        window: iced::window::Settings {
            size: (400, 470),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct TemperatureConverter {
    /// The current temperature inputted
    temperature: f64,

    /// The current input
    input: String,

    /// The scale that is currently being used
    scale: Option<Scales>,
}

#[derive(Debug, Clone)]
enum Message {
    ScaleSelected(Scales),
    InputChanged(String),
    UpdateTemperature,
}

impl Sandbox for TemperatureConverter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Temperature converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ScaleSelected(scale) => {
                self.scale = Some(scale);
            }
            Message::InputChanged(temperature) => {
                self.input = temperature;
            }
            Message::UpdateTemperature => {
                self.temperature = self.input.trim().replace(',', ".").parse().unwrap_or(0.0);
                self.input = self.temperature.to_string();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let pick_list = pick_list(&Scales::ALL[..], self.scale, Message::ScaleSelected)
            .placeholder("Select a scale");
        let temp_column: Element<_> = column(
            Scales::ALL
                .iter()
                .map(|i| {
                    text(format!(
                        "{} {}",
                        self.scale
                            .unwrap_or_default()
                            .convert_to(i.to_owned(), self.temperature),
                        i.short()
                    ))
                    .size(25)
                    .into()
                })
                .collect(),
        )
        .align_items(Alignment::Center)
        .into();

        let input = text_input("Input temperature", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::UpdateTemperature);

        container(
            column![
                text("Temperature converter").size(30).font(Font {
                    weight: Weight::Bold,
                    ..Default::default()
                }),
                temp_column,
                row![input, text(self.scale.unwrap_or_default().short())]
                    .spacing(5)
                    .align_items(Alignment::Center),
                pick_list,
            ]
            .padding(30)
            .spacing(15)
            .align_items(Alignment::Center),
        )
        .center_x()
        .center_y()
        .into()
    }
}
