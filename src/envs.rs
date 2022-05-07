pub fn get_env(env_name: &str) -> String {
    std::env::var(env_name).unwrap_or_else(|_| panic!("{} must be set", env_name))
}
