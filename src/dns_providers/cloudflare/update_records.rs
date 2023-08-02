use reqwest;

use crate::dns_providers::cloudflare::Cloudflare;

pub async fn update_record_gandi(record_fc: Cloudflare, ipv: String) -> Result<String, reqwest::Error> {
    
    let client = reqwest::Client::new();

    // url for api
    let get_url = format!("https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns_records/{identifier}",
        identifier=record_fc.domain(),
        zone_identifier=record_fc.zone_id());
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