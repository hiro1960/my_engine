// 別ディレクトリに定義してあるモジュールを使うことを宣言
use super::super::core;
use super::super::model;
use super::super::simframe;

// trait Modelの参照に必要
use crate::model::basemodel::Model;

pub fn foo_func1() {
    println!("Foo 1!");
}

pub fn foo_func2() {
    println!("Foo 2!");
}

pub fn integ_test() {
        // 積分のテスト
        println!("### 積分のテスト");
        let mut int = core::etc::Integrator1::new(1.0, 1.0);
        let mut work = 1.0;
        work = int.integral(0.1);
        println!("{}", work);
        work = int.integral(0.1);
        println!("{}", work);
        println!(" val = {}", int.val());
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
    dest_pt = q1.euler_trans(&src_pt);
    q1.output();
    println!(" trans result ");
    dest_pt.print_val();

    dest_pt = q1.euler_trans_inv(&src_pt);
    println!(" inv result ");
    dest_pt.print_val();

}

pub fn model_test() {
    // BaseModelのテスト
    println!("### BaseModelのテスト");
    
    // let mut m1 = model::basemodel::BaseModel::new(1, "Tank");
    let mut m1 = model::basemodel::BaseModel::new();
    m1.set_id(1);
    m1.set_name("Tank");
    println!("{}, {} !", m1.id(), m1.name());
    m1.set_id(21);
    m1.set_name("Ships");
    println!("{}, {} !", m1.id(), m1.name());

    m1.pos().set_x(1.0);    // Copyされたposに対する処理なので、m1.posには値が反映されにのに注意
    m1.pos().set_y(2.0);
    m1.pos().set_z(3.0);
    m1.pos().print_val();

    println!("##### AirModelのテスト");
    // let mut m2 = model::air::AirModel::new(2, "Swallow");
    let mut m2 = model::air::AirModel::new();
    m2.base.set_id(2);
    m2.base.set_name("Swallow");
    println!("{}, {} !", m2.base.id(), m2.base.name());
    m2.base.set_id(34);
    m2.print_own(); // traitで定義した共通関数
    m2.set_up();
    let pos = core::point::Point::new_pt( 1.0, 2.0, 3.0 );  // 仮の位置変数
    m2.base.set_pos( &pos );
    println!("{}, {}, {} !", m2.base.id(), m2.base.name(), m2.base.pos() );
    // m2.base.pos.print_val();

    println!("##### LandModelのテスト");
    // let mut m3 = model::land::LandModel::new(3, "Cat");
    let mut m3 = model::land::LandModel::new();
    m3.base.set_id(3);
    m3.base.set_name("Cat");
    println!("{}, {} !", m3.base.id(), m3.base.name());
    m3.base.set_id(42);
    m3.print_own(); // traitで定義した共通関数
    m3.base.set_pos( &pos );
    println!("{}, {}, {} !", m3.base.id(), m3.base.name(), m3.base.pos() );
    // m3.base.pos.print_val();
    
}

pub fn env_test() {
    // Envのテスト
    println!("### Envのテスト");
    let mut env = core::env::Env::new();

    println!("radius = {}", env.radius());

    env.set_up("environment.json");

    println!("static pressure test ={}", env.static_pressure.get_value(66.0));
}

pub fn simframe_test() {

    let x = 5;
    // 出力レベル順（下に行くほど、情報量が増える）
    error!("Bright red error {}", x);
    warn!("warn");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
    trace!("trace");

    let mut sim_db = simframe::sim_data::SimData::new();
    sim_db.time_set.set(10, 0.1);

    simframe::init::initialize(&mut sim_db);

    // 読み込んだ結果の確認（initialize()内でsim_dbにデータを設定している）
    println!("in simframe_test(), SimCtrl {} {}", sim_db.time_set.count(), sim_db.time_set.delta_time());

    for i in 0..sim_db.object_db.len() {
        println!("{} {} {}", sim_db.object_db[i].id(), sim_db.object_db[i].name(), sim_db.object_db[i].category() );
        println!("{}, {}", sim_db.object_db[i].pos(), sim_db.object_db[i].vel() );
    }
    
    simframe::update::update(&mut sim_db);
    
}