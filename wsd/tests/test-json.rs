use serde::{Serialize, Deserialize};
use std::{collections::HashMap};
use wsd::json::*;

#[test]
fn json_instance() {
    let mut json = json! {
        name: "native json",
        students:  [
            {name: "John", age: 17},
            {name: "Jack", age: 20}
        ],
        array: [1,2,3,4,5],
        vector: vec![5,4,3,2,1],
        hashmap: HashMap::from([("a", 1), ("b", 2), ("c", 3)]),
        rect: {x: 10, y: 10, width: 100, height: 50}
    };

    json.students[0].age += 1;    
    json.rect.x += 10;
    json.name = "Native JSON";
}

#[test]
fn json_declare() {
    json! {
    School {
        name: String,
        students:[{
            name: String,
            age: i32,
            tutor: {
                name: String,
                course: String
              }
            },
            ...
        ],
        nullable: Option<String>,
        map: HashMap<String, i32>
    }}

    let mut school = School::new();
    school.name = "MIT".to_owned();

    let mut newbie = School_students_item::new();
    let tutor = School_students_item_tutor { name: "Don Markuson".to_owned(), course: "Math".to_owned()};
    newbie.name = "John".to_owned();
    newbie.age = 17;
    newbie.tutor = tutor;
    school.students.push(newbie);
}

#[test]
fn json_serialize() {
    let mut json = json!{
        name: "native json",
        point: { x: 10, y: 20},
        array: [1,2,3,4,5],
        vector: vec![1,2,3,4,5,6]
    };

    let s = json.stringify(4);

    json.name = "";
    if json.parse(&s).is_ok() {
        assert_eq!(json.name, "native json");
    }

    let _ = json.to_string();
}

#[test]
fn json_read_write() -> Result<(), std::io::Error> {
    let mut json = json!{
        name: "native json",
        point: { x: 10, y: 20},
        array: [1,2,3,4,5],
        vector: vec![1,2,3,4,5,6],
        hashmap: HashMap::from([("a", 1), ("b", 2)])
    };

    let file = "json-rw.json";
    assert!(json.write(file).is_ok());

    json.name = "";
    json.vector.clear();
    json.hashmap.clear();

    // read and parse from file
    let mut zoombie = String::new();
    assert!(json.read(file, &mut zoombie).is_ok());
    assert!(json.vector[0] == 1);    

    // test methods
    let s = std::fs::read_to_string(file)?;
    assert!(json.parse(&s).is_ok());
    let _s1 = json.to_string();
    let _s2 = json.stringify(4);  

    wsd::fs::remove(file);

    Ok(())
}