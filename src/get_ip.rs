use std::net::{Ipv4Addr,Ipv6Addr};

struct IpAdressSet {
    ipv4_status: bool,
    ipv4_adress: Ipv4Addr,
    ipv6_status: bool,
    ipv6_adress: Ipv6Addr,
    
}

pub async fn get_ipadress() -> IpAdressSet {
    // Attempt to get an IP address and print it.
    if let Some(ipv4) = public_ip::addr_v4().await {
        println!("public ip address: {:?}", &ipv4);
        let ipv4_st = true;
    } else {
        println!("couldn't get an IP address");
        let ipv4_st = false;
    }
    if let Some(ipv6) = public_ip::addr_v6().await {
        println!("public ip address: {:?}", &ipv6);
        let ipv6_st = true;
    } else {
        println!("couldn't get an IP address");
        let ipv6_st = false;
    }
    IpAdressSet {
        ipv4_adress: ipv4,
        ipv4_status: ipv4_st,
        ipv6_adress: ipv6,
        ipv6_status: ipv6_st,
    }
    
}