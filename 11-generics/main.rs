#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:#?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let chatMessage1: ChatMessage<&str> = ChatMessage {
        content: "Hello",
        time: String::from("03/12/2025 @ 05:46 PM"),
    };

    let chatMessage2: ChatMessage<String> = ChatMessage {
        content: String::from("Hi"),
        time: String::from("03/12/2025 @ 05:47 PM"),
    };

    let chatMessage3 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("03/12/2025 @ 05:48 PM"),
    };
    chatMessage3.consume_entertainment();

    let chatMessage1Time: String = chatMessage1.retrieve_time();
    println!("Chat Message 1 Time: {}", chatMessage1Time);

    let chatMessage2Time: String = chatMessage2.retrieve_time();
    println!("Chat Message 2 Time: {}", chatMessage2Time);

    let chatMessage3Time: String = chatMessage3.retrieve_time();
    println!("Chat Message 3 Time: {}", chatMessage3Time);
}
