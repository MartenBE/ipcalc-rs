use std::fmt;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Copy)]
pub struct Ipv4Network {
    address: Ipv4Addr,
    prefix: u8,
}

impl Ipv4Network {
    pub fn new(address: Ipv4Addr, prefix: u8) -> Self {
        assert!(prefix <= 32, "prefix must be between 0 and 32");

        Self { address, prefix }
    }

    pub fn address(&self) -> Ipv4Addr {
        self.address
    }

    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    pub fn mask(&self) -> Ipv4Addr {
        Ipv4Addr::from(mask_from_prefix(self.prefix))
    }

    pub fn network(&self) -> Ipv4Addr {
        let address = u32::from(self.address);
        let mask = mask_from_prefix(self.prefix);

        Ipv4Addr::from(address & mask)
    }

    pub fn broadcast(&self) -> Ipv4Addr {
        let network = u32::from(self.network());
        let mask = mask_from_prefix(self.prefix);

        Ipv4Addr::from(network | !mask)
    }

    pub fn first_host(&self) -> Option<Ipv4Addr> {
        if self.prefix >= 31 {
            return None;
        }

        Some(Ipv4Addr::from(u32::from(self.network()) + 1))
    }

    pub fn last_host(&self) -> Option<Ipv4Addr> {
        if self.prefix >= 31 {
            return None;
        }

        Some(Ipv4Addr::from(u32::from(self.broadcast()) - 1))
    }

    pub fn host_count(&self) -> u32 {
        if self.prefix >= 31 {
            0
        } else {
            (1u32 << (32 - self.prefix)) - 2
        }
    }

    pub fn split(&self, split_prefix: u8) -> Vec<Ipv4Network> {
        assert!(
            split_prefix >= self.prefix,
            "split prefix must be greater than or equal to original prefix"
        );
        assert!(split_prefix <= 32, "split prefix must be between 0 and 32");

        let subnet_count = 1u32 << (split_prefix - self.prefix);
        let subnet_size = 1u32 << (32 - split_prefix);
        let start = u32::from(self.network());

        let mut subnets = Vec::new();
        for i in 0..subnet_count {
            subnets.push(Ipv4Network::new(Ipv4Addr::from(start + i * subnet_size), split_prefix));
        }
        subnets
    }
}

fn mask_from_prefix(prefix: u8) -> u32 {
    if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    }
}
