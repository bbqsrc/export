#[export::unstable]
/// Have some docs
///
/// Have some anger
fn lolwut() {
    println!("yes");
}

pub fn lolwut2() {
    println!("yes");
}

#[export::unstable]
fn lolwut3() {
    println!("yes");
}

#[export::unstable]
mod lolmod {
    #[export::unstable]
    fn lolwut4() {}
}
