use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::process::exit;
use colored::Colorize;
use local_ip_address::local_ip;
use regex::Regex;

fn bitwise(data: (Ipv4Addr, char)) {
    let ip = data.0;
    let class = data.1;
    let ip_binary: String = format!("{:08b}.{:08b}.{:08b}.{:08b}", ip.octets()[0], ip.octets()[1], ip.octets()[2], ip.octets()[3]);

    
    match data.1 {
        'A' => {
            let host_part: String = format!("{}", host_part_vec[0]);
            let network_part: String = format!("{}", network_part_vec[0]);
            println!("{}",format!("Network part: {}", network_part.purple()).bold().blue());
            println!("{}",format!("Host part: {}", host_part.purple()).bold().blue());
        },
        'B' => {
            let host_part: String = format!("{}.{}", host_part_vec[0], host_part_vec[1]);
            let network_part: String = format!("{}.{}", network_part_vec[0], network_part_vec[1]);
            println!("{}",format!("Network part: {}", network_part.purple()).bold().blue());
            println!("{}",format!("Host part: {}", host_part.purple()).bold().blue());
        },
        'C' => {
            let host_part: String = format!("{}.{}.{}", host_part_vec[0], host_part_vec[1], host_part_vec[2]);
            let network_part: String = format!("{}.{}.{}", network_part_vec[0], network_part_vec[1], network_part_vec[2]);
            println!("{}",format!("Network part: {}", network_part.purple()).bold().blue());
            println!("{}",format!("Host part: {}", host_part.purple()).bold().blue());
        },
        _ => println!("Invalid class"),
    }
}

fn calc_subnet(octets: &Vec<u8>) -> (Ipv4Addr, char) {
    let submask: Ipv4Addr;
    let class: char;
    if octets[0] < 128 {
        submask = Ipv4Addr::new(255, 0, 0, 0);
        class = 'A';
    }
    else if octets[0] >= 128 && octets[0] < 192 {
        submask = Ipv4Addr::new(255, 255, 0, 0);
        class = 'B';
    }
    else if octets[0] >= 192 && octets[0] < 224 {
        submask = Ipv4Addr::new(255, 255, 255, 0);
        class = 'C';
    }
    else if octets[0] >= 224 && octets[0] < 240 {
        submask = Ipv4Addr::new(255, 255, 255, 255);
        class = 'D';
    }
    else {
        println!("{}", "Invalid IP address.".bold().red());
        exit(5);
    }
    let ip = IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]));
    println!("{}",format!("IP address: {}", ip.to_string().purple()).bold().blue());
    println!("{}",format!("Subnet Mask: {}", submask.to_string().purple()).bold().blue());
    println!("{}",format!("Class: {}", class.to_string().purple()).bold().blue());
    (submask, class)
}

fn main() {
    let reg = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    let ip: String;
    let inputted_ip: String = dialoguer::Input::<String>::new()
    .with_prompt("Enter IP address")
    .interact()
    .unwrap()
    .parse()
    .unwrap();

    if reg.is_match(&inputted_ip) {
        ip = inputted_ip
    }
    else {
        println!("{}", "Invalid IP address, using local ip address".bold().yellow());
        ip: Ipv4Addr = local_ip().unwrap()
    }
    
    let subnet = calc_subnet();
    bitwise(subnet);
}
