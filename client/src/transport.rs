use async_trait::async_trait;
use jsonrpc_async::Transport;
use url::Url;

pub struct ReqwestTransport {
    client: reqwest::Client,
    url: Url,
}

impl ReqwestTransport {
    pub fn new(url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
        }
    }

    async fn request<R>(&self, req: impl serde::Serialize) -> Result<R, reqwest::Error>
    where
        R: for<'a> serde::de::Deserialize<'a>,
    {
        let response = self.client.post(self.url.clone()).json(&req).send().await?;
        response.json().await
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
