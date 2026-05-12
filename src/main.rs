mod ipv4address;
mod ipv4network;

use clap::Parser;
use prettytable::{Table, format, row};
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "Print addresses in binary.", long, short)]
    binary: bool,

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
    Host(Ipv4Addr),                     // 192.168.1.0
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

            if let Ok(prefix) = suffix.parse::<u8>() {
                if prefix > 32 {
                    return Err(format!("Prefix length {prefix} is out of range (0-32)"));
                }
                Ok(NetworkInput::SubnetWithPrefix(ip, prefix))
            } else if let Ok(mask) = suffix.parse::<Ipv4Addr>() {
                Ok(NetworkInput::SubnetWithMask(ip, mask))
            } else {
                Err(format!(
                    "Invalid suffix '{suffix}': expected a prefix length (0-32) or a subnet mask (a.b.c.d)"
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
        NetworkInput::Host(ip) => {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_CLEAN);

            add_ipv4_address_info(&mut table, ip, args.binary);

            table.printstd();
        }
        NetworkInput::SubnetWithPrefix(ip, prefix) => {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_CLEAN);

            add_ipv4_address_info(&mut table, ip, args.binary);

            add_empty_row(&mut table);

            let network = ipv4network::Ipv4Network::new_from_prefix(ip, prefix);
            add_ipv4_network_info(&mut table, network, args.binary);

            if (args.split.is_some()) && (args.split.unwrap() > prefix) {
                add_empty_row(&mut table);
                add_ipv4_network_split_info(&mut table, network, args.split.unwrap());
            }

            table.printstd();
        }
        NetworkInput::SubnetWithMask(ip, subnetmask) => {
            // let network = ipv4network::Ipv4Network::new_from_mask(ip, subnetmask);
        }
    }
}

fn add_empty_row(table: &mut Table) {
    table.add_row(row![]);
}

fn add_ipv4_address_info(table: &mut Table, ip: Ipv4Addr, show_binary: bool) {
    table.add_row(row![
        "Address",
        ip,
        if show_binary {
            ipv4address::format_ipv4_to_binary(ip)
        } else {
            String::new()
        }
    ]);
    table.add_row(row!["Address class", ipv4address::address_class(ip)]);
    table.add_row(row!["Address space", ipv4address::address_space(ip)]);
}

fn add_ipv4_network_info(table: &mut Table, network: ipv4network::Ipv4Network, show_binary: bool) {
    table.add_row(row![
        "Network address",
        network.network(),
        if show_binary {
            ipv4address::format_ipv4_to_binary(network.network())
        } else {
            String::new()
        }
    ]);

    table.add_row(row![
        "Subnetmask",
        network.subnetmask(),
        if show_binary {
            ipv4address::format_ipv4_to_binary(network.subnetmask())
        } else {
            String::new()
        }
    ]);

    table.add_row(row![
        "First host address",
        network
            .first_host()
            .unwrap_or_else(|| "N/A".parse().unwrap()),
        if show_binary {
            ipv4address::format_ipv4_to_binary(
                network
                    .first_host()
                    .unwrap_or_else(|| "N/A".parse().unwrap()),
            )
        } else {
            String::new()
        }
    ]);
    table.add_row(row!["", "...", if show_binary { "..." } else { "" }]);
    table.add_row(row![
        "Last host address",
        network
            .last_host()
            .unwrap_or_else(|| "N/A".parse().unwrap()),
        if show_binary {
            ipv4address::format_ipv4_to_binary(
                network
                    .last_host()
                    .unwrap_or_else(|| "N/A".parse().unwrap()),
            )
        } else {
            String::new()
        }
    ]);
    table.add_row(row![
        "Broadcast address",
        network.broadcast(),
        if show_binary {
            ipv4address::format_ipv4_to_binary(network.broadcast())
        } else {
            String::new()
        }
    ]);
    table.add_row(row!["Host count", network.host_count()]);
}

fn add_ipv4_network_split_info(
    table: &mut Table,
    network: ipv4network::Ipv4Network,
    split_prefix: u8,
) {
    let subnets = network.split(split_prefix);
    for (i, subnet) in subnets.iter().enumerate() {
        table.add_row(row![
            format!("Subnet {}", i + 1),
            format!("{}/{}", subnet.network(), subnet.prefix())
        ]);
    }
}
