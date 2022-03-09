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
        // DEBUG
        println!("### in LandModel::update()");
        let mut pos = data.pos();   // data.posはCopyされるのに注意
        let vel = data.vel();

        pos.set_x(pos.x() + vel.x() * dt);
        pos.set_y(pos.y() + vel.y() * dt);
        pos.set_z(pos.z() + vel.z() * dt);

        info!("{},{}",data.id(), pos);

        // 同じ変数に格納するのではなく、別変数（今回の時間の位置という意味）にする
        data.set_post_pos(&pos);

        // とりあえず現在の速度を、変化なしで更新後の速度とする
        data.set_post_vel(&vel);

    }

}