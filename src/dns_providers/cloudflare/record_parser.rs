use ini::Properties;

use crate::dns_providers::cloudflare::Cloudflare;

pub fn record_parser_gandi(record: Properties) -> Cloudflare {
    let apikey = record.get("apikey").unwrap_or_default().to_string();
    let domain = record.get("domain").unwrap_or_default().to_string();
    let rrset_name = record.get("rrset_name").unwrap_or_default().to_string();
    let rrset_type = record.get("rrset_type").unwrap_or_default().to_string();
    let rrset_ttl = record.get("rrset_ttl").unwrap_or_default().to_string();
    Gandi::new(apikey, domain, rrset_name, rrset_type, rrset_ttl)
}