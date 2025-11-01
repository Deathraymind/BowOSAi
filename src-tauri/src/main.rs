use iced::widget::{button, column, scrollable, text, text_input};
use iced::{Center, Element, Fill, Task};

mod api;
#[allow(non_snake_case)]
mod imageAnalyzer;

struct OpenAiGui {
    image_path: String,
    input: String,
    response: String,
    analyzing: bool,
}

impl Default for OpenAiGui {
    fn default() -> Self {
        // Check for command-line arguments
        let args: Vec<String> = std::env::args().collect();
        let image_path = if args.len() > 1 {
            println!("Image path from args: {}", args[1]);
            args[1].clone()
        } else {
            String::new()
        };

        Self {
            image_path,
            input: String::new(),
            response: String::new(),
            analyzing: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ImagePathChanged(String),
    SendPressed,
    ResponseReceived(String),
    ResponseError(String),
}

impl OpenAiGui {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(s) => {
                self.input = s;
                Task::none()
            }
            Message::ImagePathChanged(p) => {
                self.image_path = p;
                Task::none()
            }
            Message::SendPressed => {
                let path = self.image_path.clone();
                if path.is_empty() {
                    return Task::future(async {
                        Message::ResponseError("No image path provided".to_string())
                    });
                }
                self.analyzing = true;
                self.response = "Analyzing image...".to_string();
                
                Task::future(async move {
                    match imageAnalyzer::ai_request(&path).await {
                        Ok(resp) => Message::ResponseReceived(resp),
                        Err(e) => Message::ResponseError(format!("Error: {}", e)),
                    }
                })
            }
            Message::ResponseReceived(resp) => {
                self.response = resp;
                self.analyzing = false;
                Task::none()
            }
            Message::ResponseError(err) => {
                self.response = err;
                self.analyzing = false;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let image_input = text_input("Image path...", &self.image_path)
            .on_input(Message::ImagePathChanged)
            .padding(10)
            .size(20);

        let context_input = text_input("Optional text/context...", &self.input)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let send_button = if self.analyzing {
            button("Analyzing...").padding(10)
        } else {
            button("Analyze").on_press(Message::SendPressed).padding(10)
        };

        let output = scrollable(column![text(&self.response)])
            .height(Fill);

        column![image_input, context_input, send_button, output]
            .spacing(10)
            .padding(20)
            .align_x(Center)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::run("OpenAI Image Analyzer", OpenAiGui::update, OpenAiGui::view)
}
