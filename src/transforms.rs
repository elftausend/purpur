use std::{path::Path, fs::File, io::Read, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use image::{ColorType, GenericImageView, imageops::FilterType};
use rand::prelude::SliceRandom;

use crate::{ImageReturn, utils, create_path_with_hash};

#[derive(Debug)]
pub enum Apply<'a> {
    ///Flip vertically
    FlipV,
    FlipH,
    Grayscale,
    Contrast(f32),
    Resize(u32, u32),
    CenterCrop(u32, u32),
    Encode,
    GetVec(&'a mut Vec<u8>),
    GetImgRet(&'a mut ImageReturn),
    SaveTo(&'a str),
}

pub struct Transforms<'a> {
    applies: Vec<Apply<'a>>,
    shuffle: bool,
}

impl <'a>Transforms<'a> {
    pub fn new(applies: Vec<Apply<'a>>) -> Transforms<'a> {
        Transforms {
            applies,
            shuffle: false,
        }
    }
    pub fn shuffle(mut self) -> Self {
        self.shuffle = !self.shuffle;
        self
    }

    pub fn apply<P: AsRef<Path>>(&mut self, path: P) -> Result<(), std::io::Error> {
        let (found_in, mut exact_paths) = utils::get_paths(path)?;

        let mut rng = rand::thread_rng();
        
        if self.shuffle {
            exact_paths.shuffle(&mut rng);
        }
        
        let mut channels = Vec::new();
        let mut dims = Vec::new();

        let image_count = exact_paths.len();

        let mut img_encode = false;

        for (idx, image_path) in exact_paths.iter().enumerate() {
            match image::open(image_path) {
                Ok(mut img) => {
                    match img.color() {
                        ColorType::L8 => channels.push(1),
                        ColorType::La8 => channels.push(1),
                        ColorType::Rgb8 => channels.push(3),
                        ColorType::Rgba8 => channels.push(3),
                        _ => continue,
                    }
                    for apply in self.applies.iter_mut() {
                        match apply {
                            Apply::FlipV => img = img.flipv(),
                            Apply::FlipH => img = img.flipv(),
                            Apply::Contrast(c) => img = img.adjust_contrast(*c),
                            Apply::Resize(w, h) => img = img.resize_exact(*w, *h, FilterType::CatmullRom),
                            Apply::GetVec(vec) => {      
                                vec.extend_from_slice(img.as_bytes());
                            },
                            Apply::SaveTo(p) => {
                                let mut hasher = DefaultHasher::default();                                
                                img.as_bytes().hash(&mut hasher);
                                let hash = hasher.finish();
                                
                                let path = create_path_with_hash(image_path, p, hash)?;
                                //let path = create_new_path_from_old(image_path.path(), p, &mut rng)?;
                                std::fs::create_dir_all(path.parent().unwrap())?;
                                img.save(&path).unwrap();
                            },
                            Apply::GetImgRet(img_ret) => {
                                
                                if img_encode {
                                    let mut from_encoder = Vec::new();
                                    let mut file = File::open(image_path).unwrap();
                                    file.read_to_end(&mut from_encoder).unwrap();
                                    img_ret.data.append(&mut from_encoder);
                                } else {
                                    img_ret.data.extend_from_slice(img.as_bytes());
                                };
                                
                                if idx == image_count-1 {
                                    **img_ret = ImageReturn {
                                        found_in: found_in.clone(),
                                        exact_paths: exact_paths.clone(),
                                        data: img_ret.data.clone(),
                                        channels: channels.clone(),
                                        dims: dims.clone(),
                                    }    
                                }
                            },
                            Apply::Encode => img_encode = true,
                            Apply::CenterCrop(nw, nh) => {
                                let (width, height) = img.dimensions();
                                
                                let subtract_w = width - *nw;
                                let subtract_h = height - *nh;
                                
                                img = img.crop_imm(subtract_w/2, subtract_h/2, width-subtract_w, height-subtract_h);
                            },
                            Apply::Grayscale => img = img.grayscale(),
                        }
                    }
                    let img_dims = img.dimensions();
                    dims.push((img_dims.0 as usize, img_dims.1 as usize));
                },
                Err(_) => continue,
            }
        }
        
        Ok(())
    }
}
