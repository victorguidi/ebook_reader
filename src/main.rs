use std::env;
use std::fs;
use std::thread;

use slint::SharedString;
use slint::VecModel;
use slint::Weak;

// TODO: Add support for other file types
// TODO: Finish implementing the GUI -> Slint.ui

struct Ebook {
    text: String,
    numberofwords: u32,
    speed: u32,
    current_set: Vec<SharedString>,
}

impl Ebook {
    fn new(text: String, numberofwords: u32, speed: u32) -> Ebook {
        Ebook {
            text,
            numberofwords,
            speed,
            current_set: Vec::new(),
        }
    }

    fn iterate_populate(&mut self) {
        let mut current: usize = 0;
        let mut sentence = String::new();

        loop {
            let group = self.get_group(current);
            if group.is_empty() {
                break;
            }

            sentence.push_str(&group);
            self.current_set.push(SharedString::from(&sentence));
            sentence.clear();

            current += self.numberofwords as usize;
            if current >= self.text.len() {
                break;
            }
        }
    }

    fn iterate(&mut self, weak: Weak<App>) {
        let mut current: usize = 0;
        let mut sentence = String::new();

        loop {
            let handle_weak_copy = weak.clone();
            let group = self.get_group(current);
            if group.is_empty() {
                break;
            }

            sentence.push_str(&group);
            let s = sentence.clone();
            // println!("sentence before {}", sentence);

            for i in 0..self.current_set.len() {
                if self.current_set[i] == sentence {
                    break;
                }
            }
            _ = slint::invoke_from_event_loop(move || {
                handle_weak_copy
                    .unwrap()
                    .set_current_string(SharedString::from(&s))
            });

            // println!("{}", sentence);

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

    export global current_text {
        callback populate([string]);
    }

    component side_bar inherits Rectangle {
        width: 20%;
        height: 100%;
        background: #2A3424;
    }

    component main_text_area inherits Rectangle {
        width: 80%;
        height: 100%;
        background: #D9D9D9;
    }

    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {

        min_width: 500px;
        min_height: 500px;

        in property <string> current_string;
        in property <[string]> current_text;

        GridLayout {
            width: 100%;
            height: 100%;
            Row {
                side_bar {}
                main_text_area {
                    VerticalBox {
                        alignment: center;
                        width: 100%;
                        height: 100%;
                        padding: 10px;
                        spacing: 5px;
                        for t in current_text : Text {
                            text: t;
                            color: t == current_string ? blue : black;
                            wrap: word-wrap;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let app: App = App::new().unwrap();
    let weak = app.as_weak();
    let args: Vec<String> = env::args().collect();
    let text_file = &args[1];

    let contents = fs::read_to_string(text_file).expect("Something went wrong reading the file");
    let mut ebook = Ebook::new(contents, 4, 300);

    ebook.iterate_populate();

    let vec_model = VecModel::from_slice(&ebook.current_set);
    weak.unwrap().set_current_text(vec_model);
    thread::spawn(move || {
        ebook.iterate(weak);
    });

    app.run().unwrap();
}
