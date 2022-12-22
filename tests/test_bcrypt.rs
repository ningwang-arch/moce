use pwhash::bcrypt::{self, BcryptSetup, BcryptVariant};

#[test]
fn test() {
    let new_password = bcrypt::hash_with(
        BcryptSetup {
            variant: Some(BcryptVariant::V2a),
            ..Default::default()
        },
        "admin".as_bytes(),
    )
    .unwrap();
    println!("{}", new_password);
}

#[test]
fn verify() {
    let ret = bcrypt::verify(
        "admin",
        "$2a$10$ylVz79ZzpekFgxXZJqSCyOqHQ3gezrYH4SV5TWwtY6S8mNhliryiu",
    );
    println!("{}", ret);
}
