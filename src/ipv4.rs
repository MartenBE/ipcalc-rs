use dns_lookup::lookup_addr;
use std::net::Ipv4Addr;

const CLASS_A_START_ADDRESS: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const CLASS_A_END_ADDRES: Ipv4Addr = Ipv4Addr::new(127, 255, 255, 255);
const CLASS_B_START_ADDRESS: Ipv4Addr = Ipv4Addr::new(128, 0, 0, 0);
const CLASS_B_END_ADDRESS: Ipv4Addr = Ipv4Addr::new(191, 255, 255, 255);
const CLASS_C_START_ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 0, 0, 0);
const CLASS_C_END_ADDRESS: Ipv4Addr = Ipv4Addr::new(223, 255, 255, 255);
const CLASS_D_START_ADDRESS: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 0);
const CLASS_D_END_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 255);
const CLASS_E_START_ADDRESS: Ipv4Addr = Ipv4Addr::new(240, 0, 0, 0);
const CLASS_E_END_ADDRESS: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 255);

pub fn print_all_info(address: Ipv4Addr) {
    println!("Address:        {}", address);
    println!("Reverse DNS:    {}", get_reverse_dns(address));
    println!("Address space:  {}", get_address_space(address));
    println!("Address class:  {}", get_address_class(address));
}

pub fn print_info(address: Ipv4Addr) {
    println!("Address:        {}", address);
    println!("Address space:  {}", get_address_space(address));
}

pub fn print_hostname(address: Ipv4Addr) {
    println!("HOSTNAME={}", get_hostname(address));
}

pub fn print_reverse_dns(address: Ipv4Addr) {
    println!("REVERSE_DNS={}", get_reverse_dns(address));
}

////////////////////////////////////////////////////////////////////////////////

fn get_reverse_dns(address: Ipv4Addr) -> String {
    // TODO:
    // - Dot at the end?
    // - Conform to RFC?
    let octets = address.octets();
    format!(
        "{}.{}.{}.{}.in-addr.arpa.",
        octets[3], octets[2], octets[1], octets[0]
    )
}

fn get_hostname(address: Ipv4Addr) -> String {
    lookup_addr(&std::net::IpAddr::V4(address)).unwrap()
}

fn get_address_space(address: Ipv4Addr) -> &'static str {
    if address.is_broadcast() {
        "Broadcast"
    } else if address.is_documentation() {
        "Documentation"
    } else if address.is_link_local() {
        "Link local"
    } else if address.is_loopback() {
        "Loopback"
    } else if address.is_multicast() {
        "Multicast"
    } else if address.is_private() {
        "Private"
    } else if address.is_unspecified() {
        "Unspecified"
    } else {
        "Internet"
    }
}

fn get_address_class(ipv4_address: Ipv4Addr) -> &'static str {
    // TODO: Conform to RFC?
    if is_address_in_range(ipv4_address, CLASS_A_START_ADDRESS, CLASS_A_END_ADDRES) {
        "Class A"
    } else if is_address_in_range(ipv4_address, CLASS_B_START_ADDRESS, CLASS_B_END_ADDRESS) {
        "Class B"
    } else if is_address_in_range(ipv4_address, CLASS_C_START_ADDRESS, CLASS_C_END_ADDRESS) {
        "Class C"
    } else if is_address_in_range(ipv4_address, CLASS_D_START_ADDRESS, CLASS_D_END_ADDRESS) {
        "Class D"
    } else if is_address_in_range(ipv4_address, CLASS_E_START_ADDRESS, CLASS_E_END_ADDRESS) {
        "Class E"
    } else {
        "Unknown class"
    }
}

fn is_address_in_range(address: Ipv4Addr, start_address: Ipv4Addr, end_address: Ipv4Addr) -> bool {
    let address_u32 = u32::from(address);
    let start_address_u32 = u32::from(start_address);
    let end_address_u32 = u32::from(end_address);

    start_address_u32 <= address_u32 && address_u32 <= end_address_u32
}


