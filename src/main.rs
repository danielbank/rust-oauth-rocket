#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use rocket::http::{Cookie, Cookies, RawStr, SameSite};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use std::env;

#[derive(Debug)]
struct CsrfState(String);

impl Into<String> for CsrfState {
    fn into(self) -> String {
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for CsrfState {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<CsrfState, Self::Error> {
        request
            .cookies()
            .get_private("csrf_state")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|csrf_state| CsrfState(csrf_state))
            .or_forward(())
    }
}

#[get("/authorize")]
fn authorize(mut cookies: Cookies<'_>, client: State<BasicClient>) -> Redirect {
    // Generate the authorization URL to which we'll redirect the user
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .url();
    // Save the CSRF State Secret in session so we can check it later when callback is called
    let mut cookie = Cookie::build("csrf_state", csrf_state.secret().clone())
        .path("/oauth")
        .secure(false)
        .http_only(true)
        .finish();
    cookie.set_same_site(SameSite::Lax);
    cookies.add_private(cookie);
    Redirect::to(authorize_url.to_string())
}

// Endpoint for receiving the callback from the Google OAuth server
#[get("/callback?<code>&<state>")]
fn callback(
    csrf_state: CsrfState,
    mut cookies: Cookies<'_>,
    client: State<BasicClient>,
    code: &RawStr,
    state: &RawStr,
) -> String {
    // Attempt to decode the code received from Google
    let decoded = code.url_decode();
    match decoded {
        Ok(auth_code) => {
            // Check that the code from Google matches the code we stored in our session
            let code = AuthorizationCode::new(auth_code);
            let returned_state = CsrfToken::new(state.to_string());
            let session_state: String = csrf_state.into();
            assert_eq!(
                returned_state.secret(),
                &session_state,
                "Google returned the following state:\n{:?} (expected {:?})\n",
                returned_state.secret(),
                &session_state
            );
            // Remove the code from session, we don't need it anymore
            cookies.remove_private(Cookie::named("csrf_state"));

            // Exchange the code for a token
            let token = client.exchange_code(code).request(http_client);
            println!("Google returned the following token:\n{:?}\n", token);
            format!("Go back to your terminal :)")
        }
        Err(err) => format!("Failed to parse AuthCode: {}", err),
    }
}

fn main() {
    // Get ClientId and ClientSecret from Environment Variables and define Google endpoints
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string());
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string());
    let redirect_url = RedirectUrl::new("http://localhost:8080/oauth/callback".to_string());

    // Set up the config for the Google OAuth2 process
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url.unwrap(),
        Some(token_url.unwrap()),
    )
    .set_redirect_url(redirect_url.unwrap());

    // Launch the Rocket web server
    rocket::ignite()
        .manage(client)
        .mount("/oauth", routes![authorize, callback])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
