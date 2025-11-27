use keyring::Entry;
use std::error::Error;

use crate::config;

pub struct CredentialStore;

impl CredentialStore {
    pub fn save(user: &str, token: &str) -> Result<(), Box<dyn Error>> {
        let app_name = &config::get().app_name;

        let entry = Entry::new(app_name, user)?;

        match entry.set_password(token) {
            Ok(_) => println!("✅ Auth set"),
            Err(e) => {
                println!("❌ Failed to set Auth");
                return Err(e.into());
            }
        }

        // Verify it was saved
        match entry.get_password() {
            Ok(saved) => println!("✅ Auth verified"),
            Err(e) => println!("❌ Failed to verify: {}", e),
        }

        Ok(())
    }
    pub fn get(user: &str) -> Result<String, Box<dyn Error>> {
        let app_name = &config::get().app_name;
        let entry = Entry::new(app_name, user)?;
        let token = entry.get_password()?;
        Ok(token)
    }
}
