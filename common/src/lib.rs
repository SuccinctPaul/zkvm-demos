use dotenv::dotenv;

mod hash;

/// Load fib param from environment variable.
pub fn load_fib_n() -> u32 {
    dotenv().ok();

    let n = std::env::var("FIBONACCI_N").expect("FIBONACCI_N not set");
    n.parse::<u32>()
        .expect("Failed to parse TRICKY_GOOGLE_RECAPTCHA_SITE_KEY: {}")
}
