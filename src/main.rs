#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use std::env;

#[get("/authorize")]
pub fn authorize(client: State<BasicClient>) -> Redirect {
    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .url();
    Redirect::to(authorize_url.to_string())
}

#[get("/callback?<code>&<state>")]
pub fn callback(client: State<BasicClient>, code: &RawStr, state: &RawStr) -> String {
    let decoded = code.url_decode();
    match decoded {
        Ok(auth_code) => {
            let code = AuthorizationCode::new(auth_code);
            println!("Google returned the following code:\n{}\n", code.secret());
            let state = CsrfToken::new(state.to_string());
            // TODO: NEED TO GET THE CSRF_STATE HERE FROM AUTHORIZE()
            println!(
                "Google returned the following state:\n{:?} (expected csrf_state.secret() :P)\n",
                state.secret(),
                // csrf_state.secret()
            );

            // TODO: SEND THE TOKEN REQUEST USING ROCKET OR AT LEAST HYPER
            let token = client.exchange_code(code).request(http_client);
            println!("Google returned the following token:\n{:?}\n", token);
            format!("Go back to your terminal :)")
        }
        Err(err) => format!("Failed to parse AuthCode: {}", err),
    }
}

fn main() {
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string());
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string());
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url.unwrap(),
        Some(token_url.unwrap()),
    )
    .set_redirect_url(
        RedirectUrl::new("http://localhost:8080/oauth/callback".to_string()).unwrap(),
    );

    rocket::ignite()
        .manage(client)
        .mount("/oauth", routes![authorize, callback])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
