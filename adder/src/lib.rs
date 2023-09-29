pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn fails() {
        panic!("Make this test fail");
    }

    #[test]
    fn holds_a_smaller_rectangle() {
        // Arrange
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };
        let larger: Rectangle = Rectangle {
            width: 10,
            height: 5,
        };

        // Act
        let can_hold = larger.can_hold(&smaller);

        // Assert
        assert!(can_hold);
    }

    #[test]
    fn smaller_rectangle_does_not_hold_larger_one() {
        // Arrange
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };
        let larger: Rectangle = Rectangle {
            width: 10,
            height: 5,
        };

        // Act
        let can_hold = smaller.can_hold(&larger);

        // Assert
        assert!(!can_hold);
    }
}
