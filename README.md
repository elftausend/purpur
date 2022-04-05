# purpur

Library for loading pictures and csv files.
It supports some sort of "Transforms" (torchvision)

## Example

```rust
use purpur::{Transforms, Apply, ImageReturn};

let mut ir = ImageReturn::default();
let mut transforms = Transforms::new(vec![
    Apply::CenterCrop(120, 120),
    Apply::GetImgRet(&mut ir),
]);

//For instance: A dataset directory contains 3 directories with pictures of dogs, cats and birds.
transforms.apply("../dataset").unwrap();
```