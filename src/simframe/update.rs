// 更新処理

use super::super::simframe;
use super::super::model;

// trait Modelの参照に必要
use crate::model::basemodel::Model;


pub fn update(db: &mut simframe::sim_data::SimData) {
    // シミュレーション制御情報取得
    let ct = db.time_set.count();   // 繰り返し回数
    let delta_time = db.time_set.delta_time();  // 刻み時間[s]
    let num = db.object_db.len();   // オブジェクト数

    // 更新用の変数
    let land = model::land::LandModel::new();
    let air = model::air::AirModel::new();

    for i in 0..ct {
        println!("in update, {}", i);
        for n in 0..num {
            match db.object_db[n].category() {
                // Match "Air"
                "Air" => air.update(&mut db.object_db[n], &db.env, delta_time),
                // Match "Land"
                "Land" => land.update(&mut db.object_db[n], &db.env, delta_time),
                // Handle the rest of case
                _=> println!("## no category {} ##", db.object_db[n].category()),
            }

            println!("{} {}", db.object_db[n].id(), db.object_db[n].pos());
        }
    }

}