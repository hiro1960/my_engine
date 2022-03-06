// model land型
use super::basemodel;
use super::super::core;

pub struct LandModel {
    pub base: basemodel::BaseModel,
}

impl LandModel {
    pub fn new() -> LandModel {
        LandModel{
            // base: basemodel::BaseModel::new(num, sss),
            base: basemodel::BaseModel::new(),
        }
    }
}

impl basemodel::Model for LandModel {
    fn print_own(&self) {
        println!("in LandModel");
    }

    /**
     * 
     */
    fn set_up(&mut self) {

    }

    /**
     * オブジェクトの更新処理
     * 
     * @param[in/out] data  オブジェクト・データ
     * @param[in] env   環境データ
     */
    fn update(&self, data: &mut basemodel::BaseModel, env:&core::env::Env, dt:f64) {
        let mut pos = data.pos();   // data.posはCopyされるのに注意
        let vel = data.vel();

        pos.set_x(pos.x() + vel.x() * dt);
        pos.set_y(pos.y() + vel.y() * dt);
        pos.set_z(pos.z() + vel.z() * dt);

        data.set_pos(&pos);

    }

}