#[derive(PartialEq, Debug)]
struct Shoe {
    size:i32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


fn shoes_in_size1(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    // shoes.iter().filter(|s| s.size == shoe_size).collect()
    todo!()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_sum() {
        let v1 = vec![1,2,3];
        let total: i32 = v1.iter().sum();
        assert_eq!(total, 6);

        let v1 = vec![1,2,3];
        let v3: Vec<_> = v1.iter().map(|i| i + 1).collect();
        assert_eq!(v3, vec![2, 3, 4]);
    }

    #[test]
    fn test_map() {
        let v1 = vec![1,2,3];
        let v3: Vec<_> = v1.iter().map(|i| i + 1).collect();
        assert_eq!(v3, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

}