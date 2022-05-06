// PBKDF2 < bcrypt < scrypt
fn passhash(name: &str, pass: &str) -> String {
    let namedpass = format!("{}{}", name, pass);
    let hash = bcrypt::hash(namedpass.as_bytes(), bcrypt::DEFAULT_COST).unwrap();
    // info!("{}{}: {}", name, pass, hash);
    hash
}
fn passhash_verify(name: &str, pass: &str, hash: &str) -> bool {
    let namedpass = format!("{}{}", name, pass);
    bcrypt::verify(namedpass.as_bytes(), hash).unwrap()
}

#[cfg(any(feature = "mysql"))]
type SqlID = u64;
#[cfg(any(feature = "postgres", feature = "sqlite"))]
type SqlID = i64;

// https://docs.rs/sqlx/0.5.7/sqlx/trait.FromRow.html
// Extend derive(FromRow): https://github.com/launchbadge/sqlx/issues/156
type SqlDateTime = chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: SqlID,
    pub name: String,
    // pub phone: String,
    pub email: String,
    // not return password
    #[serde(skip_serializing)]
    pub pass: String,
    pub status: String,
    pub create_dt: SqlDateTime,
    pub update_dt: SqlDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub name: String,
    pub password: String,
    #[serde(default)]
    pub rememberme: bool,
}

impl Login {
    pub fn verify(&self, hash: &str) -> bool {
        passhash_verify(&self.name, &self.password, hash)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    // username
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Register {
    #[validate(length(min = 3, max = 33), custom = "validate_username")]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

use validator::ValidationError;
fn validate_username(username: &str) -> Result<(), ValidationError> {
    // todo: use regex for robust
    if first_char_is_number(username) {
        return Err(ValidationError::new(
            "terrible_username: first char is number",
        ));
    }

    if username.contains("@") {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username: contains @"));
    }

    Ok(())
}

impl Register {
    pub fn passhash(&self) -> String {
        passhash(&self.name, &self.password)
    }
}

pub fn first_char_is_number(s: &str) -> bool {
    s.get(0..1).and_then(|c| c.parse::<u8>().ok()).is_some()
}
