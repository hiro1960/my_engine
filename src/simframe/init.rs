// 初期化モジュール
use super::super::simframe;
use super::super::core;
use super::super::model;

use std::env;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
struct ScenarioStr {
    id: i64,
    name: String,
    category: String,
    // pos: Vec[f64]    // 複雑な構造はDeserializeできない
    pos: String
}

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

    // Scenario シナリオ部読み込み
    let vsc = &vv["Scenario"];
    println!("Scenario, {}", vsc);
    // let v3: String = vsc.to_string().parse().unwrap();
    let sce_str: String = vsc.to_string().parse().unwrap();
    let v3: Vec<ScenarioStr> = serde_json::from_str(&sce_str).unwrap();

    // println!("test {}", v3.len());

    for i in 0..v3.len() {
        // println!("id {}", v3[i].id);
        // let mut data = simframe::sim_data::SimObject::new();
        let mut data = model::basemodel::BaseModel::new();
        // data.set( v3[i].id, &v3[i].name, &v3[i].category );
        data.set_id(v3[i].id);
        data.set_name(&v3[i].name);
        data.set_category(&v3[i].category);

        let pos: Vec<&str> = v3[i].pos.split(',').collect();

        let x: f64 = pos[0].parse().unwrap();
        let y: f64 = pos[1].parse().unwrap();
        let z: f64 = pos[2].parse().unwrap();

        let pt = core::point::Point::new_pt(x, y, z);
        data.set_pos(&pt);
        db.object_db.push(data);
    }

    // 読み込んだ結果の確認
    for i in 0..db.object_db.len() {
        println!("{} {} {}", db.object_db[i].id, db.object_db[i].name, db.object_db[i].category );
    }

}