use std::env;
use std::fs;
use std::thread;

use slint::SharedString;
use slint::Weak;

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

    fn iterate(&self, weak: Weak<App>) {
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

            println!("{}", sentence);

            _ = slint::invoke_from_event_loop(move || {
                handle_weak_copy
                    .unwrap()
                    .set_current_string(SharedString::from(&s))
            });

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
                        padding: 10px;
                        spacing: 5px;
                        Text {
                            text: current_string;
                            color: black;
                            wrap: word-wrap;
                        }
                        // for t in current_text : Text {
                        //     text: t;
                        //     color: t == current_string ? red : black;
                        //     wrap: word-wrap;
                        // }
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
    let ebook = Ebook::new(contents, 1, 300);

    thread::spawn(move || {
        ebook.iterate(weak);
    });

    app.run().unwrap();
}
