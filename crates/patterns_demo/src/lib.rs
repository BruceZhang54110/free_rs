fn favorite_color() {
    let favorite_color: Option<&str> = None;
    let is_trusday = false;
    let age: Result<u8, _> = "35".parse();

    if let Some(color) = favorite_color {
        println!("Using your color, {}, as the background", color);
    } else if is_trusday {
        println!("Tuseday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_favorite_color() {
        favorite_color();
    }

}