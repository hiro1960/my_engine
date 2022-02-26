// 様々な汎用関数をここに定義する

/**
 * 補間関数
 * @param[in] x   補間したい値
 * @param[in] x1
 * @param[in] x2
 * @param[in] v1  x1の時の値
 * @param[in] v2　x2の時の値
 * @return 補間結果
 */
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

/**
 * 積分型
 */
pub struct Integrator1 {
  val: f64, // 出力値
  mem: f64, // 積分補助変数（1周期前の入力値）
  dt : f64, // 積分刻み[s]
}

impl Integrator1 {
  /**
   * 新規生成
   * @param[in] av 値
   * @param[in] dt 刻み値[s]
   * @return 生成したIntegrator1
   */
  pub fn new(av:f64, t:f64) -> Integrator1 {
    Integrator1{
      val: av,
      mem: 0.0,
      dt: t,
    }
  }

  /**
   * 値のセット
   * @param[in] av 新しい値
   */
  pub fn set_val(&mut self, av:f64) {
    self.val = av;
  }

  /**
   * 刻み値のセット
   * @param[in] dtime [s]
   */
  pub fn set_dtime(&mut self, dtime: f64) {
    self.dt = dtime;
  }

  /**
   * 現在値
   * @return 出力値
   */
  pub fn val(&self) -> f64 {
    self.val
  }

  /**
   * 積分
   * @param[in] inpd 積分入力値
   * @return 積分結果（出力値）
   */
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

/**
 * "data"を返す（#define代わり）
 * 　global変数を定義できない（不可能ではないが面倒くさい）ので、その代わりの関数を定義する
 * 
 *   project毎に変化する内容なので、本来別モジュールに定義した方がいい
 *   i32等、basicな型を返すものは、const宣言するべき（String変換して返しているので、毎回メモリを確保するためconstできない）
 * 
 * @return "data"
 */
pub fn data_dir() -> String {
  // ディレクトリの定義
  "data".to_string()
}
