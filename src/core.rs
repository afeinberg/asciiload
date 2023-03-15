use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};
use termion::{clear, cursor};

const START_CHAR: char = '|';
const FILL_CHAR: char = '█';
const END_CHAR: char = ' ';

const REFRESH_INTERVAL_SECS: u64 = 1;

pub(crate) fn main_loop() {
    let mut system = System::new_all();
    let mut stdout = stdout();

    loop {
        system.refresh_cpu();

        // Get the CPU load as a percentage
        let cpu_load = system.global_cpu_info().cpu_usage() as usize;

        // Clear the screen and move the cursor to the top-left corner
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        println!("CPU Load: {}%", cpu_load);
        print_bar_chart(cpu_load, 50);
        stdout.flush().unwrap();

        thread::sleep(Duration::from_secs(REFRESH_INTERVAL_SECS));
    }
}

fn print_bar_chart(value: usize, max_width: usize) {
    let filled_width = (value * max_width) / 100;

    print!("{}", START_CHAR);
    for i in 0..max_width {
        if i < filled_width {
            print!("{}", FILL_CHAR);
        } else {
            print!("{}", END_CHAR);
        }
    }
    println!("|");
}
