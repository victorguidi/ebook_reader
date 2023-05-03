use std::env;
use std::fs;

use slint::SharedString;

// TODO: Add support for other file types
// TODO: Finish implementing the GUI -> Slint.ui

struct Ebook {
    text: String,
    numberofwords: u32,
    speed: u32,
}

impl Ebook {
    fn new(text: String, numberofwords: u32, speed: u32) -> Ebook {
        Ebook {
            text,
            numberofwords,
            speed,
        }
    }

    fn iterate(&self, app: &App) {
        let mut current: usize = 0;
        let mut sentence = String::new();

        loop {
            let group = self.get_group(current);
            if group.is_empty() {
                break;
            }
            sentence.push_str(&group);
            println!("{}", sentence);
            let s = SharedString::from(&sentence);
            app.set_current_string(s);
            sentence.clear();
            std::thread::sleep(std::time::Duration::from_millis(self.speed as u64));

            current += self.numberofwords as usize;
            if current >= self.text.len() {
                break;
            }
        }
    }

    fn get_group(&self, current: usize) -> String {
        let words = self.text.split_whitespace();
        // let current_fn = |_| if current == 0 { current } else { current + 1 };
        let group = words
            .clone()
            .skip(current)
            .take(self.numberofwords as usize);
        let mut sentence = String::new();

        for word in group {
            sentence.push_str(word);
            sentence.push_str(" ");
        }

        sentence
    }
}

slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";
    export global ChangeText {}
    export component App inherits Window {
        in property <string> current_string: "";
        GridLayout {
            padding: 10px;
            spacing: 5px;
            Text { text: current_string; colspan: 3; }
        }
    }
}

fn main() {
    let app: App = App::new().unwrap();
    let args: Vec<String> = env::args().collect();
    let text_file = &args[1];

    let contents = fs::read_to_string(text_file).expect("Something went wrong reading the file");

    let ebook = Ebook::new(contents, 4, 300);
    ebook.iterate(&app);
    app.run().unwrap();
}
