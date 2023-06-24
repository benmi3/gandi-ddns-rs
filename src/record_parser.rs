use ini::Properties;

use crate::dns_providers::Gandi;

pub fn record_parser_gandi(record: Properties) -> Gandi {
    let apikey = record.get("apikey").unwrap_or_default().to_string();
    let domain = record.get("domain").unwrap_or_default().to_string();
    let rrset_name = record.get("rrset_name").unwrap_or_default().to_string();
    let rrset_type = record.get("rrset_type").unwrap_or_default().to_string();
    let rrset_ttl = record.get("rrset_ttl").unwrap_or_default().to_string();
    

    Gandi{
        apikey: apikey,
        domain: domain,
        rrset_name: rrset_name,
        rrset_type: rrset_type,
        rrset_ttl: rrset_ttl,
    }
}