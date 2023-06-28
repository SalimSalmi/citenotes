use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};  
use citenotes::CiteNoteError;
use std::error::Error;

fn request(url: &String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/x-bibtex; charset=utf-8"),
    );

    let client = reqwest::blocking::Client::new();
    client.get(url).headers(headers).send()
}

fn response_error(response: &reqwest::blocking::Response) -> Result<&reqwest::blocking::Response, CiteNoteError> {
    if response.status().is_success() {
        Ok(response)
    } else {
        let status = response.status();
        let message = format!("Request failed with error response: {}", status);
        Err(CiteNoteError::new(message.as_str()))
    } 
}

pub fn request_bib_with_doi(url: &String) -> Result<String, Box<dyn Error>> {
    let response = request(url)?;
    response_error(&response)?;

    let body = response.text()?;
    Ok(body)
}
