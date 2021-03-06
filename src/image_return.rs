use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ImageReturn {
    pub found_in: Vec<PathBuf>,
    pub exact_paths: Vec<PathBuf>,
    pub data: Vec<u8>,
    pub channels: Vec<u8>,
    pub dims: Vec<(usize, usize)>,
}

impl ImageReturn {
    pub fn found_in(&self) -> &Vec<PathBuf> {
        &self.found_in
    }

    pub fn get_classes(&self) -> Vec<(&str, usize)> {
        let mut classes = vec![(Default::default(), 0); self.found_in().len()];
        
        for (idx, path) in self.found_in().iter().enumerate() {
            let last_folder = path.components().last().unwrap().as_os_str().to_str().unwrap();
            classes[idx] = (last_folder, idx);
        }
        classes
    }

    pub fn get_classes_for_imgs(&self) -> Vec<usize> {
        let classes = self.get_classes();
        
        let mut classes_for_imgs = Vec::new();
        
        for entry in self.exact_paths.iter() {
            let mut comps = entry.components();
            
            let class_img = comps.nth(comps.clone().count()-2).unwrap().as_os_str().to_str().unwrap();
            let mut index = 0;
            for class in &classes {
                if class.0 == class_img {
                    break;
                }
                index += 1;
            }
            classes_for_imgs.push(index);
        }
        classes_for_imgs
    }

    pub fn sample_count(&self) -> usize {
        self.exact_paths.len()
    }

    pub fn features(&self) -> usize {
        self.data.len() / self.sample_count()
    }
}