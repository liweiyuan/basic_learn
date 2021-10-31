
#[derive(Debug)]
pub struct Person<'a>{
    pub name :  &'a str,
    pub age : u8,
}