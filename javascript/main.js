window.keycloak = new Keycloak({
    url: 'http://localhost:8080/',
    realm: 'myrealm',
    clientId: 'myclient'
});

window.addEventListener("DOMContentLoaded", () => {
    window.keycloak.init({ onLoad: 'check-sso' }).then(authenticated => {
        if (authenticated) {
            console.log("🔐 Keycloak authenticated.");
            // If already on the login page and authenticated, redirect to the main app
            if (window.location.pathname === '/login/') {
                console.log("Redirecting authenticated user from /login/ to /");
                window.location.href = '/';
                return; // Stop further execution in this block
            }
            // Keep for existing backend compatibility for now, though backend should ideally validate the header token
            localStorage.setItem("user-token", keycloak.token);

            // Refresh token regularly
            setInterval(() => {
                keycloak.updateToken(70).then(refreshed => {
                    if (refreshed) {
                        console.log('Token refreshed.');
                        localStorage.setItem("user-token", keycloak.token);
                    } else {
                        console.log('Token not refreshed, valid for ' + Math.round(keycloak.tokenParsed.exp + keycloak.timeSkew - new Date().getTime() / 1000) + ' seconds');
                    }
                }).catch(() => {
                    console.error('Failed to refresh token');
                    doLogout(); // Redirect to login on refresh failure
                });
            }, 60000); // Check every minute

            loadHeader().then(() => { // Call initializeAppFeatures AFTER loadHeader completes
                getItems();
                initializeAppFeatures();
            }).catch(error => {
                console.error("Failed to initialize app features due to header loading error:", error);
                // Optionally, display a user-friendly message or redirect to an error page
            });
            }

    }).catch((error) => {
        console.error("Failed to initialize Keycloak:", error);
        if (window.location.pathname !== '/login/') {
            doLogout(); // Fallback or error page, only if not on login page
        }
    });
});

function doLogout() {
    console.log("Logging out...");
    keycloak.logout({ redirectUri: document.location.origin + "/login/" });
    localStorage.removeItem("user-token"); // Clear local storage token as well
}

function renderItems(items, processType, elementId, processFunction) {
    let placeholder = "<div>";
    let itemsMeta = [];

    for (let i = 0; i < items.length; i++) {
        let title = items[i]["title"];
        let placeholderId = processType + "-" + title.replaceAll(" ", "-");

        placeholder += `
            <div class="itemContainer">
                <p>${title}</p>
                <div class="actionButton" id="${placeholderId}">${processType}</div>
            </div>
        `;
        itemsMeta.push({ id: placeholderId, title: title });
    }

    placeholder += "</div>";
    document.getElementById(elementId).innerHTML = placeholder;

    for (let item of itemsMeta) {
        const itemElement = document.getElementById(item.id);
        if (itemElement) {
            itemElement.addEventListener("click", processFunction);
        } else {
            console.warn(`Element with ID ${item.id} not found for event listener.`);
        }
    }
}

function apiCall(url, method, body = null) { // Added body parameter for logging
    let xhr = new XMLHttpRequest();
    xhr.addEventListener('readystatechange', function () {
        if (this.readyState === this.DONE) {
            if (this.status === 401) {
                console.log("API call returned 401. Re-authenticating or logging out.");
                doLogout();
            } else if (this.status >= 200 && this.status < 300) {
                try {
                    const response = JSON.parse(this.responseText);
                    console.log(`✅ API call successful to ${url}. Response:`, response); // Log successful response
                    renderItems(response["pending_items"], "edit", "pendingItems", editItem);
                    renderItems(response["done_items"], "delete", "doneItems", deleteItem);
                    document.getElementById("completeNum").innerHTML = response["done_item_count"];
                    document.getElementById("pendingNum").innerHTML = response["pending_item_count"];
                } catch (e) {
                    console.error(`❌ Failed to parse API response from ${url}:`, e, "Response Text:", this.responseText); // Log parsing errors
                    // Optionally, provide user feedback about the error
                }
            } else {
                console.error(`❌ API call to ${url} failed with status ${this.status}: ${this.responseText}`); // Log API errors
                // Optionally, provide user feedback about the error
            }
        }
    });

    const fullUrl = "/api/v1" + url;
    console.log(`⬆️ Sending API call: ${method} ${fullUrl}`);
    let headers = {
        "Content-Type": "application/json"
    };
    if (keycloak.token) {
        const fullAuthHeader = "Bearer " + keycloak.token;
        headers["Authorization"] = fullAuthHeader;
        console.log(`   Authorization header: ${fullAuthHeader}`); // Log full token
    } else {
        console.warn("⚠️ Keycloak token not available for API call. Logging out.");
        doLogout();
        return; // Stop execution if no token
    }
    if (body) {
        console.log("   Request body:", JSON.stringify(body)); // Log request body as string
    } else {
        console.log("   No request body.");
    }

    xhr.open(method, "/api/v1" + url);
    for (let headerName in headers) {
        xhr.setRequestHeader(headerName, headers[headerName]);
    }
    xhr.send(body ? JSON.stringify(body) : null);
    return xhr;
}

function editItem() {
    let title = this.id.replaceAll("-", " ").replace("edit ", "");
    let json = { title: title, status: "done" };
    let call = apiCall("/item/edit", "PUT", json);
}

function deleteItem() {
    let title = this.id.replaceAll("-", " ").replace("delete ", "");
    let json = { title: title, status: "done" };
    let call = apiCall("/item/delete", "POST", json);
}

function loadHeader() {
    return fetch('/templates/components/header.html') // Return the promise
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            return response.text();
        })
        .then(html => {
            const headerPlaceholder = document.getElementById('header-placeholder');
            if (headerPlaceholder) {
                headerPlaceholder.innerHTML = html;
                console.log("✅ Header loaded successfully.");
            } else {
                console.error("❌ Header placeholder not found.");
                return Promise.reject(new Error("❌ Header placeholder not found.")); // Reject if placeholder is missing
            }
        })
        .catch(error => {
            console.error("Failed to load header:", error);
            return Promise.reject(error); // Re-throw or reject the promise on error
        });
}

function getItems() {
    let call = apiCall("/item/get", "GET");
}

function createItem() {
    console.log("🔵 createItem function called.");
    let titleInput = document.getElementById("name");
    if (titleInput) {
        console.log(`🔍 Input element found. Current value: "${titleInput.value}"`);
        if (titleInput.value.trim() !== "") {
            console.log(`🚀 Sending API call to create item: "${titleInput.value}"`);
            let json = { title: titleInput.value.trim() }; // Create JSON body for the new item
            let call = apiCall("/item/create/" + encodeURIComponent(titleInput.value), "POST", json);
            titleInput.value = ""; // Clear the input after sending
        } else {
            console.warn("⚠️ Item title input is empty. Please enter a title.");
            // Optionally, provide user feedback that title is required
        }
    } else {
        console.error("❌ Item title input with ID 'name' not found.");
    }
}

function initializeAppFeatures() {
    // Ensure the create-button exists before adding an event listener
    const createButton = document.getElementById("create-button");
    if (createButton) {
        console.log("✅ 'create-button' element found. Attaching event listener.");
        createButton.addEventListener("click", createItem);
    } else {
        console.error("❌ 'create-button' element NOT found.");
    }

    const logoutButton = document.getElementById("logout-button");
    if (logoutButton) {
        logoutButton.addEventListener("click", doLogout);
    } else {
        console.warn("Logout button not found.");
    }
}
