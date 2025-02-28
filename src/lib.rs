pub fn to_nerf_dart(uri: &str) -> String {
    let parsed = url::Url::parse(uri).unwrap();

    // Remove unwanted components
    let host = parsed.host_str().unwrap_or_default();
    let port = parsed.port().map(|p| format!(":{}", p)).unwrap_or_default();

    let mut path = parsed.path_segments().unwrap().map(|s|s.to_string()).collect::<Vec<String>>();

    if let Some(last) = path.last() {
        if !last.is_empty() {
            path.pop();
            path.push("".to_string());
        }
    }

    let mut path = path.join("/");
    if !path.starts_with("/") {
        path.insert(0, '/');
    }

    let final_url = format!("//{}{}{}", host, port, path);

    final_url
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_nerf_dart(uri: &str, valid: Option<&str>) {
        let valid = valid.unwrap_or("//registry.npmjs.org/");

        assert_eq!(to_nerf_dart(uri), valid);
    }

    #[test]
    fn nerf_date_runs() {
        valid_nerf_dart("http://registry.npmjs.org", None);
        valid_nerf_dart("http://registry.npmjs.org/some-package", None);
        valid_nerf_dart("http://registry.npmjs.org/some-package?write=true", None);
        valid_nerf_dart("http://user:pass@registry.npmjs.org/some-package?write=true", None);
        valid_nerf_dart("http://registry.npmjs.org/#random-hash", None);
        valid_nerf_dart("http://registry.npmjs.org/some-package#random-hash", None);
    }

    #[test]
    fn couch_nerf_dart() {
        valid_nerf_dart("http://relative.couchapp.npm:8080/design/-/rewrite/", Some("//relative\
        .couchapp.npm:8080/design/-/rewrite/"));
        valid_nerf_dart("http://relative.couchapp.npm:8080/design/-/rewrite/", Some("//relative.couchapp.npm:8080/design/-/rewrite/"))
    }

    #[test]
    fn couch_nerf_dart_2() {
        valid_nerf_dart("http://relative.couchapp.npm:8080/design/-/rewrite/some-package",
                        Some("//relative.couchapp.npm:8080/design/-/rewrite/"));
    }
}
