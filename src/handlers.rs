use std::fs;

use axum::{response::{Html, Json}};
use crate::{html::HTML, links::Links};
use crate::Fleet;

use gtfs_realtime::{FeedMessage, FeedEntity};
use crate::gtfs_static::Routes;
use crate::gtfs_rt::url_to_feedmessage;

// This function is a handler for the main request to deliver the website content 
// to the client. 
pub async fn main_content_handler() -> Html<String> { 
    let file_path = "static/index.html".to_string();
    let file_as_string =
        fs::read_to_string(file_path).expect("Should have been able to read src/index.html");
    Html(file_as_string)
}

// Returns an HTML response with a list of all routes in a feed. 
pub async fn route_list_handler() -> Html<String> {
    let file_path = "src/static/seattle/kc/".to_string();
    let routes = Routes::new_vec(&file_path).unwrap();
    let partition: (Vec<Routes>, Vec<Routes>) = routes.into_iter().partition(|x| x.route_short_name.as_ref().unwrap().parse::<i32>().is_ok());
    let mut top_level = HTML::new("routes".to_string(), "routes".to_string(), "".to_string());
    let mut build_up = String::new();
    let mut vec1 = partition.0;
    vec1.sort_by_key(|x| {
        x.route_short_name.as_ref().unwrap().parse::<i32>().unwrap()
    });
    let vec2 = partition.1;
    vec1.extend(vec2);
    for item in vec1 {
        let name = &item.route_short_name.unwrap();
        let desc = &item.route_desc.unwrap();
        let mut item_container: HTML = HTML::new("route".to_string(), name.to_owned(), "".to_string());
        let route_name: HTML = HTML::new(format!("{}-name", name.to_owned()),
                                         "".to_string(),
                                         name.to_owned());
        let route_desc: HTML = HTML::new(format!("{}-desc", name.to_owned()),
                                         "".to_string(),
                                         desc.to_owned());
        let route_str = route_name.conv_to_string();
        let desc_str = route_desc.conv_to_string();
        let new_str = format!("{}{}", route_str, desc_str);
        item_container.value = new_str;
        let item_str = item_container.conv_to_string();
        build_up.push_str(item_str.as_str());
    }
    top_level.value = build_up;
    Html(top_level.conv_to_string())
}

pub async fn nfi_de60lf_2008_handler() -> Vec<u8> {
    let links = Links::new();
    let vehicle_position = url_to_feedmessage(links.vehicles).await.unwrap();
    let header = vehicle_position.header;
    let entities = vehicle_position.entity;
    let filtered = entities.iter()
        .filter(|x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_DE_60_LF_2008);
    let collection: Vec<&FeedEntity> = filtered.collect();
    let new_msg = FeedMessage {
       header: header,
       entity: collection.into_iter().cloned().collect(),
    };
    let mut buffer: Vec<u8> = Vec::new();
    let content = prost::Message::encode(&new_msg, &mut buffer);
    println!("{:?}", content);
    if content.is_ok() {
        let message = buffer;
        message
    } else {
        let vec: Vec<u8> = Vec::new(); 
        vec
    }
}

pub async fn nfi_de60lf_2009_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_DE_60_LF_2009);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_de60lfa_2009_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_DE_60_LFA_2009);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_de60lfr_2010_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_DE_60_LFR_2010);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_de60lfr_2011_handler() -> Vec<u8> {
    let links = Links::new();
    let vehicle_position = url_to_feedmessage(links.vehicles).await.unwrap();
    let header = vehicle_position.header;
    let entities = vehicle_position.entity;
    let filtered = entities.iter()
        .filter(|x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_DE_60_LFR_2011);
    let collection: Vec<&FeedEntity> = filtered.collect();
    let new_msg = FeedMessage {
       header: header,
       entity: collection.into_iter().cloned().collect(),
    };
    let msg_as_bytes = serde_json::to_vec(&new_msg).unwrap();
    msg_as_bytes
}

pub async fn nfi_de60lfr_2012_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_DE_60_LFR_2012);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_de60lfr_2013_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_DE_60_LFR_2013);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde35_2014_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_35_2014);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde40_2014_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_40_2014);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde60_2014_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_60_2014);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub  async fn nfi_xt40_2014_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XT_40_2014);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde60_2015_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_60_2015);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xt60_2015_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XT_60_2015);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde60_2016_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_60_2016);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde40_2017_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_40_2017);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde60_2017_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_60_2017);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde60_2018_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_60_2018);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde60_2019_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_60_2019);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xe40_2021_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XE_40_2021);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xe60_2021_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XE_60_2021);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn nfi_xde60_2023_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::NFI_XDE_60_2023);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn gillig_hev40_2017_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::GILLIG_HEV_40_2017);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn gillig_hev40_2018_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::GILLIG_HEV_40_2018);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn gillig_hev40_2019_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::GILLIG_HEV_40_2019);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)

}

pub async fn orion_2010_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::OBI_VII_2010);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}

pub async fn orion_2011_handler() -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicles = trip_update.entity;
    let filtered = vehicles.iter()
        .filter(|&x| 
                Fleet::which(x.vehicle.clone().unwrap().vehicle.unwrap().id().parse::<i32>().unwrap()) 
             == Fleet::OBI_VII_2011);
    let filtered_vec: Vec<&FeedEntity> = filtered.collect();

    let string = serde_json::to_string(&filtered_vec).unwrap();
    Json(string)
}
