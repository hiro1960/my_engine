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
