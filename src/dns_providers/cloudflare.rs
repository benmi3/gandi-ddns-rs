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
    zone_id: String,
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

    pub fn name(&self) -> String{
        self.name.clone()
    }

    pub fn rrset_ttl(&self) -> String{
        self.ttl.clone()
    }

    pub fn domain_type(&self) -> String{
        self.domain_type.clone()
    }

    pub fn zone_id(&self) -> String{
        self.zone_id.clone()
    }
}