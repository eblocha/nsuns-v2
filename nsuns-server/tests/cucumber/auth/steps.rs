use cucumber::given;
use http::StatusCode;
use nsuns_server::router::AUTH_PATH;
use tower_cookies::Cookie;

use crate::world::NsunsWorld;

#[given(regex = r#"I am an anonymous user"#)]
async fn anonymous(world: &mut NsunsWorld) {
    let res = world
        .client
        .post(&format!("{AUTH_PATH}/anonymous"))
        .send()
        .await;

    if let Some(header) = res.headers().get("Set-Cookie") {
        let cookie = Cookie::parse(header.to_str().unwrap()).unwrap();
        world.auth_cookie = Some(cookie.value().to_owned())
    }

    assert_eq!(StatusCode::OK, res.status());
}
