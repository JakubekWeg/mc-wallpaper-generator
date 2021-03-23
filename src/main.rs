mod image_converter;
mod random_helper;

use std::collections::HashMap;
use crate::image_converter::{convert_every_from_directory, convert_one, convert_using_random_method};
use sfml::window::VideoMode;

#[derive(Debug)]
pub enum TerracottaBehaviour {
    Never,
    Always,
    Auto
}

fn parse_terracotta_behaviour(what: &str) -> Option<TerracottaBehaviour> {
    match what {
        "never" => Some(TerracottaBehaviour::Never),
        "always" => Some(TerracottaBehaviour::Always),
        "auto" => Some(TerracottaBehaviour::Auto),
        _ => None
    }
}

#[derive(Debug)]
pub struct ProgramConfig<'a> {
    input_folder_name: &'a str,
    output_folder_name: &'a str,
    scale_factor: f32,
    image_width: u32,
    image_height: u32,
    terracotta_behaviour: TerracottaBehaviour,
    probabilities_str: &'a str
}

fn parse_image_size(val: &String, is_width: bool) -> u32 {
    if val != "auto" {
        return val.parse::<u32>().unwrap();
    }

    if is_width {
        VideoMode::desktop_mode().width
    } else {
        VideoMode::desktop_mode().height
    }
}

fn print_help() {
    println!(r#"This program generates repeated wallpapers
Parameter --action is required, possible values:
    transform-all - transforms all images from source folder
    transform-one - transforms single image (specified by source path)
    transform-random - combines multiple images using probability parameter

Other parameters:
    --from - specifies source folder or file or multiple images separated by commas in transform-random mode
    --to - specifies destination folder (in transform-all mode) or output file
    --scale - scale of single pixel, defaults to 8
    --width - size of image in pixels or "auto" word, auto is default, when auto mode is used then default desktop size gets used
    --height - size of image in pixels or "auto" word, auto is default, when auto mode is used then default desktop size gets used
    --terracotta - mode of terracota rotations, "always", "never" or "auto", auto mode is default
    --probabilities - probabilities of images specified in --from, used only in transform-random mode
"#)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.contains(&String::from("--help")) {
        print_help();
        return;
    }
    if args.len() == 0 {
        println!("This programs requires some arguments!");
        std::thread::sleep(std::time::Duration::from_secs(3));
    } else if args.len() % 2 == 1 {
        println!("This programs requires even arguments!");
    } else {
        let mut key = "";
        let mut is_even = true;
        let mut config_map: HashMap<&str, String> = HashMap::with_capacity(args.len());
        config_map.insert("--from", String::from(""));
        config_map.insert("--to", String::from(""));
        config_map.insert("--scale", String::from("8"));
        config_map.insert("--width", String::from("auto"));
        config_map.insert("--height", String::from("auto"));
        config_map.insert("--terracotta", String::from("auto"));
        config_map.insert("--probabilities", String::from(""));

        for value in &args {
            if is_even {
                key = &value;
            } else {
                config_map.insert(key, String::clone(value));
            }
            is_even = !is_even
        }


        let config = ProgramConfig {
            input_folder_name: config_map.get("--from").unwrap(),
            output_folder_name: config_map.get("--to").unwrap(),
            scale_factor: config_map.get("--scale").unwrap().parse::<f32>().expect("Parsing scale argument"),
            image_width: parse_image_size(config_map.get("--width").unwrap(), true),
            image_height:  parse_image_size(config_map.get("--height").unwrap(), false),
            terracotta_behaviour: parse_terracotta_behaviour(config_map.get("--terracotta").unwrap()).expect("Parsing terracotta argument"),
            probabilities_str: config_map.get("--probabilities").unwrap(),
        };


        if let Some(action) = config_map.get("--action") {
            match action.as_str() {
                "transform-all" => convert_every_from_directory(config),
                "transform-one" => convert_one(config),
                "transform-random" => convert_using_random_method(config),
                _ => println!("Invalid action option")
            }
        } else {
            println!("--action parameter is required");
        }
    }
}
