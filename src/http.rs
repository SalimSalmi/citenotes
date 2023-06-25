use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};  

pub fn request_bib_with_doi(url: &String) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/bibliography; style=bibtex"),
    );

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).headers(headers).send()?;
    let body = response.text()?;

    Ok(body)
}
