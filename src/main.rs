// mod ipv4format;
// mod ipv4network;
// mod print;

use clap::Parser;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "Print addresses in binary.", long, short)]
    binary: Option<bool>,

    #[arg(value_name = "ADDRESS[/PREFIX]")]
    network_input: NetworkInput,

    #[arg(
        help = "Split the provided network using this prefix.",
        long,
        short,
        value_name = "PREFIX"
    )]
    split: Option<u8>,
}

#[derive(Debug, Clone)]
enum NetworkInput {
    Host(Ipv4Addr),
    SubnetWithPrefix(Ipv4Addr, u8),     // 192.168.1.0/25
    SubnetWithMask(Ipv4Addr, Ipv4Addr), // 192.168.1.0/255.255.255.0
}

impl FromStr for NetworkInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if let Some((ip_str, suffix)) = s.split_once('/') {
            let ip = match ip_str.parse::<Ipv4Addr>() {
                Ok(ip) => ip,
                Err(e) => return Err(format!("Invalid IP address: {e}")),
            };

            // Try prefix length first (e.g. /25), then subnet mask (e.g. /255.255.255.0)
            if let Ok(prefix) = suffix.parse::<u8>() {
                if prefix > 32 {
                    return Err(format!("Prefix length {prefix} is out of range (0-32)"));
                }
                Ok(NetworkInput::SubnetWithPrefix(ip, prefix))
            } else if let Ok(mask) = suffix.parse::<Ipv4Addr>() {
                Ok(NetworkInput::SubnetWithMask(ip, mask))
            } else {
                Err(format!(
                    "Invalid suffix '{suffix}': expected a prefix length (0-32) or a subnet mask"
                ))
            }
        } else {
            let ip = match s.parse::<Ipv4Addr>() {
                Ok(ip) => ip,
                Err(e) => return Err(format!("Invalid IP address: {e}")),
            };
            Ok(NetworkInput::Host(ip))
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.network_input {
        NetworkInput::Host(ip) => println!("Address: {}", ip),
        NetworkInput::SubnetWithPrefix(ip, prefix) => println!("Network with prefix: {}/{}", ip, prefix),
        NetworkInput::SubnetWithMask(ip, mask) => println!("Network with subnet mask: {}/{}", ip, mask),
    }
}
