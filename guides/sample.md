# Pancakes.

![Pancakes](pancake)

Mmmm pancakes??

The following is a code block:

```shell
uv pip install pip
python --version
```

other thing: `mmm`

I don't really know what to put here..
- maybe something useful?
- who knows.

List.
1. one
2. three
3. two

**super bold text**

![bal](pancake)

*The FitnessGram™ Pacer Test is a multistage aerobic capacity test that progressively gets more difficult as it continues. The 20 meter pacer test will begin in 30 seconds. Line up at the start. The running speed starts slowly, but gets faster each minute after you hear this signal. [beep] A single lap should be completed each time you hear this sound. [ding] Remember to run in a straight line, and run as long as possible. The second time you fail to complete a lap before the sound, your test is over. The test will begin on the word start. On your mark, get ready, start.*

![Pancakes](pancake)
![Pancakes](pancake)

```rust
use iced::Task;

use crate::service::gui::{App, enums::Page, message::Message};

#[derive(Debug, Clone)]
pub enum GuideMessage {
    SearchText(String),
    OpenGuide(u32), // guide id
    MarkdownInteraction(String),
}

impl Into<Message> for GuideMessage {
    fn into(self) -> Message {
        Message::GuideMessage(self)
    }
}

pub fn update(app: &mut App, msg: GuideMessage) -> Task<Message> {
    match msg {
        GuideMessage::SearchText(t) => {
            app.data.learn_data.home_search = t;
            Task::none()
        }
        GuideMessage::OpenGuide(g) => {
            app.data.page = Page::Guide(g);
            Task::none()
        }
        GuideMessage::MarkdownInteraction(s) => {
            println!("interaction: {s}");
            let _ = open::that(s);
            Task::none()
        }
    }
}
```