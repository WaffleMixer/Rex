use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Version {
    name: String,
}

/// Uses Github API to get the latest release version number to check if the current version matches with it.
/// If not, we will start the new version pop up
pub fn check_version() -> Result<bool, reqwest::Error> {
    let cu_version = "v0.0".to_string();
    static APP_USER_AGENT: &str = "Rex";

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .connect_timeout(std::time::Duration::new(2, 0))
        .build()?;

    let caller: Version = client
        .get("https://api.github.com/repos/WaffleMixer/Rex/releases/latest")
        .send()?
        .json()?;
    if cu_version != caller.name {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

#[cfg(test)]
mod tests {
    use super::check_version;

    #[test]
    // fails as there is no release posted
    fn version_checker_fails() {
        let result = check_version().unwrap_err();
        assert!(result.is_decode());
    }
}
