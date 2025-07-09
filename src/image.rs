use crate::rbg::Rgb;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Rgb>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Rgb::BLACK; width * height],
        }
    }

    fn get_index(&self, i: usize, j: usize) -> usize {
        i + (j * self.width)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Rgb> {
        self.data.iter()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Rgb] {
        &mut self.data
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Rgb;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[self.get_index(i, j)]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let idx = self.get_index(i, j);
        &mut self.data[idx]
    }
}
