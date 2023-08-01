// check current record
pub mod check_records;
// parse data into readable format
pub mod record_parser;
// update record
pub mod update_records;

#[derive(Default, Clone)]
pub struct Cloudflare {
    email: String,
    apikey: String,
    domain: String,
    name: String,
    domain_type: String,
    ttl: String,
}

impl Cloudflare {
    pub fn new(
        email: String,
        apikey: String,
        domain: String,
        name: String,
        domain_type: String,
        ttl: String
    ) -> Cloudflare {
        Cloudflare {
            email: email,
            apikey: apikey,
            domain: domain,
            name: name,
            ttl: ttl,
            domain_type: domain_type
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