#![warn(non_snake_case)]

use std::{net::IpAddr, time::Duration};
use winping::{Buffer, Pinger};
use colored::Colorize;

fn clear_console() {
    let _ = std::process::Command::new("cmd")
        .arg("/C")
        .arg("cls")
        .status();
}   
fn ipcheck(){

    let mut website: String = Default::default();
    println!("Enter the webiste's url to check the ip adress of");
    std::io::stdin().read_line(&mut website).unwrap();
    let combinedstring: String = "ping ".to_owned() + &website;
    let pingweb = std::process::Command::new("cmd")
        .arg("/C")
        .arg(combinedstring)
        .output()
        .expect("Failed to write command");



    let stdout = String::from_utf8_lossy(&pingweb.stdout);
    clear_console();
    println!("Copy the ip address out (You have 10 seconds)\nCommand output: {}", stdout);
    std::thread::sleep(Duration::from_secs(10));
    clear_console();
    println!("To paste the ip into the terminal use a right click");
    main();
}
fn main() {
    //templates
    
    let online = "Server ".to_owned() + "[" + &"ONLINE".green().to_owned() + "]";
    let mut dst: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0));
    let offline: String = "Server ".to_owned() + "[" + &"OFFILNE".red().to_owned() + "]";
    //gather data
    let mut answ: String = Default::default();
    let mut ipaddr: Vec<IpAddr> = Vec::new();
    let mut destination: String = Default::default();
    loop {
        answ.clear();
        destination.clear();
        println!("Add a destination ONLY IP ADDRESSES. Example : 1.1.1.1\nDo you want to check the ip adress of a website? Press 0");
        std::io::stdin().read_line(&mut destination).unwrap();
        if destination.trim() == "0" {
            ipcheck();    
        }
        
        dst = std::env::args()
            .nth(1)
            .unwrap_or(String::from(destination.trim()))
            .parse::<IpAddr>()
            .expect("Could not parse IP Address");

        ipaddr.push(dst);
        //re initialize

        println!("Do you want to add another ip?\n0) No\n1) Yes");
        std::io::stdin().read_line(&mut answ).unwrap();
        if answ.trim() == "0" {
            break;
        }
        else if answ.trim() == "1" {
            clear_console();
            continue;
        }
    }
    //create pinger
        let pinger = Pinger::new().unwrap();
        let mut buffer = Buffer::new();
    //ping counter
    let mut counter : i64 = Default::default();
    //ping servers
    loop { 
        clear_console();
        println!("{}. Ping", counter);
        for i in 0..ipaddr.len(){
            match pinger.send(ipaddr[i], &mut buffer) {
                Ok(rtt) => {println!("'{}' {} Response time {} ms.", ipaddr[i], online , rtt)},
                Err(err) => println!("'{}' {}. {}", ipaddr[i], offline, err),
            }}
        std::thread::sleep(Duration::from_secs(5));
        counter = counter + 1;
    }
}
