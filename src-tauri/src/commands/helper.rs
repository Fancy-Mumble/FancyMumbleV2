use tauri::regex::Regex;

pub(crate) fn extract_og_property(body: &str, pattern: &str) -> Result<String, String> {
    let re = Regex::new(pattern).map_err(|e| format!("{e:?}"))?;
    let property = re
        .captures(body)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .map(String::from)
        .ok_or("regex not found")?;

    Ok(property)
}
