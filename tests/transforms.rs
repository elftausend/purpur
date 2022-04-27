use purpur::{Transforms, Apply, ImageReturn};

#[test]
fn test_transforms() {
    let mut vec = Vec::new();
    let mut ir = ImageReturn::default();
    let mut transforms = Transforms::new(vec![
        Apply::CenterCrop(60, 60),
        Apply::GetVec(&mut vec),
        Apply::GetImgRet(&mut ir),
        //Apply::SaveTo("./cropped")
    ]);
    transforms.apply("../gradients-fallback/datasets/berries_aug_6xx/train/").unwrap();
    let features = ir.data.len() / ir.exact_paths.len();
    println!("data len: {}", ir.data.len());
    println!("features: {features}");

    println!("vec len: {}", vec.len());
    //println!("{:?}", ir.get_classes());

}