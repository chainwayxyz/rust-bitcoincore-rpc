use async_trait::async_trait;
use jsonrpc_async::Transport;
use std::time::Duration;
use url::Url;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

pub struct ReqwestTransport {
    client: reqwest::Client,
    url: Url,
}

impl ReqwestTransport {
    pub fn new(url: Url) -> Self {
        let client = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            url,
        }
    }

    pub fn with_timeouts(
        url: Url,
        timeout: Option<Duration>,
        connect_timeout: Option<Duration>,
    ) -> Self {
        let builder = reqwest::Client::builder()
            .timeout(timeout.unwrap_or(DEFAULT_TIMEOUT))
            .connect_timeout(connect_timeout.unwrap_or(DEFAULT_TIMEOUT));

        let client = builder.build().expect("Failed to build reqwest client");

        Self {
            client,
            url,
        }
    }

    async fn request<R>(&self, req: impl serde::Serialize) -> Result<R, reqwest::Error>
    where
        R: for<'a> serde::de::Deserialize<'a>,
    {
        match self.client.post(self.url.clone()).json(&req).send().await {
            Ok(res) => res.json().await,
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl Transport for ReqwestTransport {
    async fn send_request(
        &self,
        r: jsonrpc_async::Request<'_>,
    ) -> Result<jsonrpc_async::Response, jsonrpc_async::Error> {
        Ok(self.request(r).await.map_err(|e| jsonrpc_async::Error::Transport(e.into()))?)
    }

    async fn send_batch(
        &self,
        _rs: &[jsonrpc_async::Request<'_>],
    ) -> Result<Vec<jsonrpc_async::Response>, jsonrpc_async::Error> {
        unimplemented!()
    }

    fn fmt_target(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let (Some(host), Some(port)) = (self.url.host(), self.url.port()) {
            write!(f, "http://{}:{}{}", host, port, self.url.path())
        } else {
            write!(f, "http://{:?}", self.url)
        }
    }
}
