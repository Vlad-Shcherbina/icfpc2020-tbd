use std::ops::{ Index, IndexMut };

#[derive(Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct ImgMatrix {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<u8>>,
}

impl Index<Coord> for ImgMatrix {
    type Output = u8;

    fn index(&self, coord: Coord) -> &u8 {
        assert!(coord.x < self.width && coord.y < self.height);
        &self.data[coord.y][coord.x]
    }
}

impl IndexMut<Coord> for ImgMatrix {
    fn index_mut(&mut self, coord: Coord) -> &mut u8 {
        assert!(coord.x < self.width && coord.y < self.height);
        &mut self.data[coord.y][coord.x]
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for ImgMatrix {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] != other.data[i][j] { 
                    return false;
                }
            }
        }
        true
    }
}

impl ImgMatrix {
    pub fn new(width: usize, height: usize) -> ImgMatrix {
        assert_ne!(height, 0);
        assert_ne!(width, 0);
        let data: Vec<Vec<u8>> = vec![vec![0; width]; height];
        ImgMatrix { width, height, data }
    }

    pub fn from_vec(data: &Vec<Vec<u8>>) -> ImgMatrix {
        let height = data.len();
        assert_ne!(height, 0);
        let width = data[0].len();
        assert_ne!(width, 0);
        for v in data.iter() {
            assert_eq!(width, v.len());
        }
        ImgMatrix { width, height, data: data.clone() }
    }

    pub fn grow(&mut self, new_width: usize, new_height: usize) {
        assert!(new_width > self.width);
        assert!(new_height > self.height);
        let mut new_data = vec![vec![0; new_width]; new_height];
        for y in 0..self.height {
            for x in 0..self.width {
                new_data[y][x] = self.data[y][x];
            }
        }
        self.data = new_data;
        self.width = new_width;
        self.height = new_height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_implements() {
        let mut m1 = ImgMatrix::new(5, 6);
        let m2 = ImgMatrix::new(3, 4);
        let mut m3 = ImgMatrix::new(5, 6);

        assert_ne!(m1, m2);
        assert_eq!(m1, m3);
        
        let c1 = Coord {x : 1, y : 2};
        let c2 = Coord {x : 2, y : 3};
        assert_ne!(c1, c2);

        m1[c1] = 1;
        m3[c2] = 1;
        assert_ne!(m1, m3);

        m1.grow(10, 10);
    }
}
