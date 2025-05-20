use anyhow::Result;


fn get_user() -> Result<String> {
    None
}

fn get_user_name() -> Result<String> {
    let user = get_user()?;
    // Do something with `user`
    // ...
    Ok(user)
}

fn main() {

}