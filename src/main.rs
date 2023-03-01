use std::env;
use futures::executor::block_on;
use semver::{Version};

use self_update_test;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    let base_url = &String::from("http://172.16.3.95:18888/self_update_test/x86_64"); 
    let file_name = &String::from("self_update_test");
    let version_url = format!("{base_url}/version");
    let file_url = format!("{base_url}/{file_name}");
    let version: semver::Version = Version::parse(VERSION).unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--self-update" {
        let is_update_binary = self_update_test::check_version_text(&version, &version_url);
        if let Ok(_) = block_on(is_update_binary) {
            let future = self_update_test::self_update(&file_url, file_name);
            match block_on(future) {
                Ok(_) => println!("Successfully updated to the latest version!"),
                Err(err) => eprintln!("Error updating binary: {}", err),
            }
        }

        return;
    }

    if args.len() > 1 && (args[1] == "--version" || args[1] == "-V") {
        println!("Running version {}", version.to_owned());
        return;
    }

    println!("Running version {}", version.to_owned());

    // The rest of the program's code would go here.
}

