#[derive(Debug)]
pub struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 2,
            width: 6,
        };

        assert!(larger.can_hold(&smaller));
    }
}
