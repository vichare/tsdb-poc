use chrono::NaiveDateTime;
use tsdb_poc::data::types::data::Data;

fn main() {
    // println!("Hello, world!");
    let d = Data {
        value: 2.0f64,
        timestamp: NaiveDateTime::from_timestamp(1000, 0),
    };
    println!("{:?}", d);
}
