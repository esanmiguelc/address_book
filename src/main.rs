use std::io::{self};

enum Page {
    AddRecordPage,
    AllRecordsPage,
    MainPage
}

enum Message {
    AddRecord(Record),
    ChangePage(Page),
    ViewAllRecords,
    Exit,
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
            println!("Please choose an option:");
            println!("1.) Add new record.");
            println!("2.) View all records.");
            println!("0.) Exit.");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)
                .expect("Something unexpected happened.");

            match buffer.trim() {
                "1" => Message::ChangePage(Page::AddRecordPage),
                "2" => Message::ViewAllRecords,
                "0" => Message::Exit,
                _ => Message::NoOp
            }
        },
        Page::AllRecordsPage => {
            println!("Records:");
            for record in &state.records {
                println!("{}: No. {}", record.full_name, record.phone_number)
            }

            println!("Press any key to continue.");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)
                .expect("Something unexpected happened.");
            Message::ChangePage(Page::MainPage)
        },
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
            state.records.push(name);
            state.current_page = Page::MainPage
        },
        Message::ViewAllRecords => {
            state.current_page = Page::AllRecordsPage
        },
        Message::Exit => {
            std::process::exit(0)
        },
        Message::NoOp => {
        }
    }
}

fn main() {
    let mut state: State = State { current_page: Page::MainPage, records: Vec::new() };
    loop {
        let message = view(&state);
        update(&mut state, message)
    }
}
