/*
    quartenionモジュール

    [dependencies]
    ndarray="0.15.0"
*/

use ndarray::prelude::*;

pub struct Quartenion {
    e0: f64,
    e1: f64,
    e2: f64,
    e3: f64,

    e0DotPre: f64,
    e1DotPre: f64,
    e2DotPre: f64,
    e3DotPre: f64,

    // mat: Array2<[[f64; 3]; 3]>   // ここでサイズ指定までしてしまうと、初期化がうまくいかない
    mat: Array2<f64>
}

impl Quartenion {
    pub fn new() -> Quartenion {
        // 初期値
        Quartenion{ 
            e0: 0.0, e1:0.0, e2:0.0, e3:0.0,
            e0DotPre: 0.0,
            e1DotPre: 0.0,
            e2DotPre: 0.0,
            e3DotPre: 0.0,
            // 行列のサイズは初期化で指定する
            mat: ndarray::arr2( &[[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0] ])
        }
    }

    // debug用出力
    pub fn output(&self) {
        println!("in quartenion, {:?}", self.mat);
    }
}
