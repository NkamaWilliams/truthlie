use backend::{
    state::AppState,
    handlers::player,
};
use actix_web::{App, web};
use serde_json::json;

#[test]
fn test_is_working(){
    assert!(true, "Test is working!");
}

#[actix_web::test]
async fn test_create_player() {
    let app_state = AppState::new().data();

    let app = actix_web::test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(web::scope("/players")
                .service(player::create_player)
            )
    ).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/players/")
        .set_json(json!({
            "name": "Test Player"
        }))
        .to_request();

    let resp = actix_web::test::call_service(&app, req).await;
    let is_success = resp.status().is_success();
    println!("Response: {:?}", resp.into_body());
    assert!(is_success, "Expected success response");
}