# rcserde
A small rust example using [serde_json](https://docs.serde.rs/serde_json/), [std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html), and [std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html). We want to parse a json file or string describing a Space object that contains a Vec of Coordinate objects. We want to access and mutate the Space objects and the Coordinate objects and pass them as function parameters and return values without making copies of these objects.

Originally the json file(s) have been processed by Python code and this is a simpliefied detail of the reimplementation in Rust. 

*Is this a good way to do this? Can you suggest improvements?*

## Running

`cargo run`

## Expected output

```
space1 RefCell { value: Space { name: "2D", coordinates: [RefCell { value: Coordinate { name: "x", ctype: "float" } }, RefCell { value: Coordinate { name: "y", ctype: "float" } }] } }
space2 RefCell { value: Space { name: "2D", coordinates: [RefCell { value: Coordinate { name: "x", ctype: "float" } }, RefCell { value: Coordinate { name: "y", ctype: "float" } }] } }
space3 RefCell { value: Space { name: "3D", coordinates: [RefCell { value: Coordinate { name: "x", ctype: "float" } }, RefCell { value: Coordinate { name: "y", ctype: "float" } }, RefCell { value: Coordinate { name: "z", ctype: "float" } }] } }
space2 RefCell { value: Space { name: "3D", coordinates: [RefCell { value: Coordinate { name: "x", ctype: "float" } }, RefCell { value: Coordinate { name: "y", ctype: "float" } }, RefCell { value: Coordinate { name: "z", ctype: "float" } }] } }
z_coordinate RefCell { value: Coordinate { name: "z", ctype: "float" } }
```
