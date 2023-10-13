use clap::{ArgAction, Parser};
use notify_rust::{Hint, Notification, Timeout};
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::io::Write;
use std::time::Duration;
use std::{thread, time};

#[derive(Parser)]
struct Time {
    /// Enter hours for the timer.
    #[arg(long, short = 'r', default_value_t = 0)]
    hour: i16,

    /// Enter minutes for the timer.
    #[arg(short, long, default_value_t = 0)]
    minute: i16,

    /// Enter seconds for the timer.
    #[arg(short, long, default_value_t = 0)]
    second: i16,

    /// Use if you want a timer beep.
    #[arg(short, long, default_value_t = false, action=ArgAction::SetTrue)]
    beep: bool,

    /// Use if you do not want a timer notification.
    #[arg(short, long, default_value_t = true, action=ArgAction::SetFalse)]
    notification: bool,
}

fn format_time(hour: i16, minute: i16, second: i16) -> String {
    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

fn timer(mut hour: i16, mut minute: i16, mut second: i16) -> bool {
    let mut total_time = hour * 3600 + minute * 60 + second;
    let one_second = time::Duration::from_secs(1);
    while total_time > 0 {
        total_time -= 1;
        print!("\r{}", format_time(hour, minute, second));
        let _ = std::io::stdout().flush();
        if hour == 00 && minute == 00 && second == 00 {
            break;
        } else if hour > 00 && minute == 00 && second == 00 {
            hour -= 1;
            minute = 59;
            second = 59;
        } else if second > 00 {
            second -= 1;
        } else if second == 00 {
            minute -= 1;
            second = 59;
        }
        thread::sleep(one_second);
    }
    return true;
}

fn beep() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(600.0)
        .take_duration(Duration::from_secs_f32(0.5))
        .amplify(1.0);
    sink.append(source);

    sink.sleep_until_end();
}

fn main() {
    let time: Time = Time::parse();

    let hour = time.hour;
    let minute = time.minute;
    let second = time.second;
    let beep_var = time.beep;
    let notification = time.notification;

    timer(hour, minute, second);

    if notification == true {
        Notification::new()
            .summary("Time is up!")
            //.body("")
            .icon("dialog-information")
            .appname("cli-timer")
            .hint(Hint::Category("Device".to_owned()))
            .timeout(Timeout::Milliseconds(10000))
            .show()
            .unwrap();
    }

    if beep_var == true {
        beep();
        thread::sleep(Duration::from_secs_f32(0.25));
        beep();
        thread::sleep(Duration::from_secs_f32(0.25));
        beep();
        thread::sleep(Duration::from_secs_f32(0.75));
        beep();
        thread::sleep(Duration::from_secs_f32(0.25));
        beep();
        thread::sleep(Duration::from_secs_f32(0.25));
        beep();
        thread::sleep(Duration::from_secs_f32(0.25));
    }
}
