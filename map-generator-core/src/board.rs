use serde;
use crate::land::Land;
use std::cmp::{max, min};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    #[inline]
    pub fn move_cost(&self, other: &Pos) -> usize {
        max(self.x, other.x) - min(self.x, other.x) + max(self.y, other.y) - min(self.y, other.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Board<'a> {
    width: usize,
    height: usize,
    tiles: Vec<Land<'a>>,
}

impl<'a> Board<'a> {
    pub fn build<F>(width: usize, height: usize) -> Board<'a> {
        let mut tiles = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                tiles.push(builder(x, y));
            }
        }

        Board {
            width,
            height,
            tiles,
        }
    }


    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn index_at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn at(&self, x: usize, y: usize) -> &Land<'_> {
        &self.tiles[self.index_at(x, y)]
    }
}

impl<'a> Index<Pos> for Board<'a> {
    type Output = Land<'a>;

    #[inline]
    fn index(&self, p: Pos) -> &Land<'a> {
        &self.tiles[self.index_at(p.x, p.y)]
    }
}

impl<'a> IndexMut<Pos> for Board<'a> {
    #[inline]
    fn index_mut(&mut self, p: Pos) -> &mut Land<'a> {
        let idx = self.index_at(p.x, p.y);
        &mut self.tiles[idx]
    }
}

impl<'a> serde::Serialize for Board<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        struct Tiles<'a> {
            w: usize,
            h: usize,
            vec: &'a Vec<Land>,
        }
        impl<'a> serde::Serialize for Tiles<'a> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(self.h))?;
                for row in self.vec.chunks(self.w) {
                    seq.serialize_element(row)?;
                }
                seq.end()
            }
        }

        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("width", &self.width)?;
        map.serialize_entry("height", &self.height)?;
        map.serialize_entry(
            "tiles",
            &Tiles {
                w: self.width,
                h: self.height,
                vec: &self.tiles,
            },
        )?;


        map.end()
    }
}