mod test_mod;
mod core;

use std::process;

fn main() {
    println!("Hello, world!");

    // モジュールの実装テスト
    test_mod::foo::foo_func1();
    test_mod::foo::foo_func2();
    test_mod::bar::bar_func();

    // csv読込
    let filename = "data.csv";
    if let Err(err) = test_mod::csv_read::csv_read(filename) {
        println!("error runninng read: {}", err);
        process::exit(-1);
    }

    // Point構造体のテスト
    core::point::point_func();

    // let mut pp = core::point::Point { pos: [0.0, 0.0, 0.0] };
    // let mut pp = core::point::Point { pos: [0.0; 3] };
    // let mut pp : core::point::Point = Default::default();    // Default deriveを使う場合
    let mut pp = core::point::Point::new();
    let mut pp2nd = core::point::Point::new();

    pp.set_x(4.0);
    pp.set_y(3.0);
    pp.set_z(2.0);
    pp2nd.set_y(11.1);

    pp.print_val();
    pp2nd.print_val();
    /*
    let max_n = 2;
    for n in 0..=max_n {
        println!("{}", pp.pos[ n ]);
    }
    */

    let mut vv = core::dcont::Dcont::new();
    // 以下はidxをpubしないと使えない
    // vv.idx.push(0.0);
    // vv.idx.push(1.0);
    // println!("{:?}", vv.idx);
    
    // let filename = &String::from("./data.csv");
    // let filename = "data.csv";   // ２重定義に見えるが、Rustではlifeの考え方があるので、これで構わない
    if let Err(err) = vv.read(filename) {
        println!("error runninng read: {}", err);
        process::exit(-1);
    }

    let val = 3.1;
    println!("hokan = {}, {}", val, vv.get_value(val));
    let val = 3.5;
    println!("hokan = {}, {}", val, vv.get_value(val));

    let mut vw = core::tcont::Tcont::new();
    let filename_t = "dataT.csv";
    if let Err(err) = vw.read(filename_t) {
        println!("error runninng readT: {}", err);
        process::exit(-1);
    }
    vw.output();

    let val1 = 10.0;
    let val2 = 10.0;
    println!("Thokan = {},{}, {}", val1, val2, vw.get_value(val1, val2));

    // 積分のテスト
    println!("積分のテスト");
    let mut int = core::etc::Integrator1::new(1.0);
    println!("{}", int.get(1.0));
    println!("{}", int.get(1.0));

    // Quartenionのテスト
    let q1 = core::quartenion::Quartenion::new();
    q1.output();

}
