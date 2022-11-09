//! Provides most simple ways to make http request, simple as what JavaScript dose!
use reqwest::blocking::{Body, Client, RequestBuilder, Response};
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

pub use reqwest::Method;

#[allow(rustdoc::bare_urls)]

/// Most simple way to make http request, using keep-alive connection pooling.
///```rust
/// use wsd::http::*;
//
/// fn test() {
///    let mut c = Request::new(Method::POST, "https://docs.rs");
///    c.gzip(true);
///    c.timeout(5);
///    c.header("TOKEN", "1234567890");
///    c.send("{id: 100}", |data| {
///        println!("Data: {}", data.text());
///        println!("Headers: {:#?}", data.headers());
///     });
///}
///```
pub struct Request {
    url: String,
    method: Method,
    inner: Option<Client>,
    headers: HashMap<String, String>,
    error: String,
    timeout: i32,
    gzip: bool,
}

pub struct Data {
    status: u16,
    data: String,
    headers: HashMap<String, String>,
}

fn get_headers(input: &HeaderMap) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();
    for (key, value) in input {
        let k = String::from(key.as_str());
        let v = String::from(value.to_str().unwrap_or(""));
        headers.insert(k, v);
    }

    return headers;
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("url", &self.url)
            .field("headers", &self.headers)
            .finish()
    }
}

impl Request {
    pub fn new<T: AsRef<str>>(method: Method, url: T) -> Self {
        return Self {
            url: url.as_ref().to_string(),
            method: method,
            inner: None,
            headers: HashMap::new(),
            error: "".into(),
            timeout: 10,
            gzip: false,
        };
    }

    /// Whether to enable gzip
    pub fn gzip(&mut self, zip: bool) -> &mut Self {
        self.gzip = zip;
        return self;
    }

    /// Set timeout as seconds
    pub fn timeout(&mut self, seconds: i32) -> &mut Self {
        self.timeout = seconds;
        return self;
    }

    /// Insert header before send
    pub fn header<K: Into<String>>(&mut self, key: K, value: K) -> &mut Self {
        self.headers.insert(key.into(), value.into());
        return self;
    }

    fn build(&self) -> RequestBuilder {
        let c = self.inner.as_ref().unwrap();
        let mut x = c.request(self.method.clone(), self.url.clone());
        for (key, value) in &self.headers {
            x = x.header(key, value);
        }
        return x;
    }

    /// Send the request
    pub fn send<DATA: Into<Body>, F: FnMut(Data)>(&mut self, data: DATA, mut f: F) -> i32 {
        // build client once
        if self.inner.is_none() {
            let c = Client::builder()
                .gzip(self.gzip)
                .timeout(Duration::from_secs(self.timeout as u64))
                .build();
            match c {
                Ok(client) => {
                    self.inner = Some(client);
                }
                Err(e) => {
                    self.error = e.to_string();
                    return -1;
                }
            }
        }

        // build request
        let mut x = self.build();
        x = x.body(data);

        // send
        let ret = x.send();
        if let Err(e) = ret {
            self.error = e.to_string();
            return -1;
        }

        let response = ret.unwrap();
        let status = response.status().as_u16();
        let headers = get_headers(response.headers());

        // moved occured inside of the text()
        let t = response.text();
        if let Err(e) = t {
            self.error = e.to_string();
            return -1;
        }

        let upcall = Data {
            status: status,
            data: t.unwrap(),
            headers: headers,
        };

        f(upcall);

        return 0;
    }
}

impl Data {
    /// Get HTTP status code
    pub fn status(&self) -> u16 {
        return self.status;
    }

    /// Get reponse as text
    pub fn text(&self) -> &String {
        return &self.data;
    }

    /// Get reponse as JSON
    pub fn json(&self) -> serde_json::Value {
        let value = serde_json::Value::from_str(self.data.as_str());
        return value.unwrap_or(serde_json::Value::Null);
    }

    /// Get reponse headers
    pub fn headers(&self) -> &HashMap<String, String> {
        return &self.headers;
    }
}

/// Most simple way to make a http get request, the gzip was enabled by default
///
/// ```rust
/// fn test() {
///     wsd::http::get("https://docs.rs/", |data| {
///         println!("status = {}, data = {}", data.status(), data.text());
///     });
/// }
/// ```
pub fn get<URL, F>(url: URL, mut f: F)
where
    URL: AsRef<str>,
    F: FnMut(Data),
{
    let g = || -> Result<Response, Box<dyn std::error::Error>> {
        let client = Client::builder().gzip(true).build()?;
        let ret = client.get(url.as_ref()).send()?;
        return Ok(ret);
    };

    // default as connection timed out
    let mut data = Data {
        status: 522,
        data: "".to_string(),
        headers: HashMap::new(),
    };

    if let Ok(response) = g() {
        data.status = response.status().as_u16();
        data.headers = get_headers(response.headers());
        data.data = response.text().unwrap_or("".to_string());
    }

    // result
    f(data);
}

/// Most simple way to make a http post request, the gzip will be enabled if data greater than 1 KB.
///
/// ```rust
/// fn test() {
///     wsd::http::post("https://docs.rs/", "{id: 100}", |data| {
///         println!("status = {}, data = {}", data.status(), data.text());
///     });
/// }
/// ```
pub fn post<URL, BODY, F>(url: URL, body: BODY, mut f: F)
where
    URL: AsRef<str>,
    BODY: Into<Body> + AsRef<[u8]>,
    F: FnMut(Data),
{
    let g = |x: BODY| -> Result<Response, Box<dyn std::error::Error>> {
        // enable zip if data reaches MTU (1300 - 1500)
        let zip = x.as_ref().len() > 1024;
        let client = Client::builder().gzip(zip).build()?;
        let ret = client.post(url.as_ref()).body(x).send()?;
        return Ok(ret);
    };

    // default as connection timed out
    let mut data = Data {
        status: 522,
        data: "".to_string(),
        headers: HashMap::new(),
    };

    if let Ok(response) = g(body) {
        data.status = response.status().as_u16();
        data.headers = get_headers(response.headers());
        data.data = response.text().unwrap_or("".to_string());
    }

    // result
    f(data);
}
