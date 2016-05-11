extern crate openvr;
extern crate nalgebra;

pub fn main () {
    let system = openvr::init().unwrap();
    let render_model = openvr::render_models().unwrap();
    let _ = openvr::compositor().unwrap();

    loop {
        let _ = openvr::compositor().unwrap().wait_get_poses();
        let raw = system.projection_matrix(openvr::Eye::Left, 0.1, 1000.0);

        let mat = nalgebra::Matrix4::new(
            raw[0][0], raw[0][1], raw[0][2], raw[0][3],
            raw[1][0], raw[1][1], raw[1][2], raw[1][3],
            raw[2][0], raw[2][1], raw[2][2], raw[2][3],
            raw[3][0], raw[3][1], raw[3][2], raw[3][3]);

        println!("{:?}", mat);
    }
}
