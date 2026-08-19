use argon2::Argon2;
use argon2::password_hash::{
    Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
};

#[allow(unused)]
pub fn generate_hashed_password(user_password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(user_password.as_bytes(), &salt)?
        .to_string())
}

#[allow(unused)]
pub fn check_password_hash(user_password: &str, password_hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default()
        .verify_password(user_password.as_bytes(), &parsed_hash)
        .is_ok())
}
