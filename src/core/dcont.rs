// 2変数補間オブジェクト
use std::fs::File;
use std::error::Error;

// 同じ位置のmoduleの参照にはuseを使用する
use super::etc;

// 補間テーブル・データ
pub struct Dcont {
    // データ自体は外部から見えない様にしておく
    idx : Vec<f64>,     // index
    val : Vec<f64>,     // value
}

impl Dcont {
    pub fn new() -> Dcont {
        // 初期値
        Dcont{ idx: Vec::new(), val: Vec::new()}
    }

    /**
     * containerへデータの読込
     * @param[in] file_name 2次元テーブル（csv）ファイル名
     * @return 読み込み結果
     */
    pub fn read(&mut self, file_name :&str) -> Result<(), Box<dyn Error>>{
        println!("in Dcont.read, {}", file_name);

        // 最初にメンバに値を空にする
        self.idx.clear();
        self.val.clear();

        let file = File::open(file_name).expect("file not found");
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.records() {
            let record = result?;
            // println!("{:?}", record);
            self.idx.push( record[0].parse().unwrap() );
            self.val.push( record[1].parse().unwrap() );
        }

        // 値の確認
        for i in 0..self.idx.len() {
            println!(" idx = {}, val = {}", self.idx[i], self.val[i]);
        }
    
        Ok(())
    }

    /**
     * 補間
     * @param[in] val 引数
     * @return 補間結果
     */
    pub fn get_value(&self, val: f64) -> f64 {

        if val <= self.idx[0] {
            // 下限のチェック
            // 補間テーブルの下限を超えている
            return self.val[0];
        }
        else if val >= self.idx[self.idx.len() - 1]
        {
            // 上限のチェック
            // 補間テーブルの上限を超えている
            return self.val[self.idx.len() - 1]
        }
        else
        {
            // idxのサーチ
            let mut l:usize = 0;
            for i in 0..self.idx.len() {
                // println!("in get_value, {} {} {} {}", i, val, self.idx[i], self.idx.len());
                if val < self.idx[i] {
                    // println!("in get_value find, {} {} {}", i, val, self.idx[i]);
                    l = i;
                    break;
                }
            }
            // println!("in get_value, for-out {}", l);

            // 値の補間
            return etc::hokan(val, self.idx[l-1], self.idx[l], self.val[l-1], self.val[l]);
        }

    }
}