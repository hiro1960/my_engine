use std::fmt;

pub fn point_func() {
    println!("point!");
}
/*
    Pointモジュール
    位置、速度の３次元ベクトルを表すのに使う
*/
// #[derive(Default)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pos : [f64; 3]  // 3次元の位置[m]
}


impl Point {
    // Defaultを使わない場合は、new()関数を使う
    pub fn new() -> Point {
        // 初期値の定義
        Point { pos: [0.0; 3] }
    }

    /**
     * 初期値設定こみのnew
     * 
     * @param[in] x
     * @param[in] y
     * @param[in] z
     * @return Point
     */
    pub fn new_pt(x:f64, y:f64, z:f64) -> Point {
        // 引数を初期値にしてPointを生成
        Point{ pos: [x, y, z] }
    }

    // x座標に値をセット
    pub fn set_x(&mut self, val: f64) {
        self.pos[0] = val;
    }

    // y座標に値をセット
    pub fn set_y(&mut self, val: f64) {
        self.pos[1] = val;
    }

    // z座標に値をセット
    pub fn set_z(&mut self, val: f64) {
        self.pos[2] = val;
    }

    /**
     * x座標の値を取得
     * @return x座標
     */
    pub fn x(&self) -> f64 {
        return self.pos[0];
    }

    /**
     * y座標の値を取得
     * @return y座標
     */
    pub fn y(&self) -> f64 {
        return self.pos[1];
    }

    /**
     * z座標の値を取得
     * @return z座標
     */
    pub fn z(&self) -> f64 {
        return self.pos[2];
    }

    // DEBUG用
    // fmt::Displayの実装を作成したので、もう不要
    pub fn print_val(&self) {
        println!("( {}, {}, {} )", self.pos[0], self.pos[1], self.pos[2]);
    }

}

/**
 * 出力用関数の実装
 */
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.pos[0], self.pos[1], self.pos[2])
    }
}