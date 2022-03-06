// model air型
use super::basemodel;
use super::super::core;

pub struct AirModel {
    pub base: basemodel::BaseModel,
}

impl AirModel {
    pub fn new() -> AirModel {
        AirModel{
            // base: basemodel::BaseModel::new(num, sss),
            base: basemodel::BaseModel::new(),
        }
    }
    
}

impl basemodel::Model for AirModel {
    fn print_own(&self) {
        println!("in AirModel");
    }

    /**
     * AirModelのset_up関数
     */
    fn set_up(&mut self) {
        // idの設定
        self.base.set_id(101);
    }

    /**
     * オブジェクトの更新処理
     * 
     * @param[in/out] data  オブジェクト・データ
     * @param[in] env   環境データ
     */
    fn update(&self, data: &mut basemodel::BaseModel, env:&core::env::Env, dt:f64) {

    }

}