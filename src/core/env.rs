// 自然環境に関わる定数を保持する

use std::env;

// use serde::Deserialize;
use std::fs;
use serde_json::{Result, Value};
// use std::io::BufReader;
// use std::collections::HashMap;

use super::super::core;

pub struct Env {
    radius: f64,    // 地球半径[m]
    static_pressure: core::dcont::Dcont,    // 静圧
}

impl Env {
    pub fn new() -> Env {
        // 初期値
        Env{
            radius: 6371012.0,  // 地球半径[m]
            static_pressure: core::dcont::Dcont::new(), // 静圧
        }
    }

    /**
     * 地球半径の取得
     * 
     * @return 地球半径[m]
     */
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /**
     * 自然環境の定数の初期化
     * $PROJECT_TOP/data以下の、filenameのファイルを読み込む
     */
    pub fn set_up(&mut self, filename: &str) {
        // jsonを読み込むデータ型
        // struct Envs {
        //     envs: HashMap<String, String>,
        // }

        let project_data = env::var("PROJECT_TOP").expect("PROJECT_TOP is not defined") + "/" + &core::etc::data_dir();
        let env_file = project_data + "/" + filename;

        // let file = File::open(env_file).expect("environment file not exist");   // env_fileのlifeは、File::open()に持っていかれるのに注意
        // let reader = BufReader::new(file);
        // let deserialized: Envs = serde_json::from_reader(reader).unwrap();

        let contents = fs::read_to_string(env_file).unwrap();
        let vv: Value = serde_json::from_str(&contents).unwrap();

        let vs = &vv["radius"];
        self.radius = vs.to_string().parse().unwrap();
        println!("in Env, radius = {}", self.radius);

        let vs = &vv["StaticPressure"];
        // let vs = vv.get("StaticPressure");
        // let csvfile: String = vs.to_string().parse().unwrap();  //NG
        let csvfile = vs.as_str().unwrap();

        // println!("in Env, StaticPressure = {}", vs);
        println!("in Env, StaticPressure = {}", csvfile);

        let project_data = env::var("PROJECT_TOP").expect("PROJECT_TOP is not defined") + "/" + &core::etc::data_dir();
        let name = "static.csv";
        println!("in Env, StaticPressure = {}", name);
        // let static_file = project_data + "/" + name;
        let static_file = project_data + "/" + csvfile;
        self.static_pressure.read(&static_file);



    }

}