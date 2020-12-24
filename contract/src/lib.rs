/**
 * Provides Models for objects
 * 
 */
extern crate serde; 
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::fmt::{self, Formatter, Display};

#[derive(Serialize, Deserialize, Debug)]
pub struct Edges{
    pub id:String,
    pub desc:String,
    pub url:String,
}

impl Display for Edges {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}",self.id, self.desc, self.url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let n = Edges{
            id:String::from("c368c393-ccb5-f134-213e-2c50730e75ea"),
            desc:String::from("Edge South Brazil - POA"),
            url:String::from("172.1.1.1"),
        };
        println!("{}",n);
    }

}