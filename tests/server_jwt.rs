use fittrack::server::auth::JwtService;

#[test]
fn jwt_round_trip_preserves_user_id() {
    let jwt = JwtService::new("local-test-secret", 60);
    let token = jwt.issue(42).expect("token");
    let claims = jwt.verify(&token).expect("claims");

    assert_eq!(claims.sub, "42");
    assert_eq!(jwt.user_id(&token).expect("user id"), 42);
}
