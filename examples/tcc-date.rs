extern crate tcc;
extern crate time;

fn main() {
    let ts = time::get_time();
    let t = ts.sec as f64 + 1.0e-9*(ts.nsec as f64);
    println!("{}",t);
}
