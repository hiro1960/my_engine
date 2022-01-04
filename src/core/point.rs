pub fn point_func() {
    println!("point!");
}

// #[derive(Default)]
pub struct Point {
    pos : [f64; 3]  // 3次元の位置[m]
}


impl Point {
    // Defaultを使わない場合は、new()関数を使う
    pub fn new() -> Point {
        // 初期値の定義
        Point { pos: [0.0; 3] }
    }

    pub fn set_x(&mut self, val: f64) {
        self.pos[0] = val;
    }

    pub fn set_y(&mut self, val: f64) {
        self.pos[1] = val;
    }

    pub fn set_z(&mut self, val: f64) {
        self.pos[2] = val;
    }

    pub fn print_val(&self) {
        println!("( {}, {}, {} )", self.pos[0], self.pos[1], self.pos[2]);
    }

}