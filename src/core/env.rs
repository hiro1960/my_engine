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
    g: f64,         // 万有引力定数[m3/kg/s2]
    gm: f64,        // 地心重力定数[m3/s2]
    sea_level_gravity: f64,     // 重力加速度[m/s2]
    pub static_pressure: core::dcont::Dcont,    // 静圧
    pub sound_velocity: core::dcont::Dcont,     // 音速
}

impl Env {
    pub fn new() -> Env {
        // 初期値
        Env{
            radius: 6371012.0,
            g: 6.672e-11,
            gm: 3.986005e+14,
            sea_level_gravity: 9.8065,
            static_pressure: core::dcont::Dcont::new(),
            sound_velocity: core::dcont::Dcont::new(),
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
     * 
     * @param[in] filename 定数のファイル名（$PROJECT_TOP/data以下に存在）
     */
    pub fn set_up(&mut self, filename: &str) {
        // jsonを読み込むデータ型
        // struct Envs {
        //     envs: HashMap<String, String>,
        // }

        // let file = File::open(env_file).expect("environment file not exist");   // env_fileのlifeは、File::open()に持っていかれるのに注意
        // let reader = BufReader::new(file);
        // let deserialized: Envs = serde_json::from_reader(reader).unwrap();
        // 構造体で読み込む方法は定型のフォーマットに使う。任意の形式には使えない。

        let project_data = env::var("PROJECT_TOP").expect("PROJECT_TOP is not defined") + "/" + &core::etc::data_dir();
        let env_file = project_data + "/" + filename;

        let contents = fs::read_to_string(env_file).unwrap();
        let vv: Value = serde_json::from_str(&contents).unwrap();

        // 地球半径
        let vs = &vv["radius"];
        self.radius = vs.to_string().parse().expect("in Env::set_up(), radius expect floating-point!");
        println!("in Env, radius = {}", self.radius);

        // 万有引力定数
        let vs = &vv["G"];
        self.g = vs.to_string().parse().expect("in Env::set_up(), G expect floating-point!");
        println!("in Env, G = {}", self.g);

        // 地心重力定数
        let vs = &vv["GM"];
        self.gm = vs.to_string().parse().expect("in Env::set_up(), GM expect floating-point!");
        println!("in Env, GM = {}", self.gm);

        // 重力加速度
        let vs = &vv["SeaLevelGravity"];
        self.sea_level_gravity = vs.to_string().parse().expect("in Env::set_up(), SeaLevelGravity expect floating-point!");
        println!("in Env, SeaLevelGravity = {}", self.sea_level_gravity);
        
        // 静圧
        let vs = &vv["StaticPressure"];
        // let vs = vv.get("StaticPressure");
        // let csvfile: String = vs.to_string().parse().unwrap();  //NG　文字列の先頭、末尾に「"」が付いてしまう。
        let csvfile = vs.as_str().expect("in Env::set_up(), StaticPressure expect file-name!"); // &str型になる

        println!("in Env, StaticPressure = {}", csvfile);

        let project_data = env::var("PROJECT_TOP").expect("PROJECT_TOP is not defined") + "/" + &core::etc::data_dir();
        let static_file = project_data + "/" + csvfile;
        self.static_pressure.read(&static_file).unwrap();

        // 音速
        let vs = &vv["SoundVelocity"];
        let csvfile = vs.as_str().expect("in Env::set_up(), SoundVelocity expect file-name!"); // &str型になる
        println!("in Env, SoundVelocity = {}", csvfile);
        let project_data = env::var("PROJECT_TOP").expect("PROJECT_TOP is not defined") + "/" + &core::etc::data_dir();
        let csv_file = project_data + "/" + csvfile;
        self.sound_velocity.read(&csv_file).unwrap();



    }

}