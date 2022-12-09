pub mod coordinates {

    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct Coord2D {
        pub x: i32,
        pub y: i32,
    }

    impl Coord2D {
        pub fn new(x: i32, y: i32) -> Coord2D {
            Coord2D { x, y }
        }

        pub fn get_distance(self, other: Self) -> i32 {
            let a = self.x - other.x;
            let b = self.y - other.y;
            a * a + b * b
        }

        #[allow(dead_code)]
        pub fn get_difference(self, other: Self) -> Self {
            self - other
        }

        /**
        Normalises any coordinate to a -1 to 1 scale
        */
        pub fn normalise(self) -> Self {
            Coord2D {
                x: match self.x {
                    _ if self.x >= 1 => 1,
                    _ if self.x == 0 => 0,
                    _ if self.x <= -1 => -1,
                    _ => 0,
                },
                y: match self.y {
                    _ if self.y >= 1 => 1,
                    _ if self.y == 0 => 0,
                    _ if self.y <= -1 => -1,
                    _ => 0,
                },
            }
        }
    }

    impl std::ops::Add for Coord2D {
        fn add(self, rhs: Self) -> Self::Output {
            Coord2D {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }

        type Output = Self;
    }

    impl std::ops::AddAssign for Coord2D {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl std::ops::Sub for Coord2D {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Coord2D {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl std::ops::SubAssign for Coord2D {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl std::ops::Neg for Coord2D {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Coord2D {
                x: self.x * -1,
                y: self.y * -1,
            }
        }
    }
}
