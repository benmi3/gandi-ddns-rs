use std::process;
use std::path::Path;
use check_records::check_record_gandi;
use ini::{Ini, Properties};
use record_parser::record_parser_gandi;
use update_records::update_record_gandi;

const CONF_FILE_NAME: &str = "config.ini";

mod check_records;
mod dns_providers;
mod get_ip;
mod record_parser;
mod update_records;

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
        create_config();
        // exiting program
        println!("Created the template");
        println!("Exiting program");
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

    println!("Iterating");
    let general_section_name = "__General__";
    for (sec, prop) in conf.iter() {
        let section_name = sec.as_ref().unwrap_or(&general_section_name);
        println!("-- Section: {:?} begins", section_name);
        println!("here comes prop");
        println!("-- Prop : {:?}", prop.get("dnsprovider"));
        let config_data = handle_config_data(prop.clone());
        
        if config_data {
            println!("handled the data succesfully")
        } else {
            println!("somthing happened, and the data handling was not successfull")
        }
    }
    println!();

    let section = conf.section(Some("Config1")).unwrap();
    println!("cmain={}", section.get("domain").unwrap());
    println!("conf[User][name]={}", &conf["Config1"]["domain"]);
    println!("General Section: {:?}", conf.general_section());
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

fn handle_config_data (config_data: Properties) -> bool {
    let dnsprovider = config_data.get("dnsprovider");
    match dnsprovider {
        // Match a single value
        Some("Gandi") => handle_config_gandi(config_data),
        // Match several values
        Some("Cloudflare") => false,
        // TODO add more dns options
        // Handle the rest of cases
        _ => false,
    }

}

fn handle_config_gandi(config_data: Properties) -> bool {
    // first parse the raw data
    // as the dnsprovider is Gandi, we will use the record_parser_gandi
    let record_data = record_parser_gandi(config_data);
    // check if the current values are the same as the config
    let check_result = check_record_gandi(record_data.clone());
    // if the current values are not the same, update the values
    if Some(check_result).is_some() {
        update_record_gandi(record_data.clone());
    } else {
        todo!();
    }
    true
}