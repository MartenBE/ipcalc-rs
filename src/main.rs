use clap::Parser;
use std::net::Ipv4Addr;

mod ipv4;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    ip_address: String,

    #[arg(long)]
    all_info: bool,

    #[arg(help = "Show hostname determined via DNS.", long, short)]
    hostname: bool,

    #[arg(
        action = clap::ArgAction::HelpLong,
        help = "Show this help message.",
        long,
        short = '?',
    )]
    help: Option<bool>,

    #[arg(
        help = "Print information on the provided IP address (default).",
        long,
        short
    )]
    info: bool,

    #[arg(help = "Print network in a the reverse DNS format.", long)]
    reverse_dns: bool,
    //
    // #[arg(
    //     help = "Split the provided network using the provided prefix/netmask.",
    //     long,
    //     required = false,
    //     short = 'S',
    //     value_name = "PREFIX"
    // )]
    // split: Option<String>,
}

fn main() {
    let args = Args::parse();

    let ip_address = args.ip_address.parse::<Ipv4Addr>().unwrap_or_else(|_| {
        eprintln!("Bad IPv4 address: {}", args.ip_address);
        std::process::exit(1);
    });

    if args.all_info {
        ipv4::print_all_info(ip_address.into());
    } else if args.hostname {
        ipv4::print_hostname(ip_address.into());
    } else if args.reverse_dns {
        ipv4::print_reverse_dns(ip_address.into());
    } else {
        // args.info
        ipv4::print_info(ip_address.into());
    }
}
