extern crate hidapi;

use std::{thread, time};
use hidapi::{HidApi};
use rand::random;

mod controller;
mod config;
use controller::Controller;
use config::Config;


fn main() {
    let api = HidApi::new().unwrap();
    let mut controller: Option<(u16, u16)> = None;

    for x in api.device_list() {
        if let 0x054C = x.vendor_id() { // If it's a ps4 controller
            controller = Some((x.vendor_id(), x.product_id()));
        }
    }

    match controller {
        Some(x) => {
            // If the program found a PS4 controller

            let device = Controller::new(
                    api.open(x.0, x.1).expect("No permissions to open the hardware.")
                );

            let conf = Config::read();

            match conf["pattern"].as_str().unwrap() {
                // Pattern-specific behavior

                "full" => {
                    loop {
                        device.rumble(
                            conf["small_motor_level"].as_u64().unwrap() as u8,
                            conf["big_motor_level"].as_u64().unwrap() as u8
                        )
                    }
                }
                "random" => {
                    let mut values: (u8, u8);
                    loop {
                        if random() {
                            values = (
                            conf["small_motor_level"].as_u64().unwrap() as u8,
                                conf["big_motor_level"].as_u64().unwrap() as u8
                                )
                        } else {
                            values = (0, 0)
                        }
                        device.rumble(values.0, values.1);
                        thread::sleep(time::Duration::from_millis(
                            conf["delay_millis"].as_u64().unwrap()
                        ));
                    }
                }
                x => {
                    let (valid, seq) = is_valid_pattern(x);
                    if valid {
                        loop {
                            for char in seq.chars() {
                                match char {
                                    'O' => {
                                        device.rumble(0, 0);
                                    },
                                    'X' => {
                                        device.rumble(
                                            conf["small_motor_level"].as_u64().unwrap() as u8,
                                            conf["big_motor_level"].as_u64().unwrap() as u8
                                        )
                                    },
                                    _ => {}
                                }
                                thread::sleep(time::Duration::from_millis(
                                    conf["delay_millis"].as_u64().unwrap()
                                ));
                            }
                        }
                    }
                }
            }
        },
        None => {
            println!("No controller found.")
        }
    }
}

fn is_valid_pattern(string: &str) -> (bool, String) {

    let mut failed_char_counter = 0;
    let string = string.chars().map(|c| format!("{}", c.to_uppercase())).collect::<Vec<String>>();
    let joined = &string.join("");

    for char in string {
        match &char[..] {
            "X" | "O" => {},
            _ => {failed_char_counter += 1}
        }
    }

    if failed_char_counter == 0 {
        return (true, joined.to_string());
    }
    return (false, joined.to_string());
}
