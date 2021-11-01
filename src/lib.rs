

mod front_of_house;
mod engine;
mod basic;
mod type_transform;

use crate::engine::java_engine;
use crate::front_of_house::hosting;
use crate::basic::structure::entity;
use crate::type_transform::from;

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

        println!("Person {:?}", perter);
        assert_eq!(perter.age, 27);

        let nil = entity::Nil;

        //println!(nil);

        let pair = entity::Pair(-1,3);
        println!("pair contains {:?} and {:?}", pair.0, pair.1);
        assert_eq!(pair.0 , -1);
    }

    #[test]
    fn test_square(){
        let rectangle = entity::Point::new(3,2);
        assert_eq!(rectangle.square(), 6);
        //常量
        assert_eq!(entity::THRESHOLD, 10);
    }

    #[test]
    fn test_vec(){
        let mut vec = Vec::new();
        vec.push(20);
        println!("{:?}", vec);
    }

    #[test]
    fn test_from_trait(){

        //From trait
        let from = from::Number::from(20);
        println!("number is {:?}", from);

        //Into trait
        let int = 5;
        let num: from::Number = int.into();
        println!("number is {:?}", num);
    }



}
