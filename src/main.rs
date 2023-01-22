use iced::widget::{button, column, text};
use iced::{executor, Command, Theme};
use iced::{Alignment, Application, Element, Settings};
mod lex_parse;

struct Counter {
    value: i32,
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Counter, Command<Self::Message>) {
        (Counter { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced Test")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() -> iced::Result {
    Counter::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
