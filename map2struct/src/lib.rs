/// Simple Crate for converting a HashMap<String, String>
/// to a struct.
/// 
/// Provides one main trait and derive macro, 
/// named `Map2Struct`.
/// 
/// # Example
/// ```
/// use std::collections::HashMap;
/// use map2struct::Map2Struct;
///
/// #[derive(Map2Struct)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let mut map = HashMap::new();
/// map.insert("name".to_string(), "John".to_string());
/// map.insert("age".to_string(), "30".to_string());
/// let person = Person::from_map(map).expect("Parsing failed");
/// assert_eq!(person.name, "John");
/// assert_eq!(person.age, 30);
/// ```
/// 
/// Fields are parsed using the `.parse` method of `String` 
/// values. 
/// 
/// The following validations steps are performed:
/// 1) Check if all fields are present
/// 2) Check that no additional fields are present
/// 3) Check type conversions
/// 

use std::collections::HashMap;

pub use map2struct_derive::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing field: {0}")]
    MissingField(String),

    #[error("Extra fields: {0:?}")]
    ExtraFields(Vec<String>),
    
    #[error("Failed to convert field '{0}': {1}")]
    FieldConversion(String, Box<dyn std::error::Error>)
}

pub type Result<S> = std::result::Result<S, Error>;


pub trait Map2Struct  {
    fn from_map(map: HashMap<String, String>) -> Result<Self> where Self: Sized;
}


