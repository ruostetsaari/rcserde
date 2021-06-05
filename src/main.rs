// Add to Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive", "rc"] }
// serde_json = "1.0"
use serde::{Serialize, Deserialize};
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Coordinate {
    name: String,
    ctype: String,
}

type RCoordinate = Rc<RefCell<Coordinate>>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Space {
    name: String,
    coordinates: Vec<RCoordinate>,
}

type RSpace = Rc<RefCell<Space>>;

// Note! Error handling ignored in these functions to shorten the code

fn parse_space(json_str: &str) -> RSpace {
    Rc::new(RefCell::new(serde_json::from_str(&json_str).unwrap()))
}

fn get_coordinate(space: &RSpace, index: usize) -> RCoordinate {
    space.borrow().coordinates[index].clone()
}

fn main() {
    let input_str = r#"
	{
	    "name": "2D",
	    "coordinates": [
		{
		    "name": "x",
		    "ctype": "float"
		},
		{
		    "name": "y",
		    "ctype": "float"
		}
	    ]
	}"#;
    let space1 = parse_space(&input_str);
    println!("space1 {:?}", &space1);
    let space2 = parse_space(&serde_json::to_string(&space1).unwrap());
    println!("space2 {:?}", &space2);
    assert_eq!(space1, space2);
    let space3 = space2.clone();
    space3.borrow_mut().name = "3D".to_string();
    space3.borrow_mut().coordinates.push(Rc::new(RefCell::new(Coordinate { name: "z".to_string(), ctype: "float".to_string() })));
    println!("space3 {:?}", &space3);
    println!("space2 {:?}", &space2);
    assert_eq!(space2, space3);
    println!("z_coordinate {:?}", &get_coordinate(&space2, 2));
}
