use std::io::stdout;
use std::process;
use std::path::Path;
use ini::Ini;

const CONF_FILE_NAME: &str = "config.ini";

mod get_ip;

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

    // start creating and updating entries
    let mut conf = Ini::new();
    conf.with_section(None::<String>).set("encoding", "utf-8");
    conf.with_section(Some("User"))
        .set("name", "Raspberry树莓")
        .set("value", "Pi");
    conf.with_section(Some("Library"))
        .set("name", "Sun Yat-sen U")
        .set("location", "Guangzhou=world\x0ahahaha");

    conf.section_mut(Some("Library")).unwrap().insert("seats", "42");

    println!("---------------------------------------");
    println!("Writing to file {:?}\n", CONF_FILE_NAME);
    conf.write_to(&mut stdout()).unwrap();

    conf.write_to_file(CONF_FILE_NAME).unwrap();

    println!("----------------------------------------");
    println!("Reading from file {:?}", CONF_FILE_NAME);
    let i = Ini::load_from_file(CONF_FILE_NAME).unwrap();

    println!("Iterating");
    let general_section_name = "__General__";
    for (sec, prop) in i.iter() {
        let section_name = sec.as_ref().unwrap_or(&general_section_name);
        println!("-- Section: {:?} begins", section_name);
        for (k, v) in prop.iter() {
            println!("{}: {:?}", k, v);
        }
    }
    println!();

    let section = i.section(Some("User")).unwrap();
    println!("name={}", section.get("name").unwrap());
    println!("conf[User][name]={}", &i["User"]["name"]);
    println!("General Section: {:?}", i.general_section());
}

fn create_config () -> bool {
    // start with conf
    let mut conf = Ini::new();
    // set encoding to UTF-8
    conf.with_section(None::<String>).set("encoding", "utf-8");
    conf.with_section(Some("Config1"))
        .set("apikey", "s3cr3t4p1k3y")
        .set("domain", "example.com")
        .set("rrset_name", "@")
        .set("rrset_type", "A")
        .set("rrset_ttl", "18000");
    conf.with_section(Some("Config2"))
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