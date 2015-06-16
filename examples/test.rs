extern crate vr;

fn main() {
    let vr = vr::IVRSystem::init().unwrap();
    print!("{:?}", vr.bounds());
}