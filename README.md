# TransitFoamer 2

## Disclaimer

This project is my foray into making a large project in Rust and as such will 
undoubtedly suffer from first-time user pitfalls. I have endeavored to replicate
the behavior of my first major programming project (TransitFoamer) which was
written in Python. In addition, I sought to add functionality to my project 
which would have been difficult to add to it in the state it was formerly in.

I have made much more extensive use of modularity and separation of concerns for
this project than I did for my previous, which was significantly more haphazard
than I would have liked. 

## Directory

The project directory looks something like this:
```
TransitFoamer
|----(src)
     |----main.rs
     |----import.rs
     |----script.rs
     |----search.rs
     |----gtfsrt.rs
     |----gtfsstatic.rs
     |----(static)
          |- *(Remark 1)
          |----index.txt
```

### Remark 1
Inside the static folder, I have placed the GTFS static definitions for each 
city I am interested in analyzing. The files are placed in directories like 
`<city_name>/<org_abbrev>`: all files can be accessed from paths like this.
The file `index.txt` looks something like this:
```
00,/pittsburgh/prt/,Pittsburgh
01,/san_antonio/via/,San Antonio
02,/seattle/king_county/,Seattle
03,/chicago/cta/,Chicago
04,/san_francisco/muni/,San Francisco
```
It contains three attributes: (1) input codes, (2) file paths, (3) city names.

## Project Organization
+ `main.rs` passes off input handling logic to `import.rs`.
+ `gtfsstatic.rs` contains functions which generate GTFS static data from their
definitions.
+ `gtfsrt.rs` contains functions which generate GTFS realtime data from their
definitions.
+ `import.rs` contains input handling logic.
+ `search.rs` contains the features which get the data used to satisfy client
requests.
+ `script.rs` gets specialized data from the static feed.

## Configuration
Cloning the repository: `git clone https://github.com/hauschkaured/TransitFoamer`.
Once your working directory is the repo, enter `src` and run `mkdir static`.
This folder is where you will place folders containing GTFS static feeds.
Create an `index.txt` file as well. 

## Functionality
To run the program, enter the following command(s):

+ `cargo run` generates a list of cities currently in your `index.txt`.
+ `cargo run -- <code> routes` generates a list of routes operated by a given
transit agency.
+ `cargo run -- <code> route <foo>` generates a list of vehicles operating on a 
given route.
+ `cargo run -- <code> range <foo-bar>` generates a list of vehicles operating 
where the vehicles are within vehicle IDs of `foo` and `bar`. NOTE: your vehicle 
IDs must be numerical. 
+ `cargo run -- <code> stop <foo,bar>` generates a list of vehicles with real-time
tracked arrivals at the inputted stop. Stop must be valid!
