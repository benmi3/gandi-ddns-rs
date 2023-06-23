use ini::Properties;

use crate::dns_providers::Gandi;

pub fn record_parser_gandi(record: Properties) -> Gandi {
    let apikey = record.get("apikey");
    let domain = record.get("domain");
    let rrset_name = record.get("rrset_name");
    let rrset_type = record.get("rrset_type");
    let rrset_ttl = record.get("rrset_ttl");

    let new_gandi = Gandi{
        apikey: apikey.to_owned(),
        domain: domain,
        rrset_name: rrset_name,
        rrset_type: rrset_type,
        rrset_ttl: rrset_ttl.parse(),
    }
}