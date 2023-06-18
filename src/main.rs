mod get_ip;

//use configparser::ini::Ini;
//use log::{info,warn};

#[tokio::main]
async fn main() {
    //let mut config = Ini::new();
    //let _ = config.read(String::from("[somesection]someintvalue = 5"));
    //let my_value = config.getint("somesection", "someintvalue").unwrap().unwrap();
    //assert_eq!(my_value, 5); // value accessible!

    //You can ofcourse just choose to parse the values yourself:
    //let my_string = String::from("1984");
    //let my_int = my_string.parse::<i32>().unwrap();
    //println!("This is my int {my_int}");
    // Attempt to get an IP address and print it.
    let ip_set = get_ip::get_ipadress().await;
}
