use purpur::{Transforms, Apply, ImageReturn};



#[test]
fn test_transforms() {
    let mut ir = ImageReturn::default();
    let mut transforms = Transforms::new(vec![
        Apply::CenterCrop(60, 60),
        Apply::GetImgRet(&mut ir),
        Apply::SaveTo("./cropped1")
    ]);
    transforms.apply("../gradients-fallback/datasets/berries_aug_6xx/train/").unwrap();    
    //println!("{:?}", ir.get_classes());

}