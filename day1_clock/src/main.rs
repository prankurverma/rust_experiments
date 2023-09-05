use chrono::Local;

const BITS : [[&str; 11]; 5] = [
    [" ┏━━┓ ","   ╻  "," ┏━━┓ ", " ┏━━┓ "," ╻  ╻ "," ┏━━┓ "," ┏━━┓ "," ┏━━┓ "," ┏━━┓ "," ┏━━┓ ","    "],
    [" ┃  ┃ ","   ┃  ","    ┃ ", "    ┃ "," ┃  ┃ "," ┃    "," ┃    ","    ┃ "," ┃  ┃ "," ┃  ┃ "," ◾ "],
    [" ┃  ┃ ","   ┃  "," ┏━━┛ ", "  ━━┫ "," ┗━━┫ "," ┗━━┓ "," ┣━━┓ ","    ┃ "," ┣━━┫ "," ┗━━┫ ","    "],
    [" ┃  ┃ ","   ┃  "," ┃    ", "    ┃ ","    ┃ ","    ┃ "," ┃  ┃ ","    ┃ "," ┃  ┃ ","    ┃ "," ◾ "],
    [" ┗━━┛ ","   ╹  "," ┗━━━ ", " ┗━━┛ ","    ╹ "," ┗━━┛ "," ┗━━┛ ","    ╹ "," ┗━━┛ "," ┗━━┛ ","    "],
];

fn main() {
    print!("\x1b[2J\x1b[?25l");             // ANSI escape code to erase entire screen and make cursor invisible
    print!("\x1b[48;5;7m\x1b[38;5;6m");     // Make text background white, foreground blue
    loop {
        let local_data = Local::now();
        let time = local_data.format("%H:%M:%S").to_string();  // Get time info from Local time & date

        for row in &BITS {
            for bits in time.chars() {
                let col = match bits {
                    '0'..='9' => bits as usize - '0' as usize,
                    _ => 10,
                };
                print!("{} ", row[col]);
            }
            println!();
        }

        std::thread::sleep(std::time::Duration::from_millis(999));  // Sleep for < 1 sec
        print!("\x1b[5A");                                              // Move cursor 7 characters up
    }
}