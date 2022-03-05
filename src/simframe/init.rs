// 初期化モジュール
use super::super::simframe;
use super::super::core;
use std::env;
use std::fs;
use serde_json::{Result, Value};

// SimData型の初期化関数
/**
 * SimData型の初期化関数
 * 
 * @param[in/out] db シミュレーション・データ
 */
pub fn initialize(db: &mut simframe::sim_data::SimData) {
    // 環境データの初期化
    db.env.set_up("environment.json");

    let project_data = env::var("PROJECT_TOP").expect("PROJECT_TOP is not defined") + "/" + &core::etc::data_dir();
    let env_file = project_data + "/scenario.json";

    let contents = fs::read_to_string(env_file).unwrap();
    let vv: Value = serde_json::from_str(&contents).unwrap();

    println!("in initialize() , {}", vv);

    // SimCtrl　シミュレーション制御情報セット
    let vs = &vv["SimCtrl"];
    println!("SimCtrl, {}", vs);
    let sim_ctrl: String = vs.to_string().parse().unwrap();
    let v2: Value = serde_json::from_str(&sim_ctrl).unwrap();
    let ct_value = &v2["Count"];
    let ct: i64 =  ct_value.to_string().parse().unwrap();
    let dt_value = &v2["DeltaTime"];
    let dt: f64 = dt_value.to_string().parse().unwrap();
    db.time_set.set(ct, dt);


    println!("test");

}