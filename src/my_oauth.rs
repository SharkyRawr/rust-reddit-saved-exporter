use failure;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::url::Url;

pub fn get_reddit_token() -> Option<String>{
    let client =
    BasicClient::new(
        ClientId::new("7KyncUXiH0LlmA".to_string()),
        Some(ClientSecret::new("1S_5y37NvSrt6Yk4GqzldwwCGKQ".to_string())),
        AuthUrl::new("https://www.reddit.com/api/v1/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://www.reddit.com/api/v1/access_token".to_string()).unwrap())
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_url(RedirectUrl::new("http://localhost/".to_string()).unwrap());

    // Generate a PKCE challenge.
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL.
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // Set the desired scopes.
    .add_scope(Scope::new("history".to_string()))
    // Set the PKCE code challenge.
    .set_pkce_challenge(pkce_challenge)
    .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
println!("Browse to: {}", auth_url);

let mut input = String::new();
let string = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");


// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_state`.

// Now you can trade it for an access token.
let token_result =
    client
        .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request(http_client).unwrap();

// Unwrapping token_result will either produce a Token or a RequestTokenError.

    let secret = String::from(token_result.access_token().secret());
    Some(secret)

}