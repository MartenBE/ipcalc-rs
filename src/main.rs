use clap::Parser;
use std::net::Ipv4Addr;

mod ipv4;

#[derive(Parser)]
struct Args {
    #[arg(help = "print addresses in binary.", long, required = false, short)]
    binary: bool,

    #[arg(value_name = "PREFIX")]
    ip: String,

    #[arg(
        help = "Split the provided network using the provided prefix/netmask.",
        long,
        required = false,
        short,
        value_name = "PREFIX"
    )]
    split: Option<String>,
}

fn main() {
    let args = Args::parse();

    let ip_parts = args.ip.split_terminator('/').collect::<Vec<_>>();
    let ip_address_str = ip_parts[0];

    let ip_address = parse_ipv4_address(ip_address_str);

    ipv4::print_info(ip_address.into());

    let format = if args.binary {
        ipv4::Ipv4Format::Binary
    } else {
        ipv4::Ipv4Format::Decimal
    };

    if ip_parts.len() == 2 {
        println!();

        let prefix_str = ip_parts[1];
        let prefix = parse_prefix(prefix_str);

        ipv4::print_subnet_info(ip_address.into(), prefix, format);

        if let Some(split_prefix_str) = args.split {
            println!();
            let split_prefix = parse_prefix(&split_prefix_str);
            ipv4::print_subnet_splits(ip_address.into(), prefix, split_prefix, format);
        }
    }
}

fn parse_ipv4_address(ip_str: &str) -> Ipv4Addr {
    ip_str.parse::<Ipv4Addr>().unwrap_or_else(|_| {
        eprintln!("ERROR! Bad IPv4 address: {}", ip_str);
        std::process::exit(1);
    })
}

fn parse_prefix(prefix_str: &str) -> u8 {
    prefix_str.parse::<u8>().unwrap_or_else(|_| {
        eprintln!("ERROR! Bad prefix: {}", prefix_str);
        std::process::exit(1);
    })
}
