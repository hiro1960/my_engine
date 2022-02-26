// modelのbase型
use super::super::core;

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

    pub fn set_pos(&mut self, pt: &core::point::Point) {
        // 位置の設定
        self.pos.set_x( pt.x() );
        self.pos.set_y( pt.y() );
        self.pos.set_z( pt.z() );
    }

}

pub trait Model {
    fn print_own(&self);
    fn set_up(&mut self);
}
