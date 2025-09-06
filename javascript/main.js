window.keycloak = new Keycloak({
    url: 'http://localhost:8080/',
    realm: 'myrealm',
    clientId: 'myclient'
});

window.addEventListener("DOMContentLoaded", () => {
    window.keycloak.init({ onLoad: 'check-sso', checkLoginIframe: false }).then(authenticated => {
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

            loadHeader(); // Load the header content
            getItems();
            initializeAppFeatures();

        } else if (window.location.pathname !== '/login/') {
            console.log("⛔ Not authenticated with Keycloak, redirecting to login.");
            doLogout(); // Ensures the user is redirected if not authenticated
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

function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.addEventListener('readystatechange', function () {
        if (this.readyState === this.DONE) {
            if (this.status === 401) {
                console.log("API call returned 401. Re-authenticating or logging out.");
                doLogout();
            } else if (this.status >= 200 && this.status < 300) {
                try {
                    const response = JSON.parse(this.responseText);
                    renderItems(response["pending_items"], "edit", "pendingItems", editItem);
                    renderItems(response["done_items"], "delete", "doneItems", deleteItem);
                    document.getElementById("completeNum").innerHTML = response["done_item_count"];
                    document.getElementById("pendingNum").innerHTML = response["pending_item_count"];
                } catch (e) {
                    console.error("Failed to parse API response:", e);
                    // Optionally, provide user feedback about the error
                }
            } else {
                console.error(`API call failed with status ${this.status}: ${this.responseText}`);
                // Optionally, provide user feedback about the error
            }
        }
    });

    console.log(`➡️ Making API call: ${method} /api/v1${url}`);
    xhr.open(method, "/api/v1" + url);
    xhr.setRequestHeader("Content-Type", "application/json");
    if (keycloak.token) {
        console.log(`🔑 Keycloak token present. Length: ${keycloak.token.length}. Sending in Authorization header.`);
        xhr.setRequestHeader("Authorization", "Bearer " + keycloak.token); // Use Keycloak's token
    } else {
        console.warn("Keycloak token not available for API call. Redirecting to login.");
        doLogout();
    }
    return xhr;
}

function editItem() {
    let title = this.id.replaceAll("-", " ").replace("edit ", "");
    let call = apiCall("/item/edit", "PUT");
    let json = { title: title, status: "done" };
    call.send(JSON.stringify(json));
}

function deleteItem() {
    let title = this.id.replaceAll("-", " ").replace("delete ", "");
    let call = apiCall("/item/delete", "POST");
    let json = { title: title, status: "done" };
    call.send(JSON.stringify(json));
}

function loadHeader() {
    fetch('/templates/components/header.html')
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
            }
        })
        .catch(error => {
            console.error("Failed to load header:", error);
        });
}

function getItems() {
    let call = apiCall("/item/get", "GET");
    call.send();
}

function createItem() {
    console.log("🔵 createItem function called.");
    let titleInput = document.getElementById("name");
    if (titleInput) {
        console.log(`🔍 Input element found. Current value: "${titleInput.value}"`);
        if (titleInput.value.trim() !== "") {
            console.log(`🚀 Sending API call to create item: "${titleInput.value}"`);
            let call = apiCall("/item/create/" + encodeURIComponent(titleInput.value), "POST");
            call.send();
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
    // Attach event listener for the Create button
    const createButton = document.getElementById("create-button");
    if (createButton) {
        console.log("✅ 'create-button' element found. Attaching event listener.");
        createButton.addEventListener("click", createItem);
    } else {
        console.error("❌ 'create-button' element NOT found.");
    }

    // Attach event listener for the Logout button
    const logoutButton = document.getElementById("logout-button");
    if (logoutButton) {
        console.log("✅ 'logout-button' element found. Attaching event listener.");
        logoutButton.addEventListener("click", doLogout);
    } else {
        console.warn("Logout button not found.");
    }
}
