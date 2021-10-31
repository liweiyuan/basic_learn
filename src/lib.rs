

mod front_of_house;
mod engine;
mod basic;

use crate::engine::java_engine;
use crate::front_of_house::hosting;
use crate::basic::structure::entity;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    java_engine::add_to_wait_list();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let result = java_engine::add_to_wait_list();
        assert_eq!(result, 1,"result mut equal 1");
    }

    #[test]
    fn test_hosting(){
        let result = hosting::add_to_wait_list();
        assert!(result);
    }

    //person
    #[test]
    fn test_person(){
        let name = "Peter";
        let age = 27;
        let perter = entity::Person{
            name,
            age,
        };

        println!("Person {:?}", perter)
    }
}
