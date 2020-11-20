use crate::api::{LocationOpenData, OpenDataResponse, StatusOpenData, LOCATION_URL, STATUS_URL};
use hyper::{Body, Method, Request};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_tls::HttpsConnector;
use log::debug;

/// The location response type
pub type LocationResponse = OpenDataResponse<LocationOpenData>;
use bytes::buf::BufExt as _;
const USER_AGENT: &'static str = "strasbourgpark-rs";

use serde::de::DeserializeOwned;
use thiserror::Error;
/// The list of error returned by the client
#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("api error")]
    APIError,
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    HttpError(#[from] hyper::http::Error),
    #[error(transparent)]
    ProxyConfiguration(#[from] hyper::http::uri::InvalidUri),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Tokio(#[from] tokio::task::JoinError),
}
pub(crate) enum HTTPClient {
    Https(hyper::Client<HttpsConnector<hyper::client::HttpConnector>, hyper::Body>),
    Proxy(
        hyper::Client<
            hyper_proxy::ProxyConnector<HttpsConnector<hyper::client::HttpConnector>>,
            hyper::Body,
        >,
    ),
}
/// A `Client` allowing to download the location and status of the parking.
pub struct Client {
    inner: HTTPClient,
    pagination: u32,
}

impl HTTPClient {
    fn request(&self, req: Request<hyper::Body>) -> hyper::client::ResponseFuture {
        match self {
            HTTPClient::Https(client) => client.request(req),
            HTTPClient::Proxy(client) => client.request(req),
        }
    }
}

impl Client {
    /// Create a new Client instance.
    /// It will automatically checks for the presence of
    /// one `https_proxy` variable on the system.
    pub fn new() -> Result<Client, ClientError> {
        // HTTP
        let https = HttpsConnector::new();

        let builder = hyper::Client::builder();

        let inner = match std::env::var_os("https_proxy") {
            Some(proxy_var) => {
                debug!("Proxy detected: {:?}", proxy_var);
                let uri = proxy_var.to_str().unwrap().parse()?;

                let proxy = Proxy::new(Intercept::All, uri);

                let connector = ProxyConnector::from_proxy(https, proxy)?;

                HTTPClient::Proxy(builder.build(connector))
            }
            None => {
                debug!("No proxy detected!");
                HTTPClient::Https(builder.build(https))
            }
        };
        Ok(Client {
            inner,
            pagination: 10,
        })
    }

    /// Retrieve a page of location using the api.
    pub async fn fetch_locations(
        &self,
        offset: u32,
        limit: u32,
    ) -> Result<LocationResponse, ClientError> {
        self.fetch(LOCATION_URL, offset, limit).await
    }

    /// Retrive a page of status from the api.
    pub async fn fetch_status(
        &self,
        offset: u32,
        limit: u32,
    ) -> Result<StatusOpenData, ClientError> {
        self.fetch(STATUS_URL, offset, limit).await
    }

    /// Retrieve all the locations from the API.
    pub async fn fetch_all_locations(&self) -> Result<Vec<LocationOpenData>, ClientError> {
        self.fetch_all_pages(LOCATION_URL).await
    }

    /// Retrieve all the status from the API.
    pub async fn fetch_all_status(&self) -> Result<Vec<StatusOpenData>, ClientError> {
        self.fetch_all_pages(STATUS_URL).await
    }

    async fn fetch_all_pages<T>(&self, uri: &str) -> Result<Vec<T>, ClientError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        // Get the first page
        let first: OpenDataResponse<T> = self.fetch(uri, 0, self.pagination).await?;
        let rest = first.pagination.total - first.records.len() as i32;

        let pagin = self.pagination as i32;
        if rest == 0 {
            Ok(first.records.into_iter().map(|f| f.inner).collect())
        } else {
            let r = rest % pagin as i32;
            let pages = rest / pagin + r;

            let mut values: Vec<_> = (1..pages).map(|w| (pagin * w, pagin)).collect();
            let last_rest = rest - (pages - 1) * pagin;
            values.push((pagin * pages, last_rest));

            let calls: Vec<_> = values
                .iter()
                .map(|f| self.fetch::<OpenDataResponse<T>>(uri, f.0 as u32, f.1 as u32))
                .collect();
            let unpin_futs: Vec<_> = calls.into_iter().map(Box::pin).collect();
            let mut futs = unpin_futs;

            let mut results: Vec<OpenDataResponse<T>> = vec![first];

            while !futs.is_empty() {
                match futures::future::select_all(futs).await {
                    (Ok(val), _index, remaining) => {
                        results.push(val);
                        futs = remaining;
                    }
                    (Err(_e), _index, _remaining) => {
                        // Ignoring all errors
                        return Err(ClientError::APIError);
                    }
                }
            }

            let elements: Vec<T> = results
                .into_iter()
                .flat_map(|f| f.records)
                .map(|f| f.inner)
                .collect();

            Ok(elements)
        }
    }

    async fn fetch<T>(&self, uri: &str, offset: u32, limit: u32) -> Result<T, ClientError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}&start={}&rows={}", uri, offset, limit);
        // Building the request
        let req = Request::builder()
            .method(Method::GET)
            .header("User-Agent", USER_AGENT)
            .uri(url)
            .body(Body::default())?;

        // Read the response
        let resp = self.inner.request(req).await?;
        let (parts, stream) = resp.into_parts();

        if !parts.status.is_success() {
            Err(ClientError::APIError)
        } else {
            // Decoding
            let body = hyper::body::aggregate(stream).await?;
            let t = tokio::task::spawn_blocking(move || serde_json::from_reader(body.reader())).await??;
            Ok(t)
        }
    }
}
