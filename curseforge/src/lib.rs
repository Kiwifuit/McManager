#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env::var;

    #[test]
    fn setup_env() {
        dotenv().expect("expected .env to exist");
        let key = var("CURSEFORGE_API_KEY").expect("expected CURSEFORGE_API_KEY var to be set");

        dbg!(key);
    }
}
