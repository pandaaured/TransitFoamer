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
TransitFoamer_Rust
|----(data)
     |- *(Remark 1)
|----(fetchers)
     |----gtfsrt_pb2.py
     |- *(Remark 1)
|----(src)
     |----main.rs
     |----feedmessage.rs
     |----staticfeed.rs
     |----realtime.rs
     |----search.rs
     |----(static)
          |- *(Remark 2)
```

### Remark 1
This folder is where I put data that I obtain from running separate scripts
which I have stored in the folder `fetchers`. 

### Remark 2 
Inside the static folder, I have placed the GTFS static definitions for each 
city I am interested in analyzing. The files are placed in directories like 
`<city_name>/<org_abbrev>`: all files can be accessed from paths like this.

## Functionality
Currently, there are two* working features. They are 
+ Vehicles within a certain range (that is, `vehicle_id`s).
+ Vehicles on a certain route (similarly, `vehicle_id`s).

## Future plans
+ Converting python scripts to Rust.
+ Predictions for a given stop.
