use base64::{engine::general_purpose::STANDARD, Engine as _};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{env, error::Error, fs, io, path::PathBuf};

const PLACEHOLDER: &str = "{{TRUNK_INLINE_SCRIPT_HASHES}}";
const MAX_HEADER_LINE_LENGTH: usize = 2_000;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn inline_script_hashes(html: &str) -> Result<String> {
    let script_pattern = Regex::new(r"(?is)<script\b(?P<attributes>[^>]*)>(?P<body>.*?)</script>")?;
    let src_pattern = Regex::new(r"(?i)(?:^|\s)src\s*=")?;
    let mut hashes = Vec::new();

    for captures in script_pattern.captures_iter(html) {
        let attributes = &captures["attributes"];
        if src_pattern.is_match(attributes) {
            continue;
        }

        let digest = Sha256::digest(captures["body"].as_bytes());
        let source = format!("sha256-{}", STANDARD.encode(digest));
        if !hashes.contains(&source) {
            hashes.push(source);
        }
    }

    if hashes.is_empty() {
        return Err(
            io::Error::other("Trunk output contains no inline scripts to authorize").into(),
        );
    }

    Ok(hashes
        .into_iter()
        .map(|source| format!("\u{27}{source}\u{27}"))
        .collect::<Vec<_>>()
        .join(" "))
}

fn render_headers(template: &str, generated_html: &str) -> Result<String> {
    if template.matches(PLACEHOLDER).count() != 1 {
        return Err(
            io::Error::other(format!("Expected exactly one {PLACEHOLDER} placeholder")).into(),
        );
    }

    let headers = template.replace(PLACEHOLDER, &inline_script_hashes(generated_html)?);
    if headers
        .lines()
        .any(|line| line.chars().count() > MAX_HEADER_LINE_LENGTH)
    {
        return Err(io::Error::other(
            "Generated headers exceed the Cloudflare 2,000-character line limit",
        )
        .into());
    }

    Ok(headers)
}

fn main() -> Result<()> {
    let staging_dir = env::var_os("TRUNK_STAGING_DIR")
        .ok_or_else(|| io::Error::other("TRUNK_STAGING_DIR is not set"))?;
    let staging_dir = PathBuf::from(staging_dir);
    let generated_html = fs::read_to_string(staging_dir.join("index.html"))?;
    let template_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/_headers.template");
    let template = fs::read_to_string(template_path)?;
    let headers = render_headers(&template, &generated_html)?;
    let output_path = staging_dir.join("_headers");

    fs::write(&output_path, headers)?;
    println!(
        "Generated {} with CSP hashes for Trunk inline scripts",
        output_path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_inline_scripts_and_ignores_external_scripts() {
        let html = r#"<script type="module">console.log("Trunk");</script>
<script src="/external.js"></script>"#;

        assert_eq!(
            inline_script_hashes(html).unwrap(),
            "\u{27}sha256-N26yhoY9aqULnY9P7JgiH95M1t0xNQ4KJlIqoJBieCA=\u{27}"
        );
    }

    #[test]
    fn deduplicates_identical_inline_scripts() {
        let html = "<script>same</script><script>same</script>";
        let hashes = inline_script_hashes(html).unwrap();

        assert_eq!(hashes.matches("sha256-").count(), 1);
    }

    #[test]
    fn rejects_html_without_inline_scripts() {
        let error = inline_script_hashes(r#"<script src="/external.js"></script>"#).unwrap_err();

        assert!(error.to_string().contains("no inline scripts to authorize"));
    }

    #[test]
    fn renders_the_header_template() {
        let template = "Content-Security-Policy: script-src {{TRUNK_INLINE_SCRIPT_HASHES}};";
        let headers = render_headers(template, "<script>same</script>").unwrap();

        assert!(!headers.contains(PLACEHOLDER));
        assert!(headers.contains("script-src \u{27}sha256-"));
    }
}
