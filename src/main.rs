struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // check if a rectangle can fit in another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {}

fn give_seven() -> i32 {
    7
}

#[cfg(test)]
mod tests {
    use super::*;

    // simple assert
    #[test]
    fn assert_test() {
        assert!(4 == 4);
    }

    // assertion equals
    #[test]
    #[ignore] // ignore this test
    fn assert_eq_test() {
        assert_eq!(2 + 2, 4); // equal - should pass
        assert_ne!(2 + 2, 5); // not equal - should pass
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn test_fn() {
        assert_eq!(super::give_seven(), 7);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        assert!(rect1.can_hold(&rect2));
        assert!(!rect1.can_hold(&rect3));
    }
}
