# Simple Crate For Converting Hashmaps To Structs

Convert `HashMap<String, String>` values directly into structs,
with optional type conversion per field.

Provides one main trait and derive macro,
named `Map2Struct`.

# Example

```rust 
use std::collections::HashMap;
use map2struct::Map2Struct;

#[derive(Map2Struct)]
struct Person {
    name: String,
    age: u32,
}

let mut map = HashMap::new();
map.insert("name".to_string(), "John".to_string());
map.insert("age".to_string(), "30".to_string());
let person = Person::from_map(map).expect("Parsing failed");
assert_eq!(person.name, "John");
assert_eq!(person.age, 30);
```

Fields are parsed using the `.parse` method of `String`
values.

The following validations steps are performed:
1) Check if all fields are present
2) Check that no additional fields are present
3) Check type conversions
