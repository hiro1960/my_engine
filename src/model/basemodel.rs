// modelのbase型
use super::super::core;

pub struct BaseModel {
    // とりあえずテスト中はpub宣言しておく
    id: i64,
    name: String,
    category: String,
    pos: core::point::Point,
    vel: core::point::Point,
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn category(&self) -> &str {
        &self.category
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
    pub fn pos(&self) -> core::point::Point {
        // Point型のCopy traitがここで動く(#[derive(Debug, Copy, Clone)]定義をしてある)
        self.pos
    }

    /**
     * 速度の取得
     * 
     * @return vel Point型
     */
    pub fn vel(&self) -> core::point::Point {
        self.vel
    }

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
