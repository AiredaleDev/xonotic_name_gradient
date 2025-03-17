use std::{env, process::ExitCode};

fn main() -> ExitCode {
    // Argument parsing
    let args = env::args().collect::<Vec<String>>();
    let (left_color, name, right_color) = match &args[..] {
        [_, l, n, r] => (l, n, r),
        _ => {
            help();
            return ExitCode::FAILURE;
        }
    };

    let left_color_num =
        u16::from_str_radix(left_color, 16).expect("lh_color: failed to parse u16");
    let right_color_num =
        u16::from_str_radix(right_color, 16).expect("rh_color: failed to parse u16");

    // really unsure if this is ever going to be run but just in case...
    if name.is_empty() {
        println!("Empty name string! I can't work with this!");
        return ExitCode::FAILURE;
    }

    // Parse individual colors from the numbers.
    let (red_l, red_r) = (
        ((left_color_num & 0xF00) >> 8) as f32,
        ((right_color_num & 0xF00) >> 8) as f32,
    );
    let (gre_l, gre_r) = (
        ((left_color_num & 0x0F0) >> 4) as f32,
        ((right_color_num & 0x0F0) >> 4) as f32,
    );
    let (blu_l, blu_r) = (
        (left_color_num & 0x00F) as f32,
        (right_color_num & 0x00F) as f32,
    );

    let (step_r, step_g, step_b) = (
        (red_r - red_l) / (name.len() - 1) as f32,
        (gre_r - gre_l) / (name.len() - 1) as f32,
        (blu_r - blu_l) / (name.len() - 1) as f32,
    );

    let mut formatted_name = String::new();
    for (i, c) in name.chars().enumerate() {
        let color_shift = (((red_l + step_r * i as f32) as u16) << 8)
            + (((gre_l + step_g * i as f32) as u16) << 4)
            + (blu_l + step_b * i as f32) as u16;
        let color_str = format!("^x{:03X}{}", color_shift, c);
        formatted_name.push_str(&color_str);
    }

    println!("{}", formatted_name);

    ExitCode::SUCCESS
}

fn help() {
    println!("Usage: xonotic_color_gradient lh_color name rh_color");
    println!("Format colors as 000 to FFF. ^x will be inserted automatically.");
    println!("Run this without any args to display this message");
}
