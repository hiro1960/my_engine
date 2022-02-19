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
pub struct Integrator1 {
  val: f64,
  mem: f64,
  dt : f64,
}

impl Integrator1 {
  /**
    新規生成
    @param[in] av 値
    @param[in] dt 刻み値[s]
    @return 生成したIntegrator1
   */
  pub fn new(av:f64, t:f64) -> Integrator1 {
    Integrator1{
      val: av,
      mem: 0.0,
      dt: t,
    }
  }

  /**
   値のセット
   @param[in] av 新しい値
   */
  pub fn set_val(&mut self, av:f64) {
    self.val = av;
  }

  /**
   刻み値のセット
   @param[in] dtime [s]
   */
  pub fn set_dtime(&mut self, dtime: f64) {
    self.dt = dtime;
  }

  // 現在値
  pub fn val(&self) -> f64 {
    self.val
  }

  // 積分
  // inpd: 入力値
  // -> 積分結果
  pub fn integral(&mut self, inpd:f64 ) -> f64 {
    //                                                                                                         
    // improved trapezoidal integral                                                                           
    //                                                                                                         
    // outd(t+1) = out(t) + (3*inpd(t) - inpd(t-1))/2*dt                                                       
    //                                                                                                         
    // mem: inpd(t-1)                                                                                          
    //    
    let outd:f64; // 積分結果

    outd = self.val + ( 3.0 * inpd - self.mem ) * self.dt / 2.0;
    self.mem = inpd;
    self.val = outd;

    // returnと書かなくてもOK
    outd
  }
}