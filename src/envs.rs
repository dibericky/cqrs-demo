pub fn get_env(env_name: &str) -> String {
    std::env::var(env_name)
        .ok()
        .expect(&format!("{} must be set", env_name))
}
