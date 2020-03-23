#[export::unstable]
/// Have some docs
///
/// Have some anger
fn lolwut() {
    println!("yes");
}

#[must_use]
pub fn lolwut2() {
    println!("yes");
}

#[export::unstable]
fn lolwut3() {
    println!("yes");
}

#[export::unstable]
/// If I document this
/// 
/// Is it happy?
mod lolmod {
    #[export::unstable]
    fn lolwut4() {}
}

#[export::unstable]
/// They see my struct, they hatin'
pub(crate) struct Foo {

}

impl Foo {
    #[export::unstable]
    /// A dangerous function with no plan.
    fn bar() {

    }
}