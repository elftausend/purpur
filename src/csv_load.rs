use std::path::Path;

use csv::Reader;

use crate::shuffle;

pub struct CSVReturn<T> {
    pub x: Vec<T>,
    pub y: Vec<T>,
    pub max: T,
    pub sample_count: usize,
    pub features: usize,

}

impl <T>CSVReturn<T> {
    pub fn sample_count(&self) -> usize {
        self.sample_count
    }

    pub fn features(&self) -> usize {
        self.x.len() / self.sample_count
    }
}

pub struct CSVLoader {
    shuffle: bool,
}

impl CSVLoader {
    pub fn new(shuffle: bool) -> Self {
        CSVLoader {
            shuffle
        }
    }

    pub fn load<T, P: AsRef<Path> >(&self, path: P) -> Result<CSVReturn<T>, std::io::Error> where T: Default + Copy + std::cmp::PartialOrd + std::str::FromStr {
        let mut rdr = Reader::from_path(path).unwrap();
        
        let mut x: Vec<T> = Vec::new();
        let mut y: Vec<T> = Vec::new();
        let mut max = T::default();
        let mut m: usize = 0;

        
        
        for result in rdr.records() {
            let mut first = true;
            let record = result.unwrap();
            
            for r in record.iter() {
                let mut r = r.to_string();
                r.retain(|c| c != '"');
                
                let value = match r.parse::<T>() {
                    Ok(value) => value,
                    Err(_) => panic!("Read value was not an integer."),
                };

                if first {
                    y.push(value);
                    first = false;
                } else { 
                    if value > max {
                        max = value;
                    }
                    x.push(value)
                }
            }
            m+=1;
        }
 
        if self.shuffle {
            let shuffeled = shuffle(m, &x, &y);
            x = shuffeled.0;
            y = shuffeled.1;
        }
        let csv = CSVReturn {
            features: x.len() / m,
            y,
            x,
            max,
            sample_count: m,
            
        };
        Ok(csv)
        
    }
}
