use keyring::Entry;
use std::error::Error;

const SERVICE_NAME: &str = "ticktick-cli";

pub struct CredentialStore;

impl CredentialStore {
    pub fn save(token: &str) -> Result<(), Box<dyn Error>> {
        let entry = Entry::new(SERVICE_NAME, "user-test")?;
        entry.set_password(token)?;
        Ok(())
    }
    pub fn get() -> Result<String, Box<dyn Error>> {
        let entry = Entry::new(SERVICE_NAME, "user-test")?;
        let token = entry.get_password()?;
        Ok(token)
    }
}
