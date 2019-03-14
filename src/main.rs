use std::io::{self};

enum Page {
    AddRecordPage,
    MainPage
}

enum Message {
    AddRecord(Record),
    ChangePage(Page),
    NoOp,
}

#[derive(Debug)]
struct Record {
    full_name: String,
    phone_number: String
}

struct State {
    current_page: Page,
    records: Vec<Record>
}

fn view(state: &State) -> Message {
    match state.current_page {
        Page::MainPage => {
            let mut buffer = String::new();
            println!("Welcome to the Address book!");
            println!("Please choose an option:");
            println!("1.) Add new record.");
            io::stdin().read_line(&mut buffer)
                .expect("Something unexpected happened.");

            match buffer.trim() {
                "1" => Message::ChangePage(Page::AddRecordPage),
                _ => Message::NoOp
            }
        }
        Page::AddRecordPage => {
            println!("What is the full name?");
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("Something unexpected happened.");

            let mut phone = String::new();
            println!("What is the phone number?");
            io::stdin().read_line(&mut phone)
                .expect("Something unexpected happened.");
            Message::AddRecord(Record { full_name: name.trim().to_string(), phone_number: phone.trim().to_string() })
        }
    }
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ChangePage(page) => {
            state.current_page = page
        },
        Message::AddRecord(name) => {
            state.records.push(name)
        },
        Message::NoOp => {
        }
    }
}

fn main() {
    let mut state: State = State { current_page: Page::MainPage, records: Vec::new() };
    while state.records.len() == 0 {
        let message = view(&state);
        update(&mut state, message)
    }

    println!("{:?}", state.records)
}
