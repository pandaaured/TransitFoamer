use gtfs_realtime::FeedEntity;

/* Top level module comment: 
 * This file contains the following structs along with some of their impls:
 * - FeedCategory
 * - Fleet
 * - FleetEntry
 * - HTML
 * - Links
 */

pub struct FeedCategory {
    pub list: Vec<FeedEntity>,
    pub name: String
}

#[derive(PartialEq)]
pub enum Fleet {
    NFIXDE352014,
    NFIXE402021,
    NFIXE602021,
    NFIDE60LFA2009,
    NFIDE60LFR2011,
    NFIDE60LFR2013,
    NFIXDE602015,
    NFIXDE602018,
    NFIXDE602019,
    NFIXDE602023,
    NFIDE60LF2008,
    NFIDE60LF2009,
    NFIDE60LFR2010,
    NFIDE60LFR2012,
    OBIVII2010,
    OBIVII2011,
    NFIXDE402014,
    GILLIGHEV402017,
    GILLIGHEV402018,
    GILLIGHEV402019,
    NFIXDE602014,
    NFIXDE602016,
    NFIXDE602017,
    NFIXDE402017,
    NFIXT402014,
    NFIXT602015,
    NoneFleet,
}

struct FleetEntry {
    year: i32,
    manufacturer: String,
    model: String,
}

#[derive(Debug, Clone)]
pub struct HTML {
    pub class: String,
    pub id: String,
    pub value: String,
}

pub struct Links {
    pub vehicles: String,
    pub trips: String,
    pub alerts: String,
}


// Horrible. Fix this with some elegant pattern matching when
// you get access to the docs!
impl FleetEntry {
    fn which(vid: i32) -> Option<FleetEntry> {
        if 3700 <= vid && vid <= 3759 {
            let entry = FleetEntry {
                year: 2014,
                manufacturer: "New Flyer".to_string(),
                model: "XDE35".to_string(),
            };
            return Some(entry);
        } else if 4700 <= vid && vid <= 4719 {
            let entry = FleetEntry {
                year: 2021,
                manufacturer: "New Flyer".to_string(),
                model: "XE40".to_string(),
            };
            return Some(entry);
        } else if 4800 <= vid && vid <= 4819 {
            let entry = FleetEntry {
                year: 2021,
                manufacturer: "New Flyer".to_string(),
                model: "XE60".to_string(),
            };
            return Some(entry);
        } else if 6000 <= vid && vid <= 6019 {
            let entry = FleetEntry {
                year: 2009,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFA".to_string(),
            };
            return Some(entry);
        } else if 6020 <= vid && vid <= 6073 {
            let entry = FleetEntry {
                year: 2011,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string(),
            };
            return Some(entry);
        } else if 6075 <= vid && vid <= 6117 {
            let entry = FleetEntry {
                year: 2013,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string(),
            };
            return Some(entry);
        } else if 6200 <= vid && vid <= 6219 {
            let entry = FleetEntry {
                year: 2015,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string(),
            };
            return Some(entry);
        } else if 6220 <= vid && vid <= 6241 {
            let entry = FleetEntry {
                year: 2018,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string(),
            };
            return Some(entry);
        } else if 6242 <= vid && vid <= 6269 {
            let entry = FleetEntry {
                year: 2019,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string(),
            };
            return Some(entry);
        } else if 6400 <= vid && vid <= 6412 {
            let entry = FleetEntry {
                year: 2023,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string(),
            };
            return Some(entry);
        } else if 6813 <= vid && vid <= 6850 {
            let entry = FleetEntry {
                year: 2008,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LF".to_string(),
            };
            return Some(entry);
        } else if 6851 <= vid && vid <= 6865 {
            let entry = FleetEntry {
                year: 2009,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LF".to_string(),
            };
            return Some(entry);
        } else if 6866 <= vid && vid <= 6921 {
            let entry = FleetEntry {
                year: 2010,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string(),
            };
            return Some(entry);
        } else if 6922 <= vid && vid <= 6935 {
            let entry = FleetEntry {
                year: 2011,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string(),
            };
            return Some(entry);
        } else if 6936 <= vid && vid <= 6999 {
            let entry = FleetEntry {
                year: 2012,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string(),
            };
            return Some(entry);
        } else if vid == 6800 {
            let entry = FleetEntry {
                year: 2012,
                manufacturer: "New Flyer".to_string(),
                model: "DE60LFR".to_string(),
            };
            return Some(entry);
        } else if 7001 <= vid && vid <= 7093 {
            let entry = FleetEntry {
                year: 2010,
                manufacturer: "Orion".to_string(),
                model: "VII".to_string(),
            };
            return Some(entry);
        } else if 7094 <= vid && vid <= 7199 {
            let entry = FleetEntry {
                year: 2011,
                manufacturer: "Orion".to_string(),
                model: "VII".to_string(),
            };
            return Some(entry);
        } else if 7200 <= vid && vid <= 7259 {
            let entry = FleetEntry {
                year: 2014,
                manufacturer: "New Flyer".to_string(),
                model: "XDE40".to_string(),
            };
            return Some(entry);
        } else if vid == 7300 {
            let entry = FleetEntry {
                year: 2017,
                manufacturer: "Gillig".to_string(),
                model: "Low Floor HEV 40'".to_string(),
            };
            return Some(entry);
        } else if 7301 <= vid && vid <= 7429 {
            let entry = FleetEntry {
                year: 2018,
                manufacturer: "Gillig".to_string(),
                model: "Low Floor HEV 40'".to_string(),
            };
            return Some(entry);
        } else if 7430 <= vid && vid <= 7494 {
            let entry = FleetEntry {
                year: 2019,
                manufacturer: "Gillig".to_string(),
                model: "Low Floor HEV 40'".to_string(),
            };
            return Some(entry);
        } else if 8000 <= vid && vid <= 8084 {
            let entry = FleetEntry {
                year: 2015,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string(),
            };
            return Some(entry);
        } else if 8100 <= vid && vid <= 8199 {
            let entry = FleetEntry {
                year: 2016,
                manufacturer: "New Flyer".to_string(),
                model: "XDE60".to_string(),
            };
            return Some(entry);
        } else if 8200 <= vid && vid <= 8299 {
            let entry = FleetEntry {
                year: 2017,
                manufacturer: "New Flyer".to_string(),
                model: "XDE40".to_string(),
            };
            return Some(entry);
        } else if 4300 <= vid && vid <= 4409 {
            let entry = FleetEntry {
                year: 2014,
                manufacturer: "New Flyer".to_string(),
                model: "XT40".to_string(),
            };
            return Some(entry);
        } else if 4500 <= vid && vid <= 4563 {
            let entry = FleetEntry {
                year: 2015,
                manufacturer: "New Flyer".to_string(),
                model: "XT60".to_string(),
            };
            return Some(entry);
        } else {
            return None;
        }
    }
}

impl Fleet {
    pub fn which(vid: i32) -> Fleet {
        if 3700 <= vid && vid <= 3759 {
            return Fleet::NFIXDE352014;
        } else if 4700 <= vid && vid <= 4719 {
            return Fleet::NFIXE402021;
        } else if 4800 <= vid && vid <= 4819 {
            return Fleet::NFIXE602021;
        } else if 6000 <= vid && vid <= 6019 {
            return Fleet::NFIDE60LFA2009;
        } else if 6020 <= vid && vid <= 6073 {
            return Fleet::NFIDE60LFR2011;
        } else if 6075 <= vid && vid <= 6117 {
            return Fleet::NFIDE60LFR2013;
        } else if 6200 <= vid && vid <= 6219 {
            return Fleet::NFIXDE602015;
        } else if 6220 <= vid && vid <= 6241 {
            return Fleet::NFIXDE602018;
        } else if 6242 <= vid && vid <= 6269 {
            return Fleet::NFIXDE602019;
        } else if 6400 <= vid && vid <= 6412 {
            return Fleet::NFIXDE602023;
        } else if 6813 <= vid && vid <= 6850 {
            return Fleet::NFIDE60LF2008;
        } else if 6851 <= vid && vid <= 6865 {
            return Fleet::NFIDE60LF2009;
        } else if 6866 <= vid && vid <= 6921 {
            return Fleet::NFIDE60LFR2010;
        } else if 6922 <= vid && vid <= 6935 {
            return Fleet::NFIDE60LFR2011;
        } else if 6936 <= vid && vid <= 6999 {
            return Fleet::NFIDE60LFR2012;
        } else if vid == 6800 {
            return Fleet::NFIDE60LFR2012;
        } else if 7001 <= vid && vid <= 7093 {
            return Fleet::OBIVII2010;
        } else if 7094 <= vid && vid <= 7199 {
            return Fleet::OBIVII2011;
        } else if 7200 <= vid && vid <= 7259 {
            return Fleet::NFIXDE402014;
        } else if vid == 7300 {
            return Fleet::GILLIGHEV402017;
        } else if 7301 <= vid && vid <= 7429 {
            return Fleet::GILLIGHEV402018;
        } else if 7430 <= vid && vid <= 7494 {
            return Fleet::GILLIGHEV402019;
        } else if 8000 <= vid && vid <= 8084 {
            return Fleet::NFIXDE602015;
        } else if 8100 <= vid && vid <= 8199 {
            return Fleet::NFIXDE602016;
        } else if 8200 <= vid && vid <= 8299 {
            return Fleet::NFIXDE402017;
        } else if 4300 <= vid && vid <= 4409 {
            return Fleet::NFIXT402014;
        } else if 4500 <= vid && vid <= 4563 {
            return Fleet::NFIXT602015;
        } else {
            return Fleet::NoneFleet;
        }
    }
}

// A struct which contains the GTFS-RT URLs for a given feed.
impl Links {
    // Initializes a struct with the hard-coded URLs.
    pub fn new() -> Links {
        Links {
            vehicles: "https://s3.amazonaws.com/kcm-alerts-realtime-prod/vehiclepositions.pb"
                .to_string(),
            trips: "https://s3.amazonaws.com/kcm-alerts-realtime-prod/tripupdates.pb".to_string(),
            alerts: "https://s3.amazonaws.com/kcm-alerts-realtime-prod/alerts.pb".to_string(),
        }
    }
}

impl HTML {
    pub fn new(class: String, id: String, value: String) -> HTML {
        HTML {
            class: class,
            id: id,
            value: value,
        }
    }

    pub fn conv_to_string(&self) -> String {
        let string = format!(
            "<div class={} id={}> {} </div>",
            self.class, self.id, self.value
        );
        string
    }

    pub fn append_value(mut html: HTML, string: String) -> HTML {
        html.value.push_str(string.as_str());

        html
    }

    pub fn set_value(mut html: HTML, string: String) -> HTML {
        html.value = string;
        html
    }
}
