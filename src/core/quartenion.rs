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
    e0: f64,
    e1: f64,
    e2: f64,
    e3: f64,

    // quartenion更新刻み[s]
    dtime: f64,

    // 積分用メンバー
    eo_intg: core::etc::Integrator1,
    e1_intg: core::etc::Integrator1,
    e2_intg: core::etc::Integrator1,
    e3_intg: core::etc::Integrator1,

    // mat: Array2<[[f64; 3]; 3]>   // ここでサイズ指定までしてしまうと、初期化がうまくいかない
    mat: Array2<f64>
    // pub mat: Array2<f64> // テスト用
}

impl Quartenion {
    pub fn new() -> Quartenion {
        // 初期値
        Quartenion{ 
            e0: 0.0, e1:0.0, e2:0.0, e3:0.0,

            dtime: 0.0,

            eo_intg: core::etc::Integrator1::new(1.0),  // とりあえず仮の値で初期化
            e1_intg: core::etc::Integrator1::new(1.0),  // とりあえず仮の値で初期化
            e2_intg: core::etc::Integrator1::new(1.0),  // とりあえず仮の値で初期化
            e3_intg: core::etc::Integrator1::new(1.0),  // とりあえず仮の値で初期化

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

        self.e0 = cos_psi2*cos_theta2*cos_phi2 + sin_psi2*sin_theta2*sin_phi2;
        self.e1 = cos_psi2*cos_theta2*sin_phi2 - sin_psi2*sin_theta2*cos_phi2;
        self.e2 = cos_psi2*sin_theta2*cos_phi2 + sin_psi2*cos_theta2*sin_phi2;
        self.e3 = -cos_psi2*sin_theta2*sin_phi2 + sin_psi2*cos_theta2*cos_phi2;
    
        self.dtime = dtime;

        self.eo_intg = core::etc::Integrator1::new(dtime);
        self.e1_intg = core::etc::Integrator1::new(dtime);
        self.e2_intg = core::etc::Integrator1::new(dtime);
        self.e3_intg = core::etc::Integrator1::new(dtime);
    
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

    // quartenionの更新
    // param@[in] p, q, r [rad/s]
    pub fn update_quartenion(&mut self, p:f64, q:f64, r:f64) {
        let kq:f64 = 0.0;
        let keps:f64 = kq * (1.0 - (self.e0*self.e0 + self.e1*self.e1 + self.e2*self.e2 + self.e3*self.e3));

        let e0_dot:f64 = -(self.e1*p+self.e2*q+self.e3*r)/2.0 + keps*self.e0;
        let e1_dot:f64 = (self.e0*p-self.e3*q+self.e2*r)/2.0 + keps*self.e1;
        let e2_dot:f64 = (self.e3*p+self.e0*q-self.e1*r)/2.0 + keps*self.e2;
        let e3_dot:f64 = (-self.e2*p+self.e1*q+self.e0*r)/2.0 + keps*self.e3;

        self.e0 = self.eo_intg.get( self.e0, e0_dot );
        self.e1 = self.e1_intg.get( self.e1, e1_dot );
        self.e2 = self.e2_intg.get( self.e2, e2_dot );
        self.e3 = self.e3_intg.get( self.e3, e3_dot );

        let aa = (self.e0*self.e0 + self.e1*self.e1 + self.e2*self.e2 + self.e3*self.e3).sqrt();

        self.e0 = self.e0 / aa;
	    self.e1 = self.e1 / aa;
	    self.e2 = self.e2 / aa;
	    self.e3 = self.e3 / aa;

        self.update_euler_matrix();
    }

// func (v *Quartenion) UpdateQuartenion(p, q, r, dtime float64) {
// 	var Kq float64
// 	Kq = 0.0

// 	Keps := Kq * (1.0 - (v.e0*v.e0 + v.e1*v.e1 + v.e2*v.e2 + v.e3*v.e3))

// 	e0_dot := -(v.e1*p+v.e2*q+v.e3*r)/2.0 + Keps*v.e0
// 	e1_dot := (v.e0*p-v.e3*q+v.e2*r)/2.0 + Keps*v.e1
// 	e2_dot := (v.e3*p+v.e0*q-v.e1*r)/2.0 + Keps*v.e2
// 	e3_dot := (-v.e2*p+v.e1*q+v.e0*r)/2.0 + Keps*v.e3

// 	IntegImp(e0_dot, dtime, &v.eo_intg, &v.e0)
// 	IntegImp(e1_dot, dtime, &v.e1_intg, &v.e1)
// 	IntegImp(e2_dot, dtime, &v.e2_intg, &v.e2)
// 	IntegImp(e3_dot, dtime, &v.e3_intg, &v.e3)

// 	aa := math.Sqrt(v.e0*v.e0 + v.e1*v.e1 + v.e2*v.e2 + v.e3*v.e3)
// 	v.e0 = v.e0 / aa
// 	v.e1 = v.e1 / aa
// 	v.e2 = v.e2 / aa
// 	v.e3 = v.e3 / aa

// 	v.UpdateEulerMatrix()
// }



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
        println!("  e0-3 : {} {} {} {}", self.e0, self.e1, self.e2, self.e3);
        println!("{:?}", self.mat);
    }
}
