
use crate::frames::{*};

use termion::raw::IntoRawMode;
use termion::{async_stdin, color};
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;

pub fn parrot() {

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();

    // default param
    //let mut speed =
    let mut bool_stop = false; // Restart/Stop
    let mut color = "Rainbow"; // Red: 1, Green: 2, Blue: 3, Rainbow(X): 4
    let mut speed = 50;


    loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }

        if let Some(Ok(b'w')) = b {
            color = "White";
        }

        if let Some(Ok(b'r')) = b {
            color = "Red";
        }

        if let Some(Ok(b'g')) = b {
            color = "Green";
        }

        if let Some(Ok(b'b')) = b {
            color = "Blue";
        }

        if let Some(Ok(b'x')) = b {
            color = "Rainbow";
        }

        if let Some(Ok(b'1')) = b {
            speed = 25;
        }

        if let Some(Ok(b'2')) = b {
            speed = 50;
        }

        if let Some(Ok(b'3')) = b {
            speed = 100;
        }

        if let Some(Ok(b's')) = b {
            if bool_stop {
                bool_stop = false;
            }
            else {
                bool_stop = true;
            }
        }

        stdout.flush().unwrap();

        for i in 0..Frames::FRAME_LIST.len() {
            if bool_stop == false {
                // stdout.write_all(format!("\r {} \r", Frames::FRAME_LIST[i]).as_bytes(), color::Fg(color::Red)).unwrap();
                // stdout.write_all(format!("\r {} \r", Frames::FRAME_NONE).as_bytes()).unwrap();
                if color == "Rainbow" {
                    if i % 3 == 0 {
                        println!("{}{}", color::Fg(color::Red), Frames::FRAME_LIST[i]);
                    }
                    else if i % 3 == 1 {
                        println!("{}{}", color::Fg(color::Green), Frames::FRAME_LIST[i]);
                    }
                    else if i % 3 == 2 {
                        println!("{}{}", color::Fg(color::Blue), Frames::FRAME_LIST[i]);
                    }
                }

                if color == "White" {
                    println!("{}{}", color::Fg(color::White), Frames::FRAME_LIST[i]);
                }

                if color == "Red" {
                    println!("{}{}", color::Fg(color::Red), Frames::FRAME_LIST[i]);
                }

                if color == "Blue" {
                    println!("{}{}", color::Fg(color::Blue), Frames::FRAME_LIST[i]);
                }

                if color == "Green" {
                    println!("{}{}", color::Fg(color::Green), Frames::FRAME_LIST[i]);
                }

                println!("{}", Frames::FRAME_NONE);
                println!("{}Quit: q\tSpeed: [1-3]\tColor: [r, g, b, x, w]\tRestart/Stop: s", color::Fg(color::White));

            }
            //stdout.write_all(format!("\r {} \r", "Quit: q\tSpeed: [1-3]\tColor: [r, g, b, x]\tRestart/Stop: s").as_bytes()).unwrap();
            //println!("{}Quit: q\tSpeed: [1-3]\tColor: [r, g, b, x, w]\tRestart/Stop: s", color::Fg(color::White));
            stdout.flush().unwrap();
            write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
            thread::sleep(Duration::from_millis(speed));
            stdout.flush().unwrap();
        }

        stdout.flush().unwrap();

    }

}
