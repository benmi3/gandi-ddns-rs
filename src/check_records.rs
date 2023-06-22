use std::fmt::format;

use reqwest;

use crate::dns_providers::{Gandi,
    gandi_apikey,
    gandi_domain,
    gandi_rrset_name,
    gandi_rrset_type
};


async fn check_record_gandi(record_fc: Gandi) -> bool {
    let client = reqwest::Client::new();
    //let plain_link = "https://api.gandi.net/v5/livedns/domains/{domain}/records/{rrset_name}/{rrset_type}";
    //let apikey = gandi_apikey(record_fc);
    //let domain = gandi_domain(record_fc);
    //let rrset_name = gandi_rrset_name(record_fc);
    //let rrset_type = gandi_rrset_type(record_fc);
    let get_url = format!("https://api.gandi.net/v5/livedns/domains/{domain}/records/{rrset_name}/{rrset_type}",
        domain=gandi_domain(record_fc),
        rrset_name=gandi_rrset_name(record_fc),
        rrset_type=gandi_rrset_type(record_fc));
    println!("{:?}",get_url);

    let body = client.get("https://api.gandi.net/v5/livedns/domains/{domain}/records/{rrset_name}/{rrset_type}")
        .header("authorization",format!("Apikey {apikey}", apikey=gandi_apikey(record_fc)))
        .send()
        .await?
        .text()
        .await?;

    //cur_value = current_status.json()
    //try:
    //    cur_address = cur_value['rrset_values']
    //except KeyError:
    //    cur_address = ["127.0.0.1"]

    true
}