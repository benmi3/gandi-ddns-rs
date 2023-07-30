// check current record
pub mod check_records;
// parse data into readable format
pub mod record_parser;
// update record
pub mod update_records;

#[derive(Default, Clone)]
pub struct Gandi {
    apikey: String,
    domain: String,
    rrset_name: String,
    rrset_type: String,
    rrset_ttl: String,
}

impl Gandi {
    pub fn new(apikey: String, domain: String, rrset_name: String, rrset_type: String, rrset_ttl: String) -> Gandi {
        Gandi {
            apikey: apikey,
            domain: domain,
            rrset_name: rrset_name,
            rrset_ttl: rrset_ttl,
            rrset_type: rrset_type
        }
    }

    pub fn apikey(&self) -> String{
        self.apikey.clone()
    }

    pub fn domain(&self) -> String{
        self.domain.clone()
    }

    pub fn rrset_name(&self) -> String{
        self.rrset_name.clone()
    }

    pub fn rrset_ttl(&self) -> String{
        self.rrset_ttl.clone()
    }

    pub fn rrset_type(&self) -> String{
        self.rrset_type.clone()
    }
}