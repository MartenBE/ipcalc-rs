use std::net::Ipv4Addr;

pub fn address_class(address: Ipv4Addr) -> &'static str {
    match address.octets()[0] {
        0..=127 => "Class A",
        128..=191 => "Class B",
        192..=223 => "Class C",
        224..=239 => "Class D",
        240..=255 => "Class E",
    }
}

pub fn address_space(address: Ipv4Addr) -> &'static str {
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

pub fn format_ipv4_to_binary(address: Ipv4Addr) -> String {
    let [a, b, c, d] = address.octets();
    format!("{a:08b}.{b:08b}.{c:08b}.{d:08b}")
}
