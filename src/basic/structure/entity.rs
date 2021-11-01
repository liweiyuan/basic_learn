
pub const  THRESHOLD: i32 = 10;

#[derive(Debug)]
pub struct Person<'a>{
    pub name :  &'a str,
    pub age : u8,
}

//单元结构体
pub struct Nil;

pub struct Pair(pub i32, pub i32);

pub struct Point{

    pub x: u8,
    pub y: u8,
}

impl Point{
    pub fn new(x : u8, y : u8) ->Self{
        Point{
            x,
            y,
        }
    }

    //面积计算
    pub fn square(&self) -> u64 {
        let square = self.x * self.y;
        square as u64
    }
}