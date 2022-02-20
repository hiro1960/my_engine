// model air型
use super::basemodel;

pub struct AirModel {
    pub base: basemodel::BaseModel,
}

impl AirModel {
    pub fn new(num:i32, sss:&str) -> AirModel {
        AirModel{
            base: basemodel::BaseModel::new(num, sss),
        }
    }
    
}

impl basemodel::Model for AirModel {
    fn print_own(&self) {
        println!("in AirModel");
    }

}