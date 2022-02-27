// 初期化モジュール
use super::super::simframe;

// SimData型の初期化関数
/**
 * SimData型の初期化関数
 * 
 * @param[in/out] db シミュレーション・データ
 */
pub fn initialize(db: &mut simframe::simData::SimData) {
    // 環境データの初期化
    db.env.set_up("environment.json");

}