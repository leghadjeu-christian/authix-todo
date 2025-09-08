# Web Application with Rust, Diesel, Keycloak, and PostgreSQL

To clone this repository, use the following command:
```bash
git clone https://github.com/leghadjeu-christian/web_application.git
cd web_application
```


This project is a web application built with Rust, utilizing the Diesel ORM for database interactions, Keycloak for authentication, and PostgreSQL as the database.

## Prerequisites

Before you begin, ensure you have the following installed:

*   **Rust and Cargo**: If you don't have Rust and Cargo installed, you can get them using `rustup`.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    After installation, make sure Cargo's bin directory is in your PATH. You might need to restart your terminal or run:
    ```bash
    source $HOME/.cargo/env
    ```

*   **Diesel CLI**: Install the Diesel CLI for database migrations and schema management.
    ```bash
    cargo install diesel_cli --no-default-features --features "postgres"
    ```

*   **Docker and Docker Compose**: These are required to run Keycloak and PostgreSQL services.
    *   [Install Docker](https://docs.docker.com/get-docker/)
    *   [Install Docker Compose](https://docs.docker.com/compose/install/)

## Getting Started

### 1. Database Setup (PostgreSQL)

The project uses a PostgreSQL database. You can run it using Docker Compose.

1.  **Environment Variables**: Create a `.env` file in the project root with your database credentials. An example `.env` is provided in the project.

    ```
    DATABASE_URL=postgres://user:password@localhost:5432/mydatabase
    # Add other environment variables as needed
    ```

2.  **Start PostgreSQL**: Use Docker Compose to start the PostgreSQL container.
    ```bash
    docker-compose up -d postgres
    ```

3.  **Run Migrations**: Once the database is running, apply the Diesel migrations to set up your schema.
    ```bash
    diesel migration run
    ```

### 2. Authentication Setup (Keycloak)

Keycloak is used for user authentication and authorization.

1.  **Start Keycloak**: Use Docker Compose to start the Keycloak container.
    ```bash
    docker-compose up -d keycloak
    ```

2.  **Access Keycloak Admin Console**: After Keycloak starts (it might take a few moments), access the admin console at `http://localhost:8080/admin`.
    *   Default Admin Username: `admin`
    *   Default Admin Password: `admin` (or as configured in `docker-compose.yaml`)

3.  **Create a Realm**:
    *   In the Keycloak admin console, hover over "Master" in the top left and click "Add Realm".
    *   Give your realm a name (e.g., `myrelm`).

4.  **Create a Client**:
    *   Go to "Clients" in your new realm.
    *   Click "Create client".
    *   Client ID (e.g., `myclient`).
    *   Set "Client authentication" to ON.
    *   Set "Standard flow enabled" to ON.
    *   Set "Valid redirect URIs" to `http://localhost:8000/*` (or your application's redirect URI).
    *   Set "Web origins" to `http://localhost:8000`.

5.  **Create a User**:
    *   Go to "Users" -> "Add User".
    *   Fill in a username and email.
    *   Go to the "Credentials" tab for the new user, set a password, and toggle "Temporary" to OFF.

6.  **Configure Application**: Update your application's Keycloak configuration (e.g., in `.env` or a configuration file) with the realm and client details.

    ```
    KEYCLOAK_REALM=myrelm
    KEYCLOAK_CLIENT_ID=myclient
    KEYCLOAK_AUTH_SERVER_URL=http://localhost:8080/
    ```

### 3. Running the Application

1.  **Build the application**:
    ```bash
    cargo build
    ```

2.  **Run the application**:
    ```bash
    cargo run
    ```
    The application should now be running, typically accessible at `http://localhost:8000`.
