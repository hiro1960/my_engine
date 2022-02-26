// model land型
use super::basemodel;

pub struct LandModel {
    pub base: basemodel::BaseModel,
}

impl LandModel {
    pub fn new(num:i32, sss:&str) -> LandModel {
        LandModel{
            base: basemodel::BaseModel::new(num, sss),
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

}