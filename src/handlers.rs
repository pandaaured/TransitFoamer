use std::fs;

use crate::structs::{Links, HTML};
use axum::response::Html;

use crate::gtfs_rt::url_to_feedmessage;
use crate::gtfs_static::Routes;
use crate::helpers;

//  Handles the main request (returns index.html)
pub async fn main_content_handler() -> Html<String> {
    let file_path = "static/index.html".to_string();
    let file_as_string =
        fs::read_to_string(file_path).expect("Should have been able to read src/index.html");
    Html(file_as_string)
}

// Handles the routes API call, returning HTML which contains all routes (for King County Metro,
// this means the attributes route_short_name and route_description).
pub async fn route_list_handler() -> Html<String> {
    let file_path = "src/static/seattle/kc/".to_string();
    let routes = Routes::new_vec(&file_path).unwrap();
    let partition: (Vec<Routes>, Vec<Routes>) = routes
        .into_iter()
        .partition(|x| x.route_short_name.as_ref().unwrap().parse::<i32>().is_ok());
    let mut top_level = HTML::new("routes".to_string(), "routes".to_string(), "".to_string());
    let mut build_up = String::new();
    let mut vec1 = partition.0;
    vec1.sort_by_key(|x| x.route_short_name.as_ref().unwrap().parse::<i32>().unwrap());
    let vec2 = partition.1;
    vec1.extend(vec2);
    for item in vec1 {
        let name = &item.route_short_name.unwrap();
        let desc = &item.route_desc.unwrap();
        let mut item_container: HTML =
            HTML::new("route".to_string(), name.to_owned(), "".to_string());
        let route_name: HTML = HTML::new(
            format!("{}-name", name.to_owned()),
            "".to_string(),
            name.to_owned(),
        );
        let route_desc: HTML = HTML::new(
            format!("{}-desc", name.to_owned()),
            "".to_string(),
            desc.to_owned(),
        );
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

// Handles the fleet API call, returning HTML which contains each fleet category (as enumerated 
// in list.rs) along with the currently running vehicles on it.
pub async fn fleet_handler() {
    let links: Links = Links::new();
    let vehicle_posn = url_to_feedmessage(links.vehicles).await.unwrap();
    let vehicle_entities = vehicle_posn.entity;
    let trip_upd = url_to_feedmessage(links.trips).await.unwrap();
    let category = helpers::get_collection(vehicle_entities); 
        
    let mut top_level: HTML = HTML::new("fleet".to_string(), "fleet".to_string(), "".to_string());
    for item in category {
        let container = HTML::new("fleet-elem".to_string(), item.name, "".to_string());
            helpers::collection_split_html(item.list);
    }

}
