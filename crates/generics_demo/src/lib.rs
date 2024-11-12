fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    println!("largest item is:{}", largest);
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    println!("largest item is:{}", largest);
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_i32() {
        let list = [1,2,3];
        assert_eq!(*largest_i32(&list), 3);
    }

    #[test]
    fn test_largest_char() {
        let list = ['a', 'b', 'c', 'd', 'e'];
        assert_eq!(*largest_char(&list), 'e');
    }

    #[test]
    fn test_largest() {
        let list = ['a', 'b', 'c', 'd', 'e'];
        assert_eq!(*largest(&list), 'e');
    }

}