use std::io::Read;


#[test]
fn test_picture_bytes() {
    let mut buf = Vec::new();
    let mut file = std::fs::File::open("../../datasets/picture/file.png").unwrap();
    file.read_to_end(&mut buf).unwrap();

//    let data = image::open("../../datasets/picture/file.png").unwrap().into_bytes();

}