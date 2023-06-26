use reqwest;

use crate::dns_providers::{Gandi,
};

pub async fn update_record_gandi(record_fc: Gandi, ipv: String) -> Result<String, reqwest::Error> {
    
    
    let client = reqwest::Client::new();
    //let plain_link = "https://api.gandi.net/v5/livedns/domains/{domain}/records/{rrset_name}/{rrset_type}";
    //let apikey = gandi_apikey(record_fc);
    //let domain = gandi_domain(record_fc);
    //let rrset_name = gandi_rrset_name(record_fc);
    //let rrset_type = gandi_rrset_type(record_fc);
    // url for api
    let get_url = format!("https://api.gandi.net/v5/livedns/domains/{domain}/records/{rrset_name}/{rrset_type}",
        domain=record_fc.domain(),
        rrset_name=record_fc.rrset_name(),
        rrset_type=record_fc.rrset_type());
    // data payload
    let payload = format!("{{\"rrset_values\":[\"{ipadress}\"],\"rrset_ttl\":{rrset_ttl}}}", ipadress=ipv, rrset_ttl=record_fc.rrset_ttl());

    // create and send the put request
    let body = client.put(get_url)
        .header("authorization",format!("Apikey {apikey}", apikey=record_fc.apikey()))
        .header("content-type", "application/json")
        .body(payload)
        .send()
        .await?
        .status();
    println!("{:?}", body);
    //cur_value = current_status.json()
    //try:
    //    cur_address = cur_value['rrset_values']
    //except KeyError:
    //    cur_address = ["127.0.0.1"]
    println!("return value {:?}",body);
    Ok(body.to_string())
}