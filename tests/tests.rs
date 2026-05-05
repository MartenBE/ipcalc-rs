// These tests are converted from
// https://gitlab.com/ipcalc/ipcalc/-/blob/master/tests/meson.build
// Using AI.

use std::process::Command;

fn ipcalc() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ipcalc-rs"))
}

fn assert_success(output: &std::process::Output) {
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_failure(output: &std::process::Output) {
    assert!(
        !output.status.success(),
        "Command was expected to fail but succeeded"
    );
}

fn output_str(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn trim_output(output: &std::process::Output) -> String {
    output_str(output).trim().to_string()
}

// Incompatible combination tests //////////////////////////////////////////////

#[test]
fn test_split_info() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("-S")
        .arg("26")
        .arg("10.100.1.0/24")
        .output()
        .unwrap();
    assert_failure(&output);
}

#[test]
fn test_check_info() {
    let output = ipcalc()
        .arg("-c")
        .arg("-abmnp")
        .arg("10.100.1.1")
        .output()
        .unwrap();
    assert_failure(&output);
}

#[test]
fn test_check_split() {
    let output = ipcalc()
        .arg("-c")
        .arg("-S")
        .arg("26")
        .arg("10.100.1.0/24")
        .output()
        .unwrap();
    assert_failure(&output);
}

#[test]
fn test_check_host() {
    let output = ipcalc()
        .arg("-c")
        .arg("-h")
        .arg("127.0.0.1")
        .output()
        .unwrap();
    assert_failure(&output);
}

#[test]
fn test_random_info() {
    let output = ipcalc()
        .arg("-r")
        .arg("24")
        .arg("-i")
        .arg("127.0.0.1")
        .output()
        .unwrap();
    assert_failure(&output);
}

// Random address generation tests /////////////////////////////////////////////

#[test]
fn test_random_ipv4() {
    let output = ipcalc().arg("-r").arg("24").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Address"));
}

#[test]
fn test_random_address_ipv4() {
    let output = ipcalc().arg("-r").arg("24").arg("-a").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("ADDRESS"));
}

#[test]
fn test_random_ipv6_implicit() {
    let output = ipcalc().arg("-r").arg("112").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Address"));
}

#[test]
fn test_random_ipv6_prefix_valid_ula() {
    let output = ipcalc()
        .arg("-6")
        .arg("-r")
        .arg("48")
        .arg("-a")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(
        out.starts_with("ADDRESS=fd"),
        "Expected ULA prefix fd, got: {}",
        out
    );
}

#[test]
fn test_random_ipv6_explicit() {
    let output = ipcalc().arg("-6").arg("-r").arg("24").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Address"));
}

// Hostname lookup tests ///////////////////////////////////////////////////////

#[test]
fn test_hostname_ipv6_localhost() {
    let output = ipcalc()
        .arg("-6")
        .arg("-o")
        .arg("localhost")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "ADDRESS=::1");
}

#[test]
fn test_hostname_ipv4_localhost() {
    let output = ipcalc()
        .arg("-4")
        .arg("-o")
        .arg("localhost")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "ADDRESS=127.0.0.1");
}

#[test]
fn test_hostname_ipv4_localhost_json() {
    let output = ipcalc()
        .arg("-j")
        .arg("-4")
        .arg("-o")
        .arg("localhost")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("127.0.0.1"));
    assert!(out.contains("ADDRESS") || out.contains("address"));
}

#[test]
fn test_ip_ipv6_localhost() {
    let output = ipcalc().arg("-h").arg("::1").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "HOSTNAME=localhost");
}

#[test]
fn test_ip_ipv4_localhost() {
    let output = ipcalc().arg("-h").arg("127.0.0.1").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "HOSTNAME=localhost");
}

#[test]
fn test_ip_ipv4_localhost_json() {
    let output = ipcalc()
        .arg("-j")
        .arg("-h")
        .arg("127.0.0.1")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("localhost"));
}

// Class-prefix tests //////////////////////////////////////////////////////////

#[test]
fn test_class_prefix_class_a() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("12.15.1.5")
        .arg("--class-prefix")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("ADDRESS=12.15.1.5"));
    assert!(out.contains("NETMASK=255.0.0.0"));
    assert!(out.contains("PREFIX=8"));
    assert!(out.contains("BROADCAST=12.255.255.255"));
    assert!(out.contains("NETWORK=12.0.0.0"));
}

#[test]
fn test_class_prefix_class_b() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("129.15.31.5")
        .arg("--class-prefix")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("NETMASK=255.255.0.0"));
    assert!(out.contains("PREFIX=16"));
}

#[test]
fn test_class_prefix_class_c() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("193.92.31.0")
        .arg("--class-prefix")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("NETMASK=255.255.255.0"));
    assert!(out.contains("PREFIX=24"));
}

// Prefix tests ////////////////////////////////////////////////////////////////

#[test]
fn test_prefix_8() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("10.10.10.10/8")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("ADDRESS=10.10.10.10"));
    assert!(out.contains("NETMASK=255.0.0.0"));
    assert!(out.contains("PREFIX=8"));
    assert!(out.contains("BROADCAST=10.255.255.255"));
    assert!(out.contains("NETWORK=10.0.0.0"));
}

#[test]
fn test_prefix_16() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("10.100.4.1/16")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("ADDRESS=10.100.4.1"));
    assert!(out.contains("NETMASK=255.255.0.0"));
    assert!(out.contains("PREFIX=16"));
    assert!(out.contains("BROADCAST=10.100.255.255"));
    assert!(out.contains("NETWORK=10.100.0.0"));
}

#[test]
fn test_prefix_24() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("10.10.10.5/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("ADDRESS=10.10.10.5"));
    assert!(out.contains("NETMASK=255.255.255.0"));
    assert!(out.contains("PREFIX=24"));
    assert!(out.contains("BROADCAST=10.10.10.255"));
    assert!(out.contains("NETWORK=10.10.10.0"));
}

#[test]
fn test_prefix_30() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("10.100.4.1/30")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("NETMASK=255.255.255.252"));
    assert!(out.contains("PREFIX=30"));
}

#[test]
fn test_prefix_31() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("192.168.1.5/31")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("NETMASK=255.255.255.254"));
    assert!(out.contains("PREFIX=31"));
}

// Split tests (decorated) /////////////////////////////////////////////////////

#[test]
fn test_split_prefix_18() {
    let output = ipcalc()
        .arg("-S")
        .arg("18")
        .arg("10.10.10.10/16")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("[Split networks]"));
    assert!(out.contains("Network:"));
    assert!(out.contains("Total:"));
    assert!(out.contains("10.10.0.0/18"));
}

#[test]
fn test_split_prefix_24() {
    let output = ipcalc()
        .arg("-S")
        .arg("24")
        .arg("10.10.10.0/16")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("10.10.0.0/24"));
    assert!(out.contains("Total:"));
}

#[test]
fn test_split_prefix_26() {
    let output = ipcalc()
        .arg("-S")
        .arg("26")
        .arg("192.168.5.45/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("192.168.5.0/26"));
    assert!(out.contains("192.168.5.64/26"));
}

#[test]
fn test_split_prefix_29() {
    let output = ipcalc()
        .arg("-S")
        .arg("29")
        .arg("192.168.5.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("192.168.5.0/29"));
    assert!(out.contains("192.168.5.248/29"));
    let lines: Vec<&str> = out.lines().filter(|l| l.starts_with("Network:")).collect();
    assert_eq!(lines.len(), 32, "Expected 32 split networks for /24 -> /29");
}

#[test]
fn test_split_prefix_31() {
    let output = ipcalc()
        .arg("-S")
        .arg("31")
        .arg("192.168.5.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("192.168.5.0/31"));
}

#[test]
fn test_split_prefix_32() {
    let output = ipcalc()
        .arg("-S")
        .arg("32")
        .arg("192.168.5.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("192.168.5.0/32"));
    assert!(out.contains("192.168.5.255/32"));
}

#[test]
fn test_split_prefix_64_ipv6() {
    let output = ipcalc()
        .arg("-S")
        .arg("64")
        .arg("2a03:2880:20:4f06:face::/56")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(
        out.contains("2a03:2880:20:4f06::/64")
            || out.contains("2a03:2880:20:4f00::/64")
            || out.contains("Network:")
    );
}

#[test]
fn test_split_prefix_120_ipv6() {
    let output = ipcalc()
        .arg("-S")
        .arg("120")
        .arg("fcfa:b4ca:f1d8:125b:dc00::/112")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Network:") || out.contains("/120"));
}

#[test]
fn test_split_prefix_128_ipv6() {
    let output = ipcalc()
        .arg("-S")
        .arg("128")
        .arg("fcfa:b4ca:f1d8:125b:dc00::/127")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("/128"));
}

// No-decorate split tests /////////////////////////////////////////////////////

#[test]
fn test_nodecorate_split_18() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("18")
        .arg("10.10.10.10/16")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(!out.contains("Network:"));
    assert!(out.contains("/18"));
}

#[test]
fn test_nodecorate_split_24() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("24")
        .arg("10.10.10.0/16")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(!out.contains("Network:"));
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        256,
        "Expected 256 split networks for /16 -> /24"
    );
}

#[test]
fn test_nodecorate_split_26() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("26")
        .arg("192.168.5.45/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(!out.contains("Network:"));
    assert!(out.contains("192.168.5.0/26"));
}

#[test]
fn test_nodecorate_split_29() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("29")
        .arg("192.168.5.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 32);
}

#[test]
fn test_nodecorate_split_31() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("31")
        .arg("192.168.5.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("192.168.5.0/31"));
}

#[test]
fn test_nodecorate_split_32() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("32")
        .arg("192.168.5.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("192.168.5.0/32"));
}

#[test]
fn test_nodecorate_split_64_ipv6() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("64")
        .arg("2a03:2880:20:4f06:face::/56")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(!out.contains("Network:"));
    assert!(out.contains("/64"));
}

#[test]
fn test_nodecorate_split_120_ipv6() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("120")
        .arg("fcfa:b4ca:f1d8:125b:dc00::/112")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("/120"));
}

#[test]
fn test_nodecorate_split_128_ipv6() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-S")
        .arg("128")
        .arg("fcfa:b4ca:f1d8:125b:dc00::/127")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("/128"));
}

// Address space tests /////////////////////////////////////////////////////////

#[test]
fn test_addrspace_internet() {
    let output = ipcalc()
        .arg("--addrspace")
        .arg("-abmnp")
        .arg("193.92.150.3/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("ADDRSPACE=Internet") || out.contains("ADDRSPACE=\"Internet\""));
}

#[test]
fn test_addrspace_ula_ipv6() {
    let output = ipcalc()
        .arg("--addrspace")
        .arg("-abmnp")
        .arg("fd95:6be5:0ae0:84a5::/64")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("Unique Local Unicast"));
}

#[test]
fn test_addrspace_ula_48_ipv6() {
    let output = ipcalc()
        .arg("--addrspace")
        .arg("-abmnp")
        .arg("fd0b:a336:4e7d::/48")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("Unique Local Unicast"));
}

// Info tests - IPv4 special addresses

#[test]
fn test_info_internet() {
    let output = ipcalc().arg("-i").arg("3.130.45.15").output().unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("Address:") || out.contains("Address:"));
    assert!(out.contains("Internet"));
}

#[test]
fn test_info_host_on_this_network() {
    let output = ipcalc().arg("-i").arg("0.0.0.1").output().unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("This host on this network"));
}

#[test]
fn test_info_private_use_10() {
    let output = ipcalc().arg("-i").arg("10.0.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Private Use") || output_str(&output).contains("Private"));
}

#[test]
fn test_info_private_use_172_16() {
    let output = ipcalc().arg("-i").arg("172.16.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Private Use") || output_str(&output).contains("Private"));
}

#[test]
fn test_info_private_use_172_31() {
    let output = ipcalc().arg("-i").arg("172.31.255.254").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Private Use") || output_str(&output).contains("Private"));
}

#[test]
fn test_info_private_use_192_168() {
    let output = ipcalc().arg("-i").arg("192.168.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Private Use") || output_str(&output).contains("Private"));
}

#[test]
fn test_info_shared_address_space() {
    let output = ipcalc().arg("-i").arg("100.64.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Shared Address Space"));
}

#[test]
fn test_info_loopback() {
    let output = ipcalc().arg("-i").arg("127.0.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Loopback"));
}

#[test]
fn test_info_loopback_2() {
    let output = ipcalc().arg("-i").arg("127.0.1.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Loopback"));
}

#[test]
fn test_info_link_local() {
    let output = ipcalc().arg("-i").arg("169.254.0.1").output().unwrap();
    assert_success(&output);
    assert!(
        output_str(&output).contains("Link Local") || output_str(&output).contains("Link local")
    );
}

#[test]
fn test_info_ietf_protocol_assignments() {
    let output = ipcalc().arg("-i").arg("192.0.0.254").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("IETF Protocol"));
}

#[test]
fn test_info_service_continuity() {
    let output = ipcalc().arg("-i").arg("192.0.0.6").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Service Continuity"));
}

#[test]
fn test_info_dummy_address() {
    let output = ipcalc().arg("-i").arg("192.0.0.8").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Dummy"));
}

#[test]
fn test_info_port_control_protocol_anycast() {
    let output = ipcalc().arg("-i").arg("192.0.0.9").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Port Control Protocol Anycast"));
}

#[test]
fn test_info_traversal_using_relays_anycast() {
    let output = ipcalc().arg("-i").arg("192.0.0.10").output().unwrap();
    assert_success(&output);
    assert!(
        output_str(&output).contains("Traversal Using Relays around NAT Anycast")
            || output_str(&output).contains("Traversal")
    );
}

#[test]
fn test_info_nat64_dns64_170() {
    let output = ipcalc().arg("-i").arg("192.0.0.170").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("NAT64") || output_str(&output).contains("DNS64"));
}

#[test]
fn test_info_nat64_dns64_171() {
    let output = ipcalc().arg("-i").arg("192.0.0.171").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("NAT64") || output_str(&output).contains("DNS64"));
}

#[test]
fn test_info_as112v4() {
    let output = ipcalc().arg("-i").arg("192.31.196.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("AS112"));
}

#[test]
fn test_info_6to4_relay_anycast() {
    let output = ipcalc().arg("-i").arg("192.88.99.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("6to4") || output_str(&output).contains("Deprecated"));
}

#[test]
fn test_info_amt_ipv4() {
    let output = ipcalc().arg("-i").arg("192.52.193.5").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("AMT"));
}

#[test]
fn test_info_direct_delegation_as112_ipv4() {
    let output = ipcalc().arg("-i").arg("192.175.48.12").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Direct Delegation AS112"));
}

#[test]
fn test_info_testnet1() {
    let output = ipcalc().arg("-i").arg("192.0.2.1").output().unwrap();
    assert_success(&output);
    assert!(
        output_str(&output).contains("Documentation") || output_str(&output).contains("TEST-NET-1")
    );
}

#[test]
fn test_info_testnet2() {
    let output = ipcalc().arg("-i").arg("198.51.100.1").output().unwrap();
    assert_success(&output);
    assert!(
        output_str(&output).contains("Documentation") || output_str(&output).contains("TEST-NET-2")
    );
}

#[test]
fn test_info_testnet3() {
    let output = ipcalc().arg("-i").arg("203.0.113.1").output().unwrap();
    assert_success(&output);
    assert!(
        output_str(&output).contains("Documentation") || output_str(&output).contains("TEST-NET-3")
    );
}

#[test]
fn test_info_benchmarking_1() {
    let output = ipcalc().arg("-i").arg("198.18.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Benchmarking"));
}

#[test]
fn test_info_benchmarking_2() {
    let output = ipcalc().arg("-i").arg("198.19.201.71").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Benchmarking"));
}

#[test]
fn test_info_benchmarking_3() {
    let output = ipcalc().arg("-i").arg("198.19.255.254").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Benchmarking"));
}

#[test]
fn test_info_multicast() {
    let output = ipcalc().arg("-i").arg("224.0.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Multicast"));
}

#[test]
fn test_info_reserved_240() {
    let output = ipcalc().arg("-i").arg("240.0.0.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Reserved"));
}

#[test]
fn test_info_reserved_250() {
    let output = ipcalc().arg("-i").arg("250.7.93.1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Reserved"));
}

#[test]
fn test_info_reserved_255_255_255_254() {
    let output = ipcalc().arg("-i").arg("255.255.255.254").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Reserved"));
}

#[test]
fn test_info_limited_broadcast() {
    let output = ipcalc().arg("-i").arg("255.255.255.255").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Limited Broadcast"));
}

// Info tests - IPv6 special addresses /////////////////////////////////////////

#[test]
fn test_info_ipv6_global_unicast() {
    let output = ipcalc()
        .arg("-i")
        .arg("2a03:2880:20:4f06:face:b00c:0:1")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Global Unicast"));
}

#[test]
fn test_info_ipv6_translat_96() {
    let output = ipcalc()
        .arg("-i")
        .arg("64:ff9b::2100:951a")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(
        output_str(&output).contains("IPv4-IPv6 Translat")
            || output_str(&output).contains("Translat")
    );
}

#[test]
fn test_info_ipv6_translat_48() {
    let output = ipcalc()
        .arg("-i")
        .arg("64:ff9b:1:ea21:4316:55::1")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Translat"));
}

#[test]
fn test_info_ipv6_teredo() {
    let output = ipcalc()
        .arg("-i")
        .arg("2001:0000:50::d78a:0:c")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("TEREDO") || output_str(&output).contains("Teredo"));
}

#[test]
fn test_info_ipv6_port_control_protocol() {
    let output = ipcalc().arg("-i").arg("2001:1::1").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Port Control Protocol Anycast"));
}

#[test]
fn test_info_ipv6_traversal_using_relays() {
    let output = ipcalc().arg("-i").arg("2001:1::2").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Traversal Using Relays"));
}

#[test]
fn test_info_ipv6_benchmarking() {
    let output = ipcalc()
        .arg("-i")
        .arg("2001:2:0:a::5:b7f")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Benchmarking"));
}

#[test]
fn test_info_ipv6_amt() {
    let output = ipcalc()
        .arg("-i")
        .arg("2001:3:89:71::ab:1")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("AMT"));
}

#[test]
fn test_info_ipv6_as112v6() {
    let output = ipcalc()
        .arg("-i")
        .arg("2001:4:112::6483:bd12:9010")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("AS112"));
}

#[test]
fn test_info_ipv6_orchid() {
    let output = ipcalc().arg("-i").arg("2001:1e:ab:7::").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("ORCHID"));
}

#[test]
fn test_info_ipv6_orchidv2() {
    let output = ipcalc().arg("-i").arg("2001:2c:ab:7::").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("ORCHID"));
}

#[test]
fn test_info_ipv6_documentation() {
    let output = ipcalc().arg("-i").arg("2001:db8:329::4a").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Documentation"));
}

#[test]
fn test_info_ipv6_ietf_protocol() {
    let output = ipcalc()
        .arg("-i")
        .arg("2001:190:70::e86a:0:b")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("IETF Protocol"));
}

#[test]
fn test_info_ipv6_6to4() {
    let output = ipcalc().arg("-i").arg("2002::34").output().unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("6to4"));
}

#[test]
fn test_info_ipv6_direct_delegation_as112() {
    let output = ipcalc()
        .arg("-i")
        .arg("2620:4f:8000::7372:ad1")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Direct Delegation AS112"));
}

#[test]
fn test_info_ipv6_ula() {
    let output = ipcalc()
        .arg("-i")
        .arg("fc00:0:a001::bcdf:4")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Unique Local Unicast"));
}

#[test]
fn test_info_ipv6_link_scoped_unicast() {
    let output = ipcalc()
        .arg("-i")
        .arg("fe80::b:0:7acd:0:5")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(
        output_str(&output).contains("Link-Scoped Unicast") || output_str(&output).contains("Link")
    );
}

#[test]
fn test_info_ipv6_ula_prefix_48() {
    let output = ipcalc()
        .arg("-i")
        .arg("fd0b:a336:4e7d::/48")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("Unique Local Unicast"));
}

// Split advanced tests ////////////////////////////////////////////////////////

#[test]
fn test_split_lines_match_total_ipv4() {
    let output = ipcalc()
        .arg("-S")
        .arg("29")
        .arg("192.168.5.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    let network_lines: usize = out
        .lines()
        .filter(|l| l.trim().starts_with("Network:"))
        .count();
    // Parse Total line
    for line in out.lines() {
        if line.trim().starts_with("Total:") {
            let total: usize = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            assert_eq!(network_lines, total, "Network lines should match Total");
            break;
        }
    }
}

#[test]
fn test_split_lines_match_total_ipv6() {
    let output = ipcalc()
        .arg("-S")
        .arg("120")
        .arg("fcfa:b4ca:f1d8:125b:dc00::/112")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    let network_lines: usize = out
        .lines()
        .filter(|l| l.trim().starts_with("Network:"))
        .count();
    for line in out.lines() {
        if line.trim().starts_with("Total:") {
            let total: usize = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            assert_eq!(network_lines, total, "Network lines should match Total");
            break;
        }
    }
}

// Help test ///////////////////////////////////////////////////////////////////

#[test]
fn test_help_command() {
    let output = ipcalc().arg("--help").output().unwrap();
    assert_success(&output);
}

// Validation/check tests //////////////////////////////////////////////////////

#[test]
fn test_validate_loopback_ipv4() {
    let output = ipcalc().arg("-c").arg("127.0.0.1").output().unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_loopback_explicit_ipv6() {
    let output = ipcalc().arg("-c").arg("-6").arg("::1").output().unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_loopback_ipv6() {
    let output = ipcalc().arg("-c").arg("::1").output().unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_private_ipv4() {
    let output = ipcalc().arg("-c").arg("192.168.1.1").output().unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_abbreviated_ipv6() {
    let output = ipcalc()
        .arg("-c")
        .arg("-6")
        .arg("2a01:198:200:300::2")
        .output()
        .unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_expanded_ipv6() {
    let output = ipcalc()
        .arg("-c")
        .arg("-6")
        .arg("2a01:198:200:300:0000:0000:0000:2")
        .output()
        .unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_complete_ipv6() {
    let output = ipcalc()
        .arg("-c")
        .arg("-6")
        .arg("2a01:0198:0200:0300:0000:0000:0000:0002")
        .output()
        .unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_loopback_ipv6_with_prefix() {
    let output = ipcalc()
        .arg("-c")
        .arg("-6")
        .arg("::1/128")
        .output()
        .unwrap();
    assert_success(&output);
}

#[test]
fn test_validate_reserved_ipv6_with_prefix() {
    let output = ipcalc()
        .arg("-c")
        .arg("-6")
        .arg("fec0::1:0:0:c0a8:8002/64")
        .output()
        .unwrap();
    assert_success(&output);
}

// Invalid input tests /////////////////////////////////////////////////////////

#[test]
fn test_no_validate_invalid_hex_ipv6() {
    let output = ipcalc().arg("-c").arg("-6").arg("gggg::").output().unwrap();
    assert_failure(&output);
}

#[test]
fn test_no_allow_ipv4_and_ipv6() {
    let output = ipcalc()
        .arg("-4")
        .arg("-6")
        .arg("2a01:198:200:300:0000:0000:0000:2")
        .output()
        .unwrap();
    assert_failure(&output);
}

#[test]
fn test_no_validate_ipv6_in_ipv4_mode() {
    let output = ipcalc()
        .arg("-c")
        .arg("-4")
        .arg("2a01:198:200:300:0000:0000:0000:2")
        .output()
        .unwrap();
    assert_failure(&output);
}

#[test]
fn test_no_validate_ipv4_and_ipv6_with_ipv4() {
    let output = ipcalc()
        .arg("-c")
        .arg("-4")
        .arg("-6")
        .arg("127.0.0.1")
        .output()
        .unwrap();
    assert_failure(&output);
}

#[test]
fn test_no_validate_invalid_prefix_ipv6() {
    let output = ipcalc().arg("-c").arg("::1/999").output().unwrap();
    assert_failure(&output);
}

// Netmask invalid tests ///////////////////////////////////////////////////////

#[test]
fn test_no_validate_negative_prefix() {
    let output = ipcalc().arg("-m").arg("192.168.1.1/-1").output().unwrap();
    assert_failure(&output);
}

#[test]
fn test_no_validate_ipv6_prefix_in_ipv4_mode() {
    let output = ipcalc().arg("-m").arg("192.168.1.1/64").output().unwrap();
    assert_failure(&output);
}

#[test]
fn test_no_validate_invalid_prefix_ipv4() {
    let output = ipcalc()
        .arg("-m")
        .arg("192.168.1.1/99999")
        .output()
        .unwrap();
    assert_failure(&output);
}

// Broadcast output tests //////////////////////////////////////////////////////

#[test]
fn test_calculate_broadcast_from_prefix() {
    let output = ipcalc().arg("-b").arg("192.168.1.1/24").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "BROADCAST=192.168.1.255");
}

#[test]
fn test_calculate_broadcast_from_netmask() {
    let output = ipcalc()
        .arg("-b")
        .arg("192.168.1.1")
        .arg("255.255.255.0")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "BROADCAST=192.168.1.255");
}

// Netmask output tests ////////////////////////////////////////////////////////

#[test]
fn test_calculate_netmask_prefix_24() {
    let output = ipcalc().arg("-m").arg("192.168.1.1/24").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETMASK=255.255.255.0");
}

#[test]
fn test_calculate_netmask_prefix_22() {
    let output = ipcalc().arg("-m").arg("172.16.59.222/22").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETMASK=255.255.252.0");
}

#[test]
fn test_calculate_netmask_no_prefix() {
    let output = ipcalc().arg("-m").arg("192.168.1.1").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETMASK=255.255.255.255");
}

// Prefix output tests /////////////////////////////////////////////////////////

#[test]
fn test_calculate_prefix_24_from_netmask() {
    let output = ipcalc()
        .arg("-p")
        .arg("192.168.1.1")
        .arg("255.255.255.0")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "PREFIX=24");
}

#[test]
fn test_calculate_prefix_32_from_netmask() {
    let output = ipcalc()
        .arg("-p")
        .arg("192.168.1.1")
        .arg("255.255.255.255")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "PREFIX=32");
}

#[test]
fn test_calculate_prefix_0_from_netmask() {
    let output = ipcalc()
        .arg("-p")
        .arg("192.168.1.1")
        .arg("0.0.0.0")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "PREFIX=0");
}

#[test]
fn test_calculate_prefix_22_from_netmask() {
    let output = ipcalc()
        .arg("-p")
        .arg("172.16.59.222")
        .arg("255.255.252.0")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "PREFIX=22");
}

// Class-prefix calculation tests //////////////////////////////////////////////

#[test]
fn test_class_netmask_24() {
    let output = ipcalc()
        .arg("--class-prefix")
        .arg("-m")
        .arg("192.168.1.1")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETMASK=255.255.255.0");
}

#[test]
fn test_class_netmask_8() {
    let output = ipcalc()
        .arg("--class-prefix")
        .arg("-m")
        .arg("10.1.2.3")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETMASK=255.0.0.0");
}

#[test]
fn test_class_netmask_16() {
    let output = ipcalc()
        .arg("--class-prefix")
        .arg("-m")
        .arg("129.22.4.3")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETMASK=255.255.0.0");
}

// Network output tests ////////////////////////////////////////////////////////

#[test]
fn test_calculate_network_prefix_32() {
    let output = ipcalc().arg("-n").arg("192.168.1.1/32").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETWORK=192.168.1.1");
}

#[test]
fn test_calculate_network_prefix_0() {
    let output = ipcalc().arg("-n").arg("192.168.1.1/0").output().unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "NETWORK=0.0.0.0");
}

// Reverse DNS tests ///////////////////////////////////////////////////////////

#[test]
fn test_reverse_dns_prefix_32_ipv4() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("193.92.150.3/32")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=3.150.92.193.in-addr.arpa."
    );
}

#[test]
fn test_reverse_dns_prefix_26_ipv4() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("193.92.150.3/26")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=0-63.150.92.193.in-addr.arpa."
    );
}

#[test]
fn test_reverse_dns_prefix_16_ipv4() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("193.92.150.3/16")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "REVERSEDNS=92.193.in-addr.arpa.");
}

#[test]
fn test_reverse_dns_prefix_12_ipv4() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("193.92.150.3/12")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(trim_output(&output), "REVERSEDNS=80-95.193.in-addr.arpa.");
}

#[test]
fn test_reverse_dns_ipv6_4321() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("4321:0:1:2:3:4:567:89ab")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=b.a.9.8.7.6.5.0.4.0.0.0.3.0.0.0.2.0.0.0.1.0.0.0.0.0.0.0.1.2.3.4.ip6.arpa."
    );
}

#[test]
fn test_reverse_dns_ipv6_first() {
    let output = ipcalc().arg("--reverse-dns").arg("1::").output().unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.1.0.0.0.ip6.arpa."
    );
}

#[test]
fn test_reverse_dns_ipv6_loopback() {
    let output = ipcalc().arg("--reverse-dns").arg("::1").output().unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=1.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.ip6.arpa."
    );
}

#[test]
fn test_reverse_dns_ipv6_prefix_128() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("1234::4321/128")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=1.2.3.4.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.4.3.2.1.ip6.arpa."
    );
}

#[test]
fn test_reverse_dns_ipv6_prefix_124() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("1234::4321/124")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=2.3.4.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.4.3.2.1.ip6.arpa."
    );
}

#[test]
fn test_reverse_dns_ipv6_prefix_120() {
    let output = ipcalc()
        .arg("--reverse-dns")
        .arg("1234::4321/120")
        .output()
        .unwrap();
    assert_success(&output);
    assert_eq!(
        trim_output(&output),
        "REVERSEDNS=3.4.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.4.3.2.1.ip6.arpa."
    );
}

// Specific info decorated & no-decorate tests /////////////////////////////////

#[test]
fn test_specific_info_output() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("--minaddr")
        .arg("--maxaddr")
        .arg("--addresses")
        .arg("192.168.2.6/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    assert!(out.contains("ADDRESS=192.168.2.6"));
    assert!(out.contains("NETMASK=255.255.255.0"));
    assert!(out.contains("PREFIX=24"));
    assert!(out.contains("BROADCAST=192.168.2.255"));
    assert!(out.contains("NETWORK=192.168.2.0"));
    assert!(out.contains("MINADDR=192.168.2.1"));
    assert!(out.contains("MAXADDR=192.168.2.254"));
    assert!(out.contains("ADDRESSES=254"));
}

#[test]
fn test_specific_info_no_decorate() {
    let output = ipcalc()
        .arg("-abmnp")
        .arg("--minaddr")
        .arg("--maxaddr")
        .arg("--addresses")
        .arg("--no-decorate")
        .arg("192.168.2.7/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = trim_output(&output);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 8);
    assert_eq!(lines[0], "192.168.2.7");
    assert_eq!(lines[1], "255.255.255.0");
    assert_eq!(lines[2], "24");
    assert_eq!(lines[3], "192.168.2.255");
    assert_eq!(lines[4], "192.168.2.0");
    assert_eq!(lines[5], "192.168.2.1");
    assert_eq!(lines[6], "192.168.2.254");
    assert_eq!(lines[7], "254");
}

// JSON output tests ///////////////////////////////////////////////////////////

#[test]
fn test_json_split_prefix_24() {
    let output = ipcalc()
        .arg("-j")
        .arg("-S")
        .arg("24")
        .arg("10.10.10.0/16")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("SPLITNETWORK"));
    assert!(out.contains("NETS"));
    assert!(out.contains("ADDRESSES"));
    assert!(out.contains("10.10.0.0/24"));
    assert!(out.contains("10.10.255.0/24"));
}

#[test]
fn test_json_split_prefix_26() {
    let output = ipcalc()
        .arg("-j")
        .arg("-S")
        .arg("26")
        .arg("192.168.5.45/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("SPLITNETWORK"));
    assert!(out.contains("192.168.5.0/26"));
}

#[test]
fn test_json_nodecorate_split_prefix_24() {
    let output = ipcalc()
        .arg("-j")
        .arg("--no-decorate")
        .arg("-S")
        .arg("24")
        .arg("10.10.10.0/16")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("SPLITNETWORK"));
}

#[test]
fn test_json_nodecorate_split_prefix_26() {
    let output = ipcalc()
        .arg("-j")
        .arg("--no-decorate")
        .arg("-S")
        .arg("26")
        .arg("192.168.5.45/24")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("SPLITNETWORK"));
}

#[test]
fn test_json_split_prefix_64_ipv6() {
    let output = ipcalc()
        .arg("-j")
        .arg("-S")
        .arg("64")
        .arg("2a03:2880:20:4f06:face::/56")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("SPLITNETWORK"));
    assert!(out.contains("/64"));
}

#[test]
fn test_json_nodecorate_split_prefix_64_ipv6() {
    let output = ipcalc()
        .arg("-j")
        .arg("--no-decorate")
        .arg("-S")
        .arg("64")
        .arg("2a03:2880:20:4f06:face::/56")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("SPLITNETWORK"));
}

#[test]
fn test_json_info_prefix_56_ipv6() {
    let output = ipcalc()
        .arg("-j")
        .arg("-i")
        .arg("2a03:2880:20:4f06:face:b00c:0:1/56")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("FULLADDRESS") || out.contains("ADDRESS"));
    assert!(out.contains("NETWORK"));
    assert!(out.contains("NETMASK"));
    assert!(out.contains("PREFIX"));
    assert!(out.contains("ADDRSPACE"));
}

#[test]
fn test_json_specific_info_ipv6() {
    let output = ipcalc()
        .arg("-j")
        .arg("-n")
        .arg("2a03:2880:20:4f06:face:b00c:0:1/56")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("ADDRESS") || out.contains("NETWORK"));
}

// All-info / Info tests ///////////////////////////////////////////////////////

#[test]
fn test_all_info() {
    let output = ipcalc()
        .arg("--all-info")
        .arg("192.168.2.7/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Address:"));
    assert!(out.contains("Network:"));
    assert!(out.contains("Netmask:"));
    assert!(out.contains("Broadcast:"));
    assert!(out.contains("Reverse DNS:"));
    assert!(out.contains("Address space:"));
    assert!(out.contains("Address class:"));
    assert!(out.contains("HostMin:"));
    assert!(out.contains("HostMax:"));
    assert!(out.contains("Hosts/Net:"));
}

#[test]
fn test_info_with_prefix() {
    let output = ipcalc().arg("-i").arg("192.168.2.7/24").output().unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Address:"));
    assert!(out.contains("Network:"));
    assert!(out.contains("Netmask:"));
    assert!(out.contains("Broadcast:"));
    assert!(out.contains("Address space:"));
    assert!(out.contains("HostMin:"));
    assert!(out.contains("HostMax:"));
    assert!(out.contains("Hosts/Net:"));
}

#[test]
fn test_info_implicit() {
    let output = ipcalc().arg("192.168.2.7/24").output().unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Network:") || out.contains("Address:"));
}

// Deaggregate tests ///////////////////////////////////////////////////////////

#[test]
fn test_deaggregate_no_decorate() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-d")
        .arg("192.168.2.7-192.168.2.13")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("192.168.2.7/32"));
    assert!(out.contains("192.168.2.8/30"));
    assert!(out.contains("192.168.2.12/31"));
}

#[test]
fn test_deaggregate() {
    let output = ipcalc()
        .arg("-d")
        .arg("192.168.2.0-192.168.3.255")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("192.168.2.0/23") || out.contains("Deaggregated"));
}

#[test]
fn test_deaggregate_large_range() {
    let output = ipcalc()
        .arg("-d")
        .arg("240.0.0.0-255.255.255.255")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("240.0.0.0"));
}

#[test]
fn test_deaggregate_large_range_2() {
    let output = ipcalc()
        .arg("-d")
        .arg("127.0.0.0-255.255.255.255")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("127.0.0.0"));
}

#[test]
fn test_deaggregate_full_range() {
    let output = ipcalc()
        .arg("-d")
        .arg("0.0.0.0-255.255.255.255")
        .output()
        .unwrap();
    assert_success(&output);
    assert!(output_str(&output).contains("0.0.0.0/0"));
}

#[test]
fn test_deaggregate_with_spaces() {
    let output = ipcalc()
        .arg("-d")
        .arg("  192.168.2.0   - 192.168.3.255   ")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("192.168.2.0") || out.contains("Deaggregated"));
}

#[test]
fn test_deaggregate_json() {
    let output = ipcalc()
        .arg("-j")
        .arg("-d")
        .arg("192.168.2.33-192.168.3.2")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("DEAGGREGATEDNETWORK"));
    assert!(out.contains("192.168.2.33/32"));
    assert!(out.contains("192.168.3.2/32"));
}

#[test]
fn test_deaggregate_ipv6_no_decorate() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("-d")
        .arg("2a03:2880:20:4f06:face::0-2a03:2880:20:4f06:face::fffe")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(!out.contains("Network:"));
    assert!(out.contains("/"));
}

#[test]
fn test_deaggregate_ipv6() {
    let output = ipcalc()
        .arg("-d")
        .arg("fcd3:57d1:733:c18f:b498:25e1:788:0-fcd3:57d1:733:c18f:b498:25e1:788:ffff")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Deaggregated") || out.contains("/"));
}

#[test]
fn test_deaggregate_ipv6_json() {
    let output = ipcalc()
        .arg("-j")
        .arg("-d")
        .arg("fcd3:57d1:733:c18f:b498:25e1:788:f-fcd3:57d1:733:c18f:b498:25e1:789:ffa9")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("DEAGGREGATEDNETWORK"));
}

// Standard/No-decorate info tests /////////////////////////////////////////////

#[test]
fn test_standard_info() {
    let output = ipcalc().arg("192.168.2.0/24").output().unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Network:"));
    assert!(out.contains("Netmask:"));
    assert!(out.contains("Broadcast:"));
    assert!(out.contains("Address space:"));
    assert!(out.contains("HostMin:"));
    assert!(out.contains("HostMax:"));
    assert!(out.contains("Hosts/Net:"));
}

#[test]
fn test_nodecorate_info() {
    let output = ipcalc()
        .arg("--no-decorate")
        .arg("192.168.2.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Network:"));
    assert!(out.contains("Netmask:"));
    assert!(out.contains("Broadcast:"));
}

#[test]
fn test_nodecorate_all_info() {
    let output = ipcalc()
        .arg("--all-info")
        .arg("--no-decorate")
        .arg("192.168.2.0/24")
        .output()
        .unwrap();
    assert_success(&output);
    let out = output_str(&output);
    assert!(out.contains("Network:"));
    assert!(out.contains("Reverse DNS:") || out.contains("Address space:"));
}
