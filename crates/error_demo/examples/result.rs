

/// 获取用户
fn get_user() -> Option<String> {
    None
}

fn get_user_name11() -> Option<String> {
    let user = get_user()?;
    // Do something with `user`
    // ...
    Some(user)
}

fn get_user_name() -> Result<String, String> {
    let user = get_user().ok_or("User not found".to_string())?;
    // Do something with `user`
    // ...
    Ok(user)
}

fn get_default_username() -> Result<String, String> {
    let user = get_user().ok_or("User not found".to_string())?;
    // Do something with `user`
    // ...
    Ok(user)
}

fn get_user_name1() -> Result<String, String> {
    let user = get_user().unwrap_or_else(|| get_default_username().unwrap());
    // Do something with `user`
    // ...
    Ok(user)
}

fn get_user_name2() -> Result<String, String> {
    let user = match get_user() {
        Some(user) => user,
        None => return Err("No User".into()),
    };
    Ok(user)
}

fn get_user_name3() -> Result<String, String> {
    let Some(user) = get_user() else {
        return Err("No User".into());
    };
    // do something with `user`
    Ok(user)
}


fn main() {

    let user = get_user_name3().unwrap();
    println!("user: {}", user);

}