use purpur::{Transforms, Apply};



#[test]
fn test_transforms() {
    let mut data = Vec::new();
    let transforms = Transforms::new(&mut [Apply::GetVec(&mut data)]);
}