#[derive(Clone, Copy)]
pub enum Ipv4Format {
    Decimal,
    Binary,
}

pub fn format_ipv4(address: Ipv4Addr, format: Ipv4Format) -> String {
    match format {
        Ipv4Format::Decimal => address.to_string(),
        Ipv4Format::Binary => {
            let [a, b, c, d] = address.octets();
            format!("{a:08b}.{b:08b}.{c:08b}.{d:08b}")
        }
    }
}
