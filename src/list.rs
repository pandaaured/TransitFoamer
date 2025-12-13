struct FleetEntry {
    year: i32,
    manufacturer: String,
    model: String,
}

// Horrible. Fix this with some elegant pattern matching when 
// you get access to the docs!
impl FleetEntry {
    fn which(vid: i32) -> Option<FleetEntry> {
        if 3700 <= vid && vid <= 3759 {
            let entry = FleetEntry {
                year: 2014,
                manufacturer: "New Flyer".to_string(),
                model: "XDE35".to_string()
            };
            return Some(entry);
        } else if 4700 <= vid && vid <= 4719 {
            let entry = FleetEntry {
                year: 2021,
                manufacturer: "New Flyer".to_string(),
                model: "XE40".to_string()
            };
            return Some(entry);
        } else if 4800 <= vid && vid <= 4819 {
            let entry = FleetEntry {
                year: 2021,
                manufacturer: "New Flyer".to_string(),
                model: "XE60".to_string()
            };
            return Some(entry);
        } else if 6000 <= vid && vid <= 6019 {
            let entry = FleetEntry {
                year: 2009,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFA".to_string()
            };
            return Some(entry);
        }  else if 6020 <= vid && vid <= 6073 {
            let entry = FleetEntry {
                year: 2011,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string()
            };
            return Some(entry);
        } else if 6075 <= vid && vid <= 6117 {
            let entry = FleetEntry {
                year: 2013,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string()
            };
            return Some(entry);
        } else if 6200 <= vid && vid <= 6219 {
            let entry = FleetEntry {
                year: 2015,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string()
            };
            return Some(entry);
        } else if 6220 <= vid && vid <= 6241 {
            let entry = FleetEntry {
                year: 2018,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string()
            };
            return Some(entry);
        } else if 6242 <= vid && vid <= 6269 {
            let entry = FleetEntry {
                year: 2019,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string()
            };
            return Some(entry);
        } else if 6400 <= vid && vid <= 6412 {
            let entry = FleetEntry {
                year: 2023,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string()
            };
            return Some(entry);
        } else if 6813 <= vid && vid <= 6850 {
            let entry = FleetEntry {
                year: 2008,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LF".to_string()
            };
            return Some(entry);
        } else if 6851 <= vid && vid <= 6865 {
            let entry = FleetEntry {
                year: 2009,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LF".to_string()
            };
            return Some(entry);
        } else if 6866 <= vid && vid <= 6921 {
            let entry = FleetEntry {
                year: 2010,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string()
            };
            return Some(entry);
        } else if 6922 <= vid && vid <= 6935 {
            let entry = FleetEntry {
                year: 2011,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string()
            };
            return Some(entry);
        } else if 6936 <= vid && vid <= 6999 {
            let entry = FleetEntry {
                year: 2012,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string()
            };
            return Some(entry);
        } else if vid == 6800 {
            let entry = FleetEntry {
                year: 2012,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string()
            };
            return Some(entry);
        } else if 7001 <= vid && vid <= 7093 {
            let entry = FleetEntry {
                year: 2010,
                manufacturer: "Orion".to_string(),
                model: "VII".to_string()
            };
            return Some(entry);
        } else if 7094 <= vid && vid <= 7199 {
            let entry = FleetEntry {
                year: 2011,
                manufacturer: "Orion".to_string(),
                model: "VII".to_string()
            };
            return Some(entry);
        } else if 7200 <= vid && vid <= 7259 {
            let entry = FleetEntry {
                year: 2014,
                manufacturer: "New Flyer".to_string(),
                model: "XDE40".to_string()
            };
            return Some(entry);
        } else if vid == 7300 {
            let entry = FleetEntry {
                year: 2017,
                manufacturer: "Gillig".to_string(),
                model: "Low Floor HEV 40'".to_string()
            };
            return Some(entry);
        } else if 7301 <= vid && vid <= 7429 {
            let entry = FleetEntry {
                year: 2018,
                manufacturer: "Gillig".to_string(),
                model: "Low Floor HEV 40'".to_string()
            };
            return Some(entry);
        } else if 7430 <= vid && vid <= 7494 {
            let entry = FleetEntry {
                year: 2019,
                manufacturer: "Gillig".to_string(),
                model: "Low Floor HEV 40'".to_string()
            };
            return Some(entry);
        } else if 8000 <= vid && vid <= 8084 {
            let entry = FleetEntry {
                year: 2015,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string()
            };
            return Some(entry);
        } else if 8100 <= vid && vid <= 8199 {
            let entry = FleetEntry {
                year: 2016,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string()
            };
            return Some(entry);
        } else if 8200 <= vid && vid <= 8299 {
            let entry = FleetEntry {
                year: 2017,
                manufacturer: "New Flyer".to_string(),
                model: "XDE40".to_string()
            };
            return Some(entry);
        } else if 4300 <= vid && vid <= 4409 {
            let entry = FleetEntry {
                year: 2014,
                manufacturer: "New Flyer".to_string(),
                model: "XT40".to_string()
            };
            return Some(entry);
        } else if 4500 <= vid && vid <= 4563 {
            let entry = FleetEntry {
                year: 2015,
                manufacturer: "New Flyer".to_string(),
                model: "XT60".to_string()
            };
            return Some(entry);
        } else {
            return None;
        }
    }
}
