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
    // pub mat: Array2<f64> // テスト用
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
            // mat: ndarray::arr2( &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0] ])   // テスト用
        }
    }

    // 初期化
    // phi, theta, psi [rad]
    pub fn initialize(&mut self, phi:f64, theta:f64, psi:f64 ) {
        let cos_psi2 = ( psi / 2.0 ).cos();
        let sin_psi2 = ( psi / 2.0 ).sin();
        let cos_theta2 = ( theta / 2.0 ).cos();
        let sin_theta2 = ( theta / 2.0 ).sin();
        let cos_phi2 = ( phi / 2.0 ).cos();
        let sin_phi2 = ( phi / 2.0 ).sin();

        self.e0 = cos_psi2*cos_theta2*cos_phi2 + sin_psi2*sin_theta2*sin_phi2;
        self.e1 = cos_psi2*cos_theta2*sin_phi2 - sin_psi2*sin_theta2*cos_phi2;
        self.e2 = cos_psi2*sin_theta2*cos_phi2 + sin_psi2*cos_theta2*sin_phi2;
        self.e3 = -cos_psi2*sin_theta2*sin_phi2 + sin_psi2*cos_theta2*cos_phi2;
    
        self.e3DotPre = 0.0;
        self.e2DotPre = 0.0;
        self.e1DotPre = 0.0;
        self.e0DotPre = 0.0;
    
        self.update_euler_matrix();
    }

    // 行列の更新
    pub fn update_euler_matrix(&mut self) {
        let e0e0 = self.e0 * self.e0;
        let e1e1 = self.e1 * self.e1;
        let e2e2 = self.e2 * self.e2;
        let e3e3 = self.e3 * self.e3;
    
        let e0e1 = self.e0 * self.e1;
        let e0e2 = self.e0 * self.e2;
        let e0e3 = self.e0 * self.e3;
        let e1e2 = self.e1 * self.e2;
        let e1e3 = self.e1 * self.e3;
        let e2e3 = self.e2 * self.e3;
    
        self.mat[[0,0]] = e0e0 + e1e1 - e2e2 - e3e3;    // l1
        self.mat[[0,1]] = 2.0 * (e1e2 - e0e3); // l2
        self.mat[[0,2]] = 2.0 * (e0e2 + e1e3); // l3
        self.mat[[1,0]] = 2.0 * (e0e3 + e1e2); // m1
        self.mat[[1,1]] = e0e0 - e1e1 + e2e2 - e3e3;   // m2
        self.mat[[1,2]] = 2.0 * (e2e3 - e0e1); // m3
        self.mat[[2,0]] = 2.0 * (e1e3 - e0e2); // n1
        self.mat[[2,1]] = 2.0 * (e0e1 + e2e3); // n2
        self.mat[[2,2]] = e0e0 - e1e1 - e2e2 + e3e3;   // n3
    
    }

    // debug用出力
    pub fn output(&self) {
        println!("in quartenion :");
        println!("{:?}", self.mat);
    }
}
