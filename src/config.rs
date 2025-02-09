use std::fs;
use std::io;

pub fn is_configured() -> bool {
    let config = fs::read_to_string("./config.txt");
    if config.is_ok() {
        let string = config.ok();
        println!("{:?}", string);
        if string.is_none() {
            println!("Can't read file to string for some reason.");
            return false
        } else {
            let path = string.unwrap();
            if path == "" {
                println!("Unconfigured! Please run `cargo run -- config <foo>` to set a path for your GTFS static directory.");
                return false
            } else {
                return check_required_files(&path);
            }
        }
    }
    
    false
}


fn check_required_files(path: &str) -> bool {
    let dir = fs::read_dir(&path);
    if dir.is_err() {
        println!("This isn't a valid directory. Please try again!");
        false 
    } else {
        println!("This directory works. Checking for GTFS data.");

        // The GTFS standard lists the following files with the following 
        // requirements:
        // agency.txt - REQUIRED
        // routes.txt - REQUIRED
        // stop_times.txt - REQUIRED
        // trips.txt - REQUIRED


        let mut path_ag = path.to_string();
        path_ag.push_str("agency.txt");
        if !fs::exists(path_ag.clone()).is_ok() {
            println!("Missing mandatory file agency.txt");
            return false
        }

        let mut path_rt = path.to_string();
        path_rt.push_str("routes.txt");
        if !fs::exists(path_rt).is_ok() {
            println!("Missing mandatory file routes.txt");
            return false
        }

        let mut path_tr = path.to_string();
        path_tr.push_str("trips.txt");
        if !fs::exists(path_tr).is_ok() {
            println!("Missing mandatory file trips.txt");
            return false 
        }

        let mut path_st = path.to_string();
        path_st.push_str("stop_times.txt");
        if !fs::exists(path_st).is_ok() {
            println!("Missing mandatory file stop_times.txt");
            return false
        }

        println!("Files agency.txt, routes.txt, trips.txt, stop_times.txt all found.");


        true

    }

}
