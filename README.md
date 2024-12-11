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
|----(src)
     |----main.rs
     |----feedmessage.rs
     |----staticfeed.rs
     |----realtime.rs
     |----search.rs
     |----(static)
          |- *(Remark 2)

```