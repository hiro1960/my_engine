// modelのbase型
use super::super::core;

use super::air;

pub struct BaseModel {
    // とりあえずテスト中はpub宣言しておく
    pub id: i32,
    pub name: String,
    pub pos: core::point::Point,
    pub vel: core::point::Point,
}

impl BaseModel {
    pub fn new(num:i32, sss:&str) -> BaseModel {
        BaseModel{
            id: num,
            name: sss.to_string(),
            pos: core::point::Point::new(),
            vel: core::point::Point::new(),
        }
    }

    pub fn set_id(&mut self, num: i32) {
        self.id = num;
    }

    pub fn set_name(&mut self, sss: &str) {
        self.name = sss.to_string();
    }

}

pub trait Model {
    fn print_own(&self);
}

// impl Model for air::AirModel {
//     fn print_own(&self) {
//         println!("in AirModel");
//     }

// }