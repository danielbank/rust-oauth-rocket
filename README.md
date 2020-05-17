# rust-oauth-rocket

Example of [OAuth2 Authentication](https://github.com/ramosbugs/oauth2-rs) with the [Rocket Web Framework](https://rocket.rs/). This project was presented at the [Desert Rust](https://rust.azdevs.org/) meetup.

## ⚠️ Experimental Code ⚠️

This is demo code and not meant for production use. The cookie used has `secure` set to `false` and `SameSite::Lax` so that it can be used for testing in localhost. I am also not sure about the use of a cookie for sharing the `csrf_state` between endpoints.

## Usage

### Create Google API Credentials

1. Go to [Google API Credentials](https://console.developers.google.com/apis/credentials)

2. Click **Create Credentials** and choose **OAuth Client ID**

3. Select **Web application** as the Application type, name the credentials something meaningful, and add two Authorized redirect URIs. If you are following along with this example, the Authorized redirect URIs should be `http://localhost:8080/` and `http://localhost:8080/oauth/callback`. The ending slash in the first redirect URI is important.

### Run the Example OAuth Code

The OAuth example code comes directly from the [oauth2-rs](https://github.com/ramosbugs/oauth2-rs) repo, it is just provided here for convenience. Run it using the following command:

```
GOOGLE_CLIENT_ID=xxx GOOGLE_CLIENT_SECRET=yyy cargo run --example oauth
```

### Run the Rocket Server

1. Rocket depends on the latest nightly build of Rust. You can make it the default toolchain by running:

```
rustup default nightly
```

You can also use a per-directory override to use the nightly version only for this project:

```
cd path/to/rust-oauth-rocket
rustup override set nightly
```

2. Run the Rocket server using the following command:

```
GOOGLE_CLIENT_ID=xxx GOOGLE_CLIENT_SECRET=yyy cargo run
```
