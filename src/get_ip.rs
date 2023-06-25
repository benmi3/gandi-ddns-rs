use std::net::{Ipv4Addr,Ipv6Addr};

#[derive(Debug, Clone)]
pub struct IpAdressSet {
    ipv4_status: bool,
    ipv4_adress: Ipv4Addr,
    ipv6_status: bool,
    ipv6_adress: Ipv6Addr,
    
}

impl IpAdressSet {
    pub fn get_ipv4(&self) -> String {
        if self.ipv4_status {
            self.ipv4_adress.clone().to_string()
        } else {
            String::from("localhost")
        }
        
    }

    pub fn get_ipv6(&self) -> String {
        if self.ipv4_status {
            self.ipv6_adress.clone().to_string()
        } else {
            String::from("localhost")
        }
        
    }
}

pub async fn get_ipadress() -> IpAdressSet {
    // Attempt to get an IP address and print it.
    let ipv4 = public_ip::addr_v4().await;
    println!("public ip address: {:?}", &ipv4);
    let ipv6 = public_ip::addr_v6().await;
    println!("public ip address: {:?}", &ipv6);
    let ipv4_st = if Some(&ipv4).is_some() {
        true
    } else {
        false
    };
    let ipv6_st = if Some(&ipv6).is_some() {
        true
    } else {
        false
    };
    IpAdressSet {
        ipv4_adress: ipv4.unwrap(),
        ipv4_status: ipv4_st,
        ipv6_adress: ipv6.unwrap(),
        ipv6_status: ipv6_st,
    }
    
}