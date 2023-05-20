use crate::error::ErrorType;

#[derive(Debug, Clone)]
pub struct ImageGray {
    pub data: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl ImageGray {
    pub fn new(data: &Vec<Vec<u8>>, height: usize, width: usize) -> Self {
        ImageGray { data: data.to_vec(), 
                    height: height, 
                    width: width, }
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[i][j] = alpha;
            }
        }
    }

    pub fn copy_from(&mut self, other: &ImageGray) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[i][j] = other.data[i][j];
            }
        }
    }

    pub fn invert(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[i][j] = 255 - self.data[i][j];
            }
        }
    }

    pub fn add_intensity(&self, beta: u8) -> Result<ImageGray, ErrorType> {
        let mut current_max: u8 = 0;
        let mut current_min: u8 = 255;
        let mut result = self.clone();

        for inner_vec in &result.data {
            let inner_max = inner_vec.iter().max().unwrap();
            let inner_min = inner_vec.iter().min().unwrap();
            if current_max < *inner_max {
                current_max = *inner_max;
            }
            if current_min > *inner_min {
                current_min = *inner_min;
            }
        }
        
        if (current_max + beta > 255) && (current_min + beta < 0 ) {
            return Err(ErrorType::PixelInvalidValue);
        } else {
            for i in 0..result.height {
                for j in 0..result.width {
                    result.data[i][j] += beta;
                }
            }
            return Ok(result);
        }
    }
}