// 3変数補間オブジェクト
use std::fs::File;
use std::error::Error;

// 同じ位置のmoduleの参照にはuseを使用する
use super::etc;

// 補間テーブル・データ
#[derive(Default)]
pub struct Tcont {
    // データ自体は外部から見えない様にしておく
    idx1    : Vec<f64>,     // index1
    idx2    : Vec<f64>,     // index2
    num_of_tag1 : usize,
    num_of_tag2 : usize,
    val     : Vec<f64>,     // value
}

impl Tcont {
    pub fn new() -> Tcont {
        // 初期値
        Tcont{ idx1: Vec::new(), idx2: Vec::new(), num_of_tag1: 0, num_of_tag2: 0, val: Vec::new()}
    }

    // debug用出力
    pub fn output(&self) {
        println!("in tcont, {:?}", self.idx1);
        println!("in tcont, {:?}", self.idx2);
        println!("in tcont, {:?}", self.num_of_tag1);
        println!("in tcont, {:?}", self.num_of_tag2);
        println!("in tcont, {:?}", self.val);
    }

    /**
     * containerへデータの読込
     * @param[in] file_name 3次元テーブル（csv）ファイル名
     * @return 読み込み結果
     */
    pub fn read(&mut self, file_name :&str) -> Result<(), Box<dyn Error>>{
        println!("in Tcont.read, {}", file_name);

        // 最初にメンバに値を空にする
        self.idx1.clear();
        self.idx2.clear();
        self.val.clear();

        let file = File::open(file_name).expect("file not found");
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.records() {
            let record = result?;
            // println!("{:?}", record);
            self.idx1.push( record[0].parse().unwrap() );
            self.idx2.push( record[1].parse().unwrap() );
            self.val.push( record[2].parse().unwrap() );
        }

        // idx2の数を数える
        let mut cnt = 0;
        let t1 = self.idx1[cnt];
        self.num_of_tag2 = 0;
        loop {
            if t1 != self.idx1[cnt] {
                break;
            }
            cnt += 1;
            self.num_of_tag2 += 1;
        }

        // idx1の数を数える
        cnt = 0;
        self.num_of_tag1 = 1;
        let mut t2 = self.idx2[0];
        loop {
            if t2 != self.idx1[cnt] {
                self.num_of_tag1 += 1;
            }
            t2 = self.idx1[cnt];
            cnt += 1;

            if cnt == self.idx1.len() {
                break;
            }
        }

        Ok(())
    
    }

    /**
     * 補間
     * @param[in] v1 第1引数
     * @param[in] v2 第2引数
     * @return 補間結果
     */
    pub fn get_value(&self, v1: f64, v2: f64) -> f64 {
        // 初期値の設定
        let mut index1 = v1;
        let mut index2 = v2;

        // index1の下限チェック
        if v1 <= self.idx1[0] {
            index1 = self.idx1[0];
        }
        // index1の上限のチェック
        if v1 >= self.idx1[self.idx1.len()-1] {
            index1 = self.idx1[self.idx1.len()-1];
        }
        // index2の下限チェック
        if v2 <= self.idx2[0] {
            index2 = self.idx2[0];
        }
        // index2の上限チェック
        if v2 >= self.idx2[self.idx2.len()-1] {
            index2 = self.idx2[self.idx2.len()-1];
        }

        // 初期値
        let mut idx1_l = self.idx1[0];   // index1 下の値
        let mut idx1_lcnt = 0;  // idx1L1の位置

        let mut i = 0;  // usize型
        while i < self.idx1.len() {
            if index1 < self.idx1[i] {
                break;
            }
            idx1_l = self.idx1[i];
            idx1_lcnt = i;

            i += self.num_of_tag1 as usize;
        }
        // println!("idx1_l = {}, idx1_lcnt = {}", idx1_l, idx1_lcnt); // デバッグ用出力

        // 初期値
        let mut idx1_u = self.idx1[self.idx1.len()-1];
        let mut idx1_ucnt = self.idx1.len() - self.num_of_tag2;

        i = idx1_ucnt;
        loop {
            if index1 >= self.idx1[i] {
                break;
            }
            idx1_u = self.idx1[i];
            idx1_ucnt = i;

            if i == 0 {
                break;  // loop終了
            }
            i -= self.num_of_tag2;
        }
        // println!("idx1_u = {}, idx1_ucnt = {}", idx1_u, idx1_ucnt); // デバッグ用出力

        // 初期値
        let mut idx2_l = self.idx2[idx1_lcnt];
        let mut idx2_lcnt = 0;
        i = idx1_lcnt;
        while i < idx1_lcnt + self.num_of_tag2 {
            if index2 < self.idx2[i] {
                break;
            }
            idx2_l = self.idx2[i];
            idx2_lcnt = i - idx1_lcnt;

            i += 1;
        }
        // println!("idx2_l = {}, idx2_lcnt = {}", idx2_l, idx2_lcnt); // デバッグ用出力

        // 初期値
        let mut idx2_u = self.idx2[idx1_lcnt + self.num_of_tag2];
        let mut idx2_ucnt = self.num_of_tag2 - 1;
        i = idx1_lcnt + self.num_of_tag2 - 1;
        while i >= idx1_lcnt {
            if index2 >= self.idx2[i] {
                break;
            }
            idx2_u = self.idx2[i];
            idx2_ucnt = i - idx1_lcnt;

            i -= 1;
        }
        // println!("idx2_u = {}, idx2_ucnt = {}", idx2_u, idx2_ucnt); // デバッグ用出力

        let temp_l = etc::hokan(index2, idx2_l, idx2_u,
             self.val[idx1_lcnt + idx2_lcnt], self.val[idx1_lcnt + idx2_ucnt]);

        let temp_u = etc::hokan(index2, idx2_l, idx2_u,
             self.val[idx1_ucnt + idx2_lcnt], self.val[idx1_ucnt + idx2_ucnt]);

        let r = etc::hokan(index1, idx1_l, idx1_u, temp_l, temp_u);

        return r;
    }

}