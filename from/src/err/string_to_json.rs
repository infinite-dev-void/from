#[inline(always)]
pub fn string_to_json(s: &String) -> String {
    let mut json = String::from('"');
    json.push_str(&s.replace('\\', r#"\\"#).replace('"', r#"\""#));
    json.push('"');
    json
}
