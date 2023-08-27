extern crate neru;

use neru::cpu::Cpu;

fn main() {
    println!("neru v{}", env!("CARGO_PKG_VERSION"));
    let cpu = Cpu::new();
    println!("exit");
}