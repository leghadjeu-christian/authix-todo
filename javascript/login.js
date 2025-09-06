// Ensure Keycloak is loaded and accessible via window.keycloak
document.getElementById("loginButton")?.addEventListener("click", () => {
    if (window.keycloak) {
        window.keycloak.login();
    } else {
        console.error("Keycloak instance not found. Make sure keycloak.js is loaded and initialized.");
    }
});

document.getElementById("registerButton")?.addEventListener("click", () => {
    if (window.keycloak) {
        window.keycloak.register();
    } else {
        console.error("Keycloak instance not found. Make sure keycloak.js is loaded and initialized.");
    }
});

// Logout is now handled by the doLogout function in main.js,
// which uses window.keycloak.logout().
// The logout button in login.js should ideally call that function if on login page,
// or be handled by the main app if on another page.
// For simplicity, if this logout button is on the login page and main.js is not loaded,
// it will perform a direct Keycloak logout.
document.getElementById("logoutButton")?.addEventListener("click", () => {
    if (window.keycloak) {
        // Assuming doLogout is in global scope or main.js is loaded
        // If main.js is not loaded on login page, this needs to be self-contained
        if (typeof doLogout === 'function') {
            doLogout();
        } else {
            // Fallback for when main.js (and thus doLogout) is not loaded
            window.keycloak.logout({ redirectUri: window.location.origin + "/login/" });
            localStorage.removeItem("user-token");
        }
    } else {
        console.error("Keycloak instance not found for logout.");
        localStorage.removeItem("user-token"); // Clear local token even if Keycloak isn't initialized
        window.location.href = window.location.origin + "/login/"; // Redirect to login
    }
});

// The token logic and redirection fix from the original login.js
// is largely superseded by the Keycloak adapter in main.js
// However, if login.js might be loaded independently or before main.js,
// we might need some initial checks, but ideally main.js should be the primary entry.
// For now, we will remove the redundant token logic as main.js handles it.
window.addEventListener("DOMContentLoaded", () => {
    // If main.js loads and initializes Keycloak first, this block might not be strictly necessary
    // for token handling, but can be kept for initial state checks if needed.
    // However, given the `onLoad: 'login-required'` in main.js,
    // Keycloak will handle the initial redirection.
    // The previous manual token extraction and redirection logic is redundant here.
    console.log("login.js DOMContentLoaded: Keycloak will handle authentication via main.js.");
});
