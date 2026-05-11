pub fn print_info(address: Ipv4Addr, format: Ipv4Format) {
    println!("Address: {}", format_ipv4(address, format));
    println!("Address space: {}", address_space(address));
    println!("Address class: {}", address_class(address));
}

pub fn print_network_info(network: Ipv4Network, format: Ipv4Format) {
    println!(
        "Network: {}/{}",
        format_ipv4(network.network(), format),
        network.prefix()
    );
    println!("Subnetmask: {}", format_ipv4(network.mask(), format));

    match network.first_host() {
        Some(host) => println!("Min host: {}", format_ipv4(host, format)),
        None => println!("Min host: N/A"),
    }

    match network.last_host() {
        Some(host) => println!("Max host: {}", format_ipv4(host, format)),
        None => println!("Max host: N/A"),
    }

    println!("Broadcast: {}", format_ipv4(network.broadcast(), format));
    println!("Amount hosts: {}", network.host_count());
}

pub fn print_subnet_splits(network: Ipv4Network, split_prefix: u8, format: Ipv4Format) {
    let subnets = network.split(split_prefix);

    println!("Amount subnet splits: {}", subnets.len());

    for (i, subnet) in subnets.iter().enumerate() {
        println!(
            "Subnet split {}: {}/{}",
            i + 1,
            format_ipv4(subnet.network(), format),
            subnet.prefix()
        );
    }
}
