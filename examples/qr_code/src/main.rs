use iced::widget::qr_code::{self, QRCode};
use iced::widget::{column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length, Sandbox, Settings, Theme};

pub fn main() -> iced::Result {
    QRGenerator::run(Settings::default())
}

#[derive(Default)]
struct QRGenerator {
    data: String,
    qr_code: Option<qr_code::Data>,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    DataChanged(String),
    ThemeChanged(Theme),
}

impl Sandbox for QRGenerator {
    type Message = Message;

    fn new() -> Self {
        QRGenerator::default()
    }

    fn title(&self) -> String {
        String::from("QR Code Generator - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DataChanged(mut data) => {
                data.truncate(100);

                self.qr_code = if data.is_empty() {
                    None
                } else {
                    qr_code::Data::new(&data).ok()
                };

                self.data = data;
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("QR Code Generator").size(70);

        let input =
            text_input("Type the data of your QR code here...", &self.data)
                .on_input(Message::DataChanged)
                .size(30)
                .padding(15);

        let choose_theme = row![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged,)
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let mut content = column![title, input, choose_theme]
            .width(700)
            .spacing(20)
            .align_items(Alignment::Center);

        if let Some(qr_code) = self.qr_code.as_ref() {
            content = content.push(QRCode::new(qr_code).cell_size(10));
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
