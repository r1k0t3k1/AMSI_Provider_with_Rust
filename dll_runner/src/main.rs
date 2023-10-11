
#[link(name = "AMSI_Provider_with_Rust.dll", kind="dylib")]
extern {
    fn add(left: usize, right: usize) -> usize;
}

fn main() {
    println!("2+2={}", unsafe{ add(2,2) });
}
