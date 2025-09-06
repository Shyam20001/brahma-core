use napi::Error;
use napi_derive::napi;

#[napi]
pub fn parse_file(path: String) -> Result<String, Error> {
    let content = std::fs::read_to_string(path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(content)
}
