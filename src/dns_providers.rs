#[derive(Default)]
pub struct Gandi {
    pub apikey: Option<String>,
    pub domain: Option<String>,
    pub rrset_name: Option<String>,
    pub rrset_type: Option<String>,
    pub rrset_ttl: Option<String>,
}