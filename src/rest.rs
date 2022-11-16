use sysinfo::{System, SystemExt};

use crate::utils::{process_info::*, request::build_reqwest_client};

pub struct RESTClient {
    port: u32,
    reqwest_client: reqwest::Client,
}

type Error = Box<dyn std::error::Error>;

impl RESTClient {
    /// Create a new instance of the LCU REST wrapper
    pub fn new() -> Result<Self, Error> {
        let mut sys = System::new_all();
        sys.refresh_all();
        let process = find_process(&sys)?;
        let args = extract_info(process)?;
        let auth_token = encode_token(&args.0);
        let client = build_reqwest_client(Some(auth_token))?;
        Ok(Self {
            port: args.1,
            reqwest_client: client,
        })
    }

    /// Make a get request to the specified endpoint
    pub async fn get(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .get(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a post request to the specified endpoint
    pub async fn post(
        &self,
        endpoint: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let mut req_build = self
            .reqwest_client
            .post(format!("https://127.0.0.1:{}{}", self.port, endpoint));

        req_build = match body {
            Some(b) => req_build.json(&b),
            None => req_build
        };

        let req = req_build.send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a put request to the specified endpoint
    pub async fn put(
        &self,
        endpoint: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let mut req_build = self
            .reqwest_client
            .put(format!("https://127.0.0.1:{}{}", self.port, endpoint));

        req_build = match body {
            Some(b) => req_build.json(&b),
            None => req_build
        };

        let req = req_build.send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a patch request to the specified endpoint
    pub async fn patch(
        &self,
        endpoint: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let mut req_build = self
            .reqwest_client
            .patch(format!("https://127.0.0.1:{}{}", self.port, endpoint));

        req_build = match body {
            Some(b) => req_build.json(&b),
            None => req_build
        };

        let req = req_build.send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a delete request to the specified endpoint
    pub async fn delete(
        &self,
        endpoint: String,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .delete(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a head request to the specified endpoint
    pub async fn head(
        &self,
        endpoint: String,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .head(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }
}
