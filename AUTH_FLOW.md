# Authentication and Authorization Flow in the Web Application

This document outlines the authentication and authorization process within the web application, detailing how JSON Web Tokens (JWTs) are handled from the frontend to the backend using Keycloak and Actix-web.

## 1. Frontend Authentication (Keycloak JavaScript Adapter)

1.  **Initialization**: When the application loads, the Keycloak JavaScript adapter is initialized using `keycloak.init({ onLoad: 'check-sso' })` in [`javascript/main.js`](javascript/main.js:8).
    *   `onLoad: 'check-sso'` attempts to silently check if the user is already authenticated with Keycloak. If a valid session exists, Keycloak retrieves the user's token.
2.  **Authentication Success**:
    *   If `keycloak.init` returns `authenticated: true`, the application proceeds to load features.
    *   The `keycloak.token` (the JWT) is stored in `localStorage` as "user-token" for initial backend compatibility, although the primary method for sending the token to the backend is via the `Authorization` header.
    *   A `setInterval` is set up to periodically refresh the Keycloak token using `keycloak.updateToken(70)` ([`javascript/main.js`](javascript/main.js:22)). This ensures the user's session remains active without requiring re-authentication as long as the refresh token is valid.
    *   `loadHeader()` is called, and upon its completion, `getItems()` and `initializeAppFeatures()` are executed to populate the UI and attach event listeners ([`javascript/main.js`](javascript/main.js:35)).
3.  **Authentication Failure/Logout**:
    *   If Keycloak initialization fails or a token refresh fails, `doLogout()` is called ([`javascript/main.js`](javascript/main.js:31)).
    *   `doLogout()` redirects the user to the Keycloak login page and clears the "user-token" from `localStorage` ([`javascript/main.js`](javascript/main.js:54)).

## 2. Frontend API Calls (javascript/main.js)

1.  **`apiCall` Function**: All API interactions from the frontend use the `apiCall(url, method, body)` function ([`javascript/main.js`](javascript/main.js:88)).
2.  **Authorization Header**: Before sending a request, `apiCall` checks for `keycloak.token`. If present, it adds an `Authorization` header to the request in the format `Bearer <JWT>` ([`javascript/main.js`](javascript/main.js:119)).
3.  **Request Body**: For `POST` and `PUT` requests, the `body` parameter is `JSON.stringify`'d and sent as the request body ([`javascript/main.js`](javascript/main.js:134)).
4.  **Error Handling**: If an API call returns a `401 Unauthorized` status, `doLogout()` is automatically invoked to redirect the user to the login page ([`javascript/main.js`](javascript/main.js:92)).

## 3. Backend Authorization Middleware (src/main.rs)

1.  **Actix-web Middleware**: The Actix-web application in [`src/main.rs`](src/main.rs:113) uses a custom `wrap_fn` middleware to intercept and process all incoming requests.
2.  **Request Logging**: Upon receiving a request, the middleware logs comprehensive details including the request method, URI, and all headers ([`src/main.rs`](src/main.rs:123)). This is crucial for debugging unauthorized access issues.
3.  **API Path Check**: The middleware checks if the request URI contains `/api/v1/item/` ([`src/main.rs`](src/main.rs:124)).
    *   If it's an API item path, the `auth::process_token()` function is called to handle token validation.
    *   For other paths (e.g., static files, login/logout pages), `passed` is set to `true`, allowing the request to proceed without token validation.
4.  **Authorization Decision**:
    *   If `auth::process_token()` returns `Ok(_)`, `passed` is set to `true`, and the request continues to the service.
    *   If `auth::process_token()` returns `Err(_)`, `passed` is set to `false`, and an `HttpResponse::Unauthorized()` (401 status) is returned to the client ([`src/main.rs`](src/main.rs:146)).

## 4. Backend Token Processing (src/auth/mod.rs & src/auth/processes.rs)

1.  **Dynamic Keycloak Configuration Fetching**: At application startup, `src/main.rs` now dynamically fetches the Keycloak OpenID Connect configuration from `http://localhost:8080/realms/myrealm/.well-known/openid-configuration` using the `fetch_keycloak_openid_config` function from [`src/auth/keycloak_config.rs`](src/auth/keycloak_config.rs).
    *   The `jwks_uri` (JSON Web Key Set URI) extracted from this configuration is then stored in the Actix-web application state using `web::Data<String>`. This replaces the previous static `keycloak_pub.pem` file for public key retrieval.

2.  **`auth::process_token(request: &ServiceRequest, jwks_uri: web::Data<String>)`**: This asynchronous function in [`src/auth/mod.rs`](src/auth/mod.rs) orchestrates the token extraction and dynamic validation.
    *   It first calls `auth::processes::extract_header_token(request)` to retrieve the `Authorization` header.
    *   If a token is successfully extracted, it then calls the asynchronous `auth::processes::check_password(token_string, jwks_uri)` to validate the token against the dynamically fetched JWKS.
    *   Detailed `info!` and `warn!` logs are used throughout this process to track token extraction and validation status, including errors related to `jwks_uri` retrieval or token validation failures.

3.  **`auth::processes::extract_header_token(request: &ServiceRequest)`**: This function in [`src/auth/processes.rs`](src/auth/processes.rs) is responsible for:
    *   Extracting the `Authorization` header from the `ServiceRequest`.
    *   Verifying that the header starts with "Bearer ".
    *   Returning the JWT string if successfully extracted, otherwise returning an `Err` with a descriptive message.

4.  **`auth::processes::check_password(jwt_token: String, jwks_uri: &str)`**: This asynchronous function in [`src/auth/processes.rs`](src/auth/processes.rs) performs the actual dynamic JWT validation:
    *   It first decodes the JWT header to extract the Key ID (`kid`).
    *   It then uses `reqwest` to fetch the JSON Web Key Set (JWKS) from the provided `jwks_uri`.
    *   The fetched JWKS is parsed using the `jsonwebkey` crate.
    *   It finds the correct JSON Web Key (JWK) within the JWKS that matches the `kid` from the JWT header.
    *   A `jsonwebtoken::DecodingKey` is created from the matched JWK.
    *   Finally, it validates the JWT's signature and claims (e.g., expiration `exp`, issuer `iss`, audience `aud`) using `jsonwebtoken` against the dynamically obtained `DecodingKey`. Keycloak typically uses the `RS256` algorithm.
    *   Extensive `info!` and `warn!` logs are generated for each step of the validation process, including fetching JWKS, finding the correct key, creating the decoding key, and final token verification. If any step fails, an `Err` is returned with a specific reason.

## Summary

The authentication flow relies on Keycloak for frontend user authentication and JWT generation. These JWTs are then sent with API requests via the `Authorization` header. The backend, powered by Actix-web, uses custom middleware to intercept these requests, extract the JWT, and validate it against the Keycloak public key. Detailed logging has been integrated into both frontend and backend components to provide deep visibility into each stage of this process, aiding in the diagnosis of unauthorized access issues.