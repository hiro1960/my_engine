// modelのbase型
use super::super::core;

pub struct BaseModel {
    // とりあえずテスト中はpub宣言しておく
    id: i64,
    pub name: String,
    pub category: String,
    pub pos: core::point::Point,
    pub vel: core::point::Point,
}

impl BaseModel {
    pub fn new() -> BaseModel {
        BaseModel{
            id: 0,
            name: String::new(),
            category: String::new(),
            pos: core::point::Point::new(),
            vel: core::point::Point::new(),
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn set_id(&mut self, num: i64) {
        self.id = num;
    }

    pub fn set_name(&mut self, sss: &str) {
        self.name = sss.to_string();
    }

    pub fn set_category(&mut self, sss: &str) {
        self.category = sss.to_string();
    }

    /**
     * 位置の取得
     * 
     * @return pos Point型
     */
    // Copy traitが必要らしく、現状の実装ではうまくいかない
    // pub fn pos(&self) -> core::point::Point {
    //     let mut pt = core::point::Point::new();
    //     pt.set_x(self.pos().x());
    //     pt.set_y(self.pos().y());
    //     pt.set_z(self.pos().z());
    //     return pt;
    // }

    /**
     * 速度の取得
     * 
     * @return vel Point型
     */
    // pub fn vel(&self) -> core::point::Point {
    //     let mut pt = core::point::Point::new();
    //     pt.set_x(self.vel().x());
    //     pt.set_y(self.vel().y());
    //     pt.set_z(self.vel().z());
    //     return pt;
    // }

    /**
     * 位置の設定
     * @param[in] pt Point型
     */
    pub fn set_pos(&mut self, pt: &core::point::Point) {
        // 位置の設定
        self.pos.set_x( pt.x() );
        self.pos.set_y( pt.y() );
        self.pos.set_z( pt.z() );
    }

    /**
     * 速度の設定
     * @param[in] pt Point型
     */
    pub fn set_vel(&mut self, pt: &core::point::Point) {
        // 速度の設定
        self.vel.set_x( pt.x() );
        self.vel.set_y( pt.y() );
        self.vel.set_z( pt.z() );
    }

}

pub trait Model {
    fn print_own(&self);
    fn set_up(&mut self);

    fn update(&self, data: &mut BaseModel, env:&core::env::Env, dt: f64);
}
