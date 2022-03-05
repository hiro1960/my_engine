// シミュレーション・データの定義
use super::super::core;

/**
 * 時間管理データ
 */
#[derive(Default)]   // Default::default()が使えるようになる
pub struct SimCtrl {
    count: i64,
    delta_time: f64,  // [s]
}

/**
 * 時間管理データ　操作関数
 */
impl SimCtrl {
    /**
     * データ設定
     * @param[in] ct 繰り返し回数
     * @param[in] dt 刻み値[s]
     */
    pub fn set(&mut self, ct: i64, dt: f64) {
        self.count = ct;
        self.delta_time = dt;
    }

    /**
     * countの取得
     * 
     * @return count
     */
    pub fn count(&self) -> i64 {
        self.count
    }

    /**
     * delta_timeの取得
     * 
     * @return delta_time
     */
    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

}

/**
 * オブジェクト・データ
 */
pub struct SimObject {
    id: i64,
    name: String,
    category: String
}

/**
 * オブジェクト・データ　操作関数
 */
impl SimObject {
    /**
     * データ設定
     * @param[in] id
     * @param[in] name
     * @param[in] category
     */
    pub fn set(&mut self, id:i64, name:&str, category:&str) {
        self.id = id;
        self.name = name.to_string();
        self.category = category.to_string();
    }
}

/**
 * シミュレーション・データ本体
 */
pub struct SimData {
    // 時間管理データ
    pub time_set: SimCtrl,

    // →処理に必要なデータを集めておく(Rustはグローバル変数を使わせてくれない)
    // 環境データ
    pub env: core::env::Env,

    // オブジェクト・データ
    pub object_db: Vec<SimObject>

}

/**
 * シミュレーション・データ操作関数
 */
impl SimData {
    // データ生成
    pub fn new() -> SimData {
        // 時間管理データ
        SimData{ 
            time_set: SimCtrl{ count:0, delta_time:0.0},
            env: core::env::Env::new(),
            object_db: Vec::new()
        }
    }
}