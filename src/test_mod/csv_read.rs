// csvファイルを読み込む
// extern crate csv;  // これはもういらない。tomlへ記述してuseすれば直接インポートされる

use std::fs::File;
use std::error::Error;

pub fn csv_read(filename :&str) -> Result<(), Box<dyn Error>>  {
    println!("in csv_read, {}", filename);

    // csvを読み込み、vetorに格納する
    let mut idx: Vec<f64> = Vec::new();
    let mut val: Vec<f64> = Vec::new();
    println!("idx.length() = {}", idx.len());

    let file = File::open(filename).expect("file not found");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        
        idx.push( record[0].parse().unwrap() );
        val.push( record[1].parse().unwrap() );
    }

    println!("idx.length() = {}", idx.len());
    for v in &idx { // idxは参照(&)を渡しておかないと、for-loop中で所有権がmoveされてしまい、以後使えなくなる
        println!(" idx = {}", v);
    }
    // 通常の、indexで参照するやり方
    for i in 0..idx.len() {
        println!(" idx = {}, val = {}", idx[i], val[i]);
    }

    Ok(())

}