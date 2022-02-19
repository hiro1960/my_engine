/*
    quartenionモジュール

    [dependencies]
    ndarray="0.15.0"
*/

use super::super::core; // この参照方式でないと、core内のmoduleを使えない
// use core;    // error
// mod core;   // error
// use self::core;  // error

use ndarray::prelude::*;

// #[derive(Default)]
pub struct Quartenion {
    // 積分用メンバー
    e0: core::etc::Integrator1,
    e1: core::etc::Integrator1,
    e2: core::etc::Integrator1,
    e3: core::etc::Integrator1,

    // mat: Array2<[[f64; 3]; 3]>   // ここでサイズ指定までしてしまうと、初期化がうまくいかない
    mat: Array2<f64>
    // pub mat: Array2<f64> // テスト用
}

impl Quartenion {
    pub fn new() -> Quartenion {
        // 初期値
        Quartenion{ 
            e0: core::etc::Integrator1::new(1.0, 0.1),  // とりあえず仮の値で初期化
            e1: core::etc::Integrator1::new(1.0, 0.1),  // とりあえず仮の値で初期化
            e2: core::etc::Integrator1::new(1.0, 0.1),  // とりあえず仮の値で初期化
            e3: core::etc::Integrator1::new(1.0, 0.1),  // とりあえず仮の値で初期化

            // 行列のサイズは初期化で指定する
            mat: ndarray::arr2( &[[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0] ])
            // mat: ndarray::arr2( &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0] ])   // テスト用
        }
    }

    // 初期化
    // param@[in] phi, theta, psi [rad]
    // param@[in] dtime [s]
    pub fn initialize(&mut self, phi:f64, theta:f64, psi:f64, dtime:f64 ) {
        let cos_psi2 = ( psi / 2.0 ).cos();
        let sin_psi2 = ( psi / 2.0 ).sin();
        let cos_theta2 = ( theta / 2.0 ).cos();
        let sin_theta2 = ( theta / 2.0 ).sin();
        let cos_phi2 = ( phi / 2.0 ).cos();
        let sin_phi2 = ( phi / 2.0 ).sin();

        let e0_temp = cos_psi2*cos_theta2*cos_phi2 + sin_psi2*sin_theta2*sin_phi2;
        let e1_temp = cos_psi2*cos_theta2*sin_phi2 - sin_psi2*sin_theta2*cos_phi2;
        let e2_temp = cos_psi2*sin_theta2*cos_phi2 + sin_psi2*cos_theta2*sin_phi2;
        let e3_temp = -cos_psi2*sin_theta2*sin_phi2 + sin_psi2*cos_theta2*cos_phi2;
    
        self.e0 = core::etc::Integrator1::new(e0_temp, dtime);
        self.e1 = core::etc::Integrator1::new(e1_temp, dtime);
        self.e2 = core::etc::Integrator1::new(e2_temp, dtime);
        self.e3 = core::etc::Integrator1::new(e3_temp, dtime);
    
        self.update_euler_matrix();
    }

    // 行列の更新
    pub fn update_euler_matrix(&mut self) {
        let e0e0 = self.e0.val() * self.e0.val();
        let e1e1 = self.e1.val() * self.e1.val();
        let e2e2 = self.e2.val() * self.e2.val();
        let e3e3 = self.e3.val() * self.e3.val();
    
        let e0e1 = self.e0.val() * self.e1.val();
        let e0e2 = self.e0.val() * self.e2.val();
        let e0e3 = self.e0.val() * self.e3.val();
        let e1e2 = self.e1.val() * self.e2.val();
        let e1e3 = self.e1.val() * self.e3.val();
        let e2e3 = self.e2.val() * self.e3.val();
    
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

    // quartenionの更新
    // param@[in] p, q, r [rad/s]
    pub fn update_quartenion(&mut self, p:f64, q:f64, r:f64) {
        let kq:f64 = 0.0;
        let keps:f64 = kq * (1.0 - (self.e0.val()*self.e0.val() + self.e1.val()*self.e1.val() + self.e2.val()*self.e2.val() + self.e3.val()*self.e3.val()));

        let e0_dot:f64 = -(self.e1.val()*p + self.e2.val()*q + self.e3.val()*r ) / 2.0 + keps*self.e0.val();
        let e1_dot:f64 = (self.e0.val()*p - self.e3.val()*q + self.e2.val()*r ) / 2.0 + keps*self.e1.val();
        let e2_dot:f64 = (self.e3.val()*p + self.e0.val()*q - self.e1.val()*r ) / 2.0 + keps*self.e2.val();
        let e3_dot:f64 = (-self.e2.val()*p + self.e1.val()*q + self.e0.val()*r ) / 2.0 + keps*self.e3.val();

        // 積分結果を返すが、必要ないので受けない
        self.e0.integral( e0_dot );
        self.e1.integral( e1_dot );
        self.e2.integral( e2_dot );
        self.e3.integral( e3_dot );

        let aa = (self.e0.val()*self.e0.val() + self.e1.val()*self.e1.val() + self.e2.val()*self.e2.val() + self.e3.val()*self.e3.val()).sqrt();

        self.e0.set_val( self.e0.val() / aa );
	    self.e1.set_val( self.e1.val() / aa );
	    self.e2.set_val( self.e2.val() / aa );
	    self.e3.set_val( self.e3.val() / aa );

        self.update_euler_matrix();
    }


    // オイラー変換
    // param@ org[in] Point型　位置ベクトル
    // return Point型　変換結果
    pub fn euler_trans(&self, org: &core::point::Point ) -> core::point::Point {
        let vec = arr1(&[org.x(), org.y(), org.z()]);
        let vec_r = vec.dot(&self.mat);
        let mut dest = core::point::Point::new();

        dest.set_x(vec_r[0]);
        dest.set_y(vec_r[1]);
        dest.set_z(vec_r[2]);

        return dest;
    }

    // 逆オイラー変換
    // param@ org[in] Point型　位置ベクトル
    // return Point型　変換結果
    pub fn euler_trans_inv(&self, org: &core::point::Point ) -> core::point::Point {
        let vec = arr1(&[org.x(), org.y(), org.z()]);
        let vec_r = self.mat.dot(&vec);
        let mut dest = core::point::Point::new();

        dest.set_x(vec_r[0]);
        dest.set_y(vec_r[1]);
        dest.set_z(vec_r[2]);

        return dest;
    }

    // debug用出力
    pub fn output(&self) {
        println!("in quartenion :");
        println!("  e0-3 : {} {} {} {}", self.e0.val(), self.e1.val(), self.e2.val(), self.e3.val());
        println!("{:?}", self.mat);
    }
}
