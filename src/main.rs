use std::process;
use std::path::Path;
use ini::{Ini, Properties};
//use check_records::check_record_gandi;
use get_ip::IpAdressSet;
use dns_providers::gandi::record_parser::record_parser_gandi;
use dns_providers::gandi::update_records::update_record_gandi;

const CONF_FILE_NAME: &str = "config.ini";

//mod check_records;
mod dns_providers;
mod get_ip;
//mod dns_providers::gandi;

//use log::{info,warn};

#[tokio::main]
async fn main() {
    // check if the config exists or not
    if Path::new(CONF_FILE_NAME).exists() {
        println!("File exists");
    }
    else{
        println!("File didnt exists");
        println!("Creating template file");
        // create a template file
        create_config();
        println!("Created the template");
        println!("Exiting program");
        // exiting program
        process::exit(1);
    };

    // get global ipadress
    let ip_set = get_ip::get_ipadress().await;
    println!("{:?}",ip_set);

    // load the file
    println!("Reading from file {:?}", CONF_FILE_NAME);
    let conf = Ini::load_from_file(CONF_FILE_NAME).unwrap();
    
    // create Vec of domains that needs update
    //let needs_updating: Vec<&str> = Vec::new();

    println!("Starting loop");
    let general_section_name = "__General__";
    for (sec, prop) in conf.iter() {
        // get section name
        let section_name = sec.as_ref().unwrap_or(&general_section_name);
        println!("Now handeling section: {:?}", section_name);
        let config_data = handle_config_data(prop.clone(), ip_set.clone()).await;
        
        if config_data {
            println!("handled the data succesfully")
        } else {
            println!("somthing happened, and the data handling was unsuccessfull")
        }
    }
    println!("Program finished")
}

fn create_config () -> bool {
    // start with conf
    let mut conf = Ini::new();
    // set encoding to UTF-8
    conf.with_section(None::<String>).set("encoding", "utf-8");
    conf.with_section(Some("Config1"))
        .set("dnsprovider", "Gandi")
        .set("email", "e@example.com")
        .set("apikey", "s3cr3t4p1k3y")
        .set("domain", "example.com")
        .set("rrset_name", "@")
        .set("rrset_type", "A")
        .set("rrset_ttl", "18000");
    conf.with_section(Some("Config2"))
        .set("dnsprovider", "Gandi")
        .set("email", "e@example.com")
        .set("apikey", "s3cr3t4p1k3y")
        .set("domain", "example.com")
        .set("rrset_name", "@")
        .set("rrset_type", "AAAA")
        .set("rrset_ttl", "18000");
    //write above template to file
    conf.write_to_file(CONF_FILE_NAME).unwrap();

        if Path::new(CONF_FILE_NAME).exists() {
            true
        }
        else {
            false
        }
}

async fn handle_config_data (config_data: Properties, ipadd: IpAdressSet) -> bool {
    let dnsprovider = config_data.get("dnsprovider");
    match dnsprovider {
        // Match a single value
        Some("Gandi") => handle_config_gandi(config_data, ipadd).await,
        // Match several values
        Some("Cloudflare") => false,
        // TODO add more dns options
        // Handle the rest of cases
        _ => false,
    }

}

async fn handle_config_gandi(config_data: Properties, ipadd: IpAdressSet) -> bool {
    // first parse the raw data
    // as the dnsprovider is Gandi, we will use the record_parser_gandi
    let record_data = record_parser_gandi(config_data);
    // update with new data
    let ipv = match record_data.rrset_type().as_str() {
        // if A record, give ipv4
        "A" => ipadd.get_ipv4(),
        // if AAAA record, give ipv6
        "AAAA" => ipadd.get_ipv6(),
        // else give local host to stop updating
        _ => "UNSUPPORTED".to_owned(),
    };
    if ipv == "localhost" {
        println!("Could not get public IP adress")
    } else if ipv == "UNSUPPORTED" {
        println!("Record type is currently unsupported")   
    } else {
        let update_result = update_record_gandi(record_data.clone(), ipv).await.unwrap();
        println!("Result of updating {:?}", update_result)
        
    }
        
    true
}