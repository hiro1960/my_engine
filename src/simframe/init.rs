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

    println!("in initialize()");
}