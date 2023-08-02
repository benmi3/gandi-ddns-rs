use reqwest;

use crate::dns_providers::cloudflare::Cloudflare;

// unused code
#[allow(dead_code)]
pub async fn check_record_cloudfalre(record_fc: Cloudflare) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let get_url = format!("https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns_records/{identifier}",
        identifier=record_fc.domain(),
        zone_identifier=record_fc.zone_id());
    //println!("{:#?}",get_url);

    let body = client.get(get_url)
        .header("X-Auth-Email:",format!("{email}", email=record_fc.email()))
        .header("Content-Type: application/json")
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", body);
    //cur_value = current_status.json()
    //try:
    //    cur_address = cur_value['rrset_values']
    //except KeyError:
    //    cur_address = ["127.0.0.1"]
    Ok(body)
}