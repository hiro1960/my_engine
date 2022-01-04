// useやmodは基本、直下を参照する。useだけ親（同位置）、先祖の子モジュールを参照できる。
use super::super::core;

pub fn bar_func() {
    println!("Bar!");

    let mut pp = core::point::Point::new();

    pp.set_x( 5.0 );

    pp.print_val();

}
