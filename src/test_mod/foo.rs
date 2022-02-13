// 別ディレクトリに定義してあるcoreモジュールを使うことを宣言
use super::super::core;

pub fn foo_func1() {
    println!("Foo 1!");
}

pub fn foo_func2() {
    println!("Foo 2!");
}

pub fn integ_test() {
        // 積分のテスト
        println!("### 積分のテスト");
        let mut int = core::etc::Integrator1::new(1.0);
        println!("{}", int.get(1.0));
        println!("{}", int.get(1.0));    
}

pub fn quart_test() {
        // Quartenionのテスト
        println!("### Quartenionのテスト");
        let mut q1 = core::quartenion::Quartenion::new();
        // q1.initialize(0.1, 0.1, 0.1, 0.1);
        q1.initialize(0.1, 0.1, 0.1, 0.1);
        q1.output();

        // println!("{}", q1.mat[[0, 0]]);
        // println!("{}", q1.mat[[0, 1]]);
        // println!("{}", q1.mat[[2, 1]]);

        let mut src_pt = core::point::Point::new();
        src_pt.set_x(1.0);
        src_pt.set_y(0.0);
        src_pt.set_z(0.0);
        let mut dest_pt = core::point::Point::new();

        dest_pt = q1.euler_trans(&src_pt);
        println!(" trans result ");
        dest_pt.print_val();

        q1.update_quartenion(0.01, 0.01, 0.01);
        q1.output();
        println!(" trans result ");
        dest_pt.print_val();

        dest_pt = q1.euler_trans_inv(&src_pt);
        println!(" inv result ");
        dest_pt.print_val();

    }