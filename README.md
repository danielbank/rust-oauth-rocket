# rust-oauth-rocket

Example of [OAuth2 Authentication](https://github.com/ramosbugs/oauth2-rs) with the [Rocket Web Framework](https://rocket.rs/). This project was presented at the [Desert Rust](https://rust.azdevs.org/) meetup.

## Usage

### Create Google API Credentials

1. Go to [Google API Credentials](https://console.developers.google.com/apis/credentials)

2. Click **Create Credentials** and choose **OAuth Client ID**

3. Select **Web application** as the Application type, name the credentials something meaningful, and add an Authorized redirect URI. If you are following along with this example, the Authorized redirect URI should be `http://localhost:8080/`. The ending slash in the redirect URI is important.

### Run the Example OAuth Code

This code comes directly from the [oauth2-rs](https://github.com/ramosbugs/oauth2-rs) repo, it is just provided here for convenience.

```
GOOGLE_CLIENT_ID=xxx GOOGLE_CLIENT_SECRET=yyy  cargo run --example oauth
```
