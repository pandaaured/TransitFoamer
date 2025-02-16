use std::fs;

pub fn is_configured_static() -> bool {
    let config = fs::read_to_string("./config.txt");
    if config.is_ok() {
        let string = config.ok();
        println!("{:?}", string);
        if string.is_none() {
            println!("Can't read file to string for some reason.");
            return false
        } else {
            let config = string.unwrap();
            let arguments: Vec<&str> = config.split('\n').collect();
            let path = arguments.get(2).unwrap();
            if *path == "" {
                println!("Unconfigured! Please run `cargo run -- config <foo>` to set a path for your GTFS static directory.");
                return false
            } else {
                return check_required_files(&path);
            }
        }
    }

    false
}

pub fn is_configured_realtime() -> bool {
    let config = fs::read_to_string("./config.txt").expect("File exists");
    let lines = config.split('\n');


    true
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

        let mut path_rt = path.to_string();
        path_rt.push_str("routes.txt");

        let mut path_tr = path.to_string();
        path_tr.push_str("trips.txt");

        let mut path_st = path.to_string();
        path_st.push_str("stop_times.txt");

        let arr = vec![path_ag, path_rt, path_tr, path_st];

        for path in arr {
            if !fs::exists(path.clone()).is_ok() {
                println!("Missing mandatory file {}", path);
                return false
            }
        }
        println!("Files agency.txt, routes.txt, trips.txt, stop_times.txt all found.");
        true
    }

}
