// 様々な汎用関数をここに定義する

// 補間関数
pub fn hokan(x:f64, x1:f64, x2:f64, v1:f64, v2:f64) -> f64 {
	//                     (v2-v1)
	//  returnValue = v1 + ------- * (x-x1)
	//                     (x2-x1)
    let r:f64;  // rは、１回しかセットしていないのでmutは不要

    if (x2 - x1).abs() > 0.0 {
		r = v1 + ((v2 - v1) * (x - x1) / (x2 - x1));
    }
    else
    {
        r = v1;
    }

    return r;
}

// 積分関数
// 注：積分をするときは、途中で刻みは変えないのでメンバーにしたほうがいい
// 　　しかし既存が引数に刻みを入れているので、合わせておく
pub struct Integrator1 {
  out: f64,
  mem: f64,
}

impl Integrator1 {
  pub fn new() -> Integrator1 {
    Integrator1{
      out:0.0,
      mem:0.0,
    }
  }

  // 積分
  // inpd: 入力値
  // dt: 刻み
  // -> 積分結果
  pub fn get(&mut self, inpd:f64, dt:f64 ) -> f64 {
    //                                                                                                         
    // improved trapezoidal integral                                                                           
    //                                                                                                         
    // outd(t+1) = out(t) + (3*inpd(t) - inpd(t-1))/2*dt                                                       
    //                                                                                                         
    // mem: inpd(t-1)                                                                                          
    //    
    let outd:f64; // 積分結果

    outd = self.out + ( 3.0 * inpd - self.mem ) * dt / 2.0;
    self.mem = inpd;
    self.out = outd;

    outd
  }
}