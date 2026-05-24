use crate::middleware::jwt_auth::Claims;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};

#[test]
fn test_valid_jwt_decoding() {
    let secret = "super_secret_test_key".as_bytes();
    
    let claims = Claims {
        sub: "usr-123".to_string(),
        username: "tester".to_string(),
        exp: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() + 3600) as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap();

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    );

    assert!(token_data.is_ok());
    let decoded_claims = token_data.unwrap().claims;
    assert_eq!(decoded_claims.sub, "usr-123");
    assert_eq!(decoded_claims.username, "tester");
}

#[test]
fn test_expired_jwt_decoding() {
    let secret = "super_secret_test_key".as_bytes();
    
    let claims = Claims {
        sub: "usr-123".to_string(),
        username: "tester".to_string(),
        exp: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() - 3600) as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap();

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    );

    assert!(token_data.is_err(), "Expired token should fail validation");
}
