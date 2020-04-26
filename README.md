# rust-oauth-rocket

Example of [OAuth2 Authentication](https://github.com/ramosbugs/oauth2-rs) with the [Rocket Web Framework](https://rocket.rs/). This project was presented at the [Desert Rust](https://rust.azdevs.org/) meetup.

## Usage

### Create Google API Credentials

1. Go to [Google API Credentials](https://console.developers.google.com/apis/credentials)

2. Click **Create Credentials** and choose **OAuth Client ID**

3. Select **Web application** as the Application type, name the credentials something meaningful, and add an Authorized redirect URI. If you are following along with this example, the Authorized redirect URI should be `http://localhost:8080/`. The ending slash in the redirect URI is important.

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
cargo run
```
