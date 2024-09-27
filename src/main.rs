use std::{env, process::exit};
use colored::Colorize;
use local_ip_address::local_ip;
use regex::Regex;

fn first_octet(octets: &Vec<u8>) {
    let submask: &str;
    let class: char;
    if octets[0] < 128 {
        submask = "255.0.0.0";
        class = 'A';
    }
    else if octets[0] >= 128 && octets[0] < 192 {
        submask = "255.255.0.0";
        class = 'B';
    }
    else if octets[0] >= 192 && octets[0] < 224 {
        submask = "255.255.255.0";
        class = 'C';
    }
    else if octets[0] >= 224 && octets[0] < 240 {
        submask = "255.255.255.255";
        class = 'D';
    }
    else {
        println!("{}", "Invalid IP address.".bold().red());
        exit(5);
    }
    let ip: String = format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]);
    println!("{}",format!("IP address: {}", ip.purple()).bold().blue());
    println!("{}",format!("Subnet Mask: {}", submask.purple()).bold().blue());
    println!("{}",format!("Class: {}", class.to_string().purple()).bold().blue());
}

fn main() {
    let reg = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    let ip;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", "No IP address specified, using local ip address".bold().yellow());
        ip = local_ip().unwrap().to_string();
    }
    else if args.len() > 2 {
        println!("{}", "Too many arguments.".bold().red());
        exit(2);
    }
    else {
        ip = args[1].clone();
    }
        if !reg.is_match(&ip) {
            println!("{}", "Invalid IP address.".bold().red());
            exit(3);
        }

    let octets: Vec<u8> = ip.split(".").map(|x| match x.parse::<u8>() {
      Ok(x) => x,
      Err(_) => {
        println!("{}", "Invalid IP address.".bold().red());
        exit(4);
      }
    }).collect();
    
    let options = ["First octet", "Bitwise"];

    let theme = dialoguer::theme::ColorfulTheme::default();

    let selection = dialoguer::Select::with_theme(&theme)
    .with_prompt("Select method to calculate subnet mask")
    .items(&options)
    .interact()
    .unwrap();

    match selection {
        0 => first_octet(&octets),
        1 => println!("{}", "Bitwise not implemented.".bold().red()),
        _ => println!("{}", "Invalid choice.".bold().red()),
    }
}
