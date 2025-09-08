use diesel::prelude::*;
use crate::database::establish_connection;
use crate::models::user::user::User;
use crate::models::user::new_user::NewUser;
use crate::schema::users;
use log::{info, error};

pub fn find_or_create_user(user_id: &str, email: &str, username: &str) -> Result<User, String> {
    let mut connection = establish_connection();

    // Try to find the user by unique_id (which is claims.sub)
    let user_result = users::table
        .filter(users::columns::id.eq(user_id))
        .first::<User>(&mut connection);

    match user_result {
        Ok(user) => {
            info!("Found existing user with unique_id: {}", user_id);
            Ok(user)
        },
        Err(diesel::NotFound) => {
            info!("User with unique_id '{}' not found. Creating new user.", user_id);
            // Create a new user if not found
            // For password, we'll use a dummy as Keycloak handles authentication
            let new_user = NewUser::new(
                username.to_string(),
                email.to_string(),
                "DUMMY_PASSWORD".to_string() // Dummy password
            );

            // Override the generated UUID with the Keycloak user_id (claims.sub)
            let new_user_with_keycloak_id = NewUser {
                id: user_id.to_string(),
                ..new_user
            };

            diesel::insert_into(users::table)
                .values(&new_user_with_keycloak_id)
                .get_result::<User>(&mut connection)
                .map_err(|e| {
                    error!("Error creating new user: {}", e);
                    format!("Failed to create user: {}", e)
                })
        },
        Err(e) => {
            error!("Error querying for user with unique_id {}: {}", user_id, e);
            Err(format!("Database error: {}", e))
        }
    }
}