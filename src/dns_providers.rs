pub struct Gandi {
    apikey: String,
    domain: String,
    rrset_name: String,
    rrset_type: String,
    rrset_ttl: u64,
}


pub fn gandi_apikey(gandi_get: Gandi) -> String {
    gandi_get.apikey
}

pub fn gandi_domain(gandi_get: Gandi) -> String {
    gandi_get.domain
}

pub fn gandi_rrset_name(gandi_get: Gandi) -> String {
    gandi_get.rrset_name
}

pub fn gandi_rrset_type(gandi_get: Gandi) -> String {
    gandi_get.rrset_type
}

pub fn gandi_rrset_ttl(gandi_get: Gandi) -> u64 {
    gandi_get.rrset_ttl
}
