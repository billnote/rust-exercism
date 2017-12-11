extern crate num;

use num::Num;

pub struct Triangle<T: Num> {
    sides: (T, T, T),
}

impl<T: Num + PartialEq + PartialOrd + Default + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Result<Triangle<T>, String> {
        let mut sides_vec = vec![sides[0], sides[1], sides[2]];
        sides_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut iter = sides_vec.iter();
        let sides_pair = (
            *iter.next().unwrap_or(&Default::default()),
            *iter.next().unwrap_or(&Default::default()),
            *iter.next().unwrap_or(&Default::default()),
        );

        if sides_pair.0 + sides_pair.1 > sides_pair.2 {
            Ok(Triangle { sides: sides_pair })
        } else {
            Err(String::from("error sides"))
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.0 == self.sides.1 && self.sides.1 == self.sides.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.0 == self.sides.1 || self.sides.1 == self.sides.2
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.0 != self.sides.1 && self.sides.1 != self.sides.2
    }
}
