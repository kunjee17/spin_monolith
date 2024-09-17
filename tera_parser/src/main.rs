use anyhow::{Context, Result};
use minify_html::{minify, Cfg};
use tokio::fs;
use tokio::io::AsyncReadExt;
use std::path::Path;
use path_slash::PathExt;
use futures::future::BoxFuture;

// Box the future to avoid recursion issues
fn compress_html_files<'a>(dir: &'a str, base_dir: &'a str) -> BoxFuture<'a, Result<Vec<(String, String)>>> {
    Box::pin(async move {
        let mut results = Vec::new();

        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                let nested_results = compress_html_files(
                    path.to_str().context("Invalid directory path")?,
                    base_dir
                ).await?;
                results.extend(nested_results);
            } else if path.is_file() {
                if path.extension().map_or(false, |ext| ext == "html") {
                    let content = read_and_minify_file(&path).await?;
                    let relative_path = path
                        .strip_prefix(base_dir)
                        .with_context(|| format!("Failed to get relative path for: {:?}", path))?
                        .to_slash()
                        .context("Failed to convert path to slash-based format")?
                        ;

                    results.push((relative_path.trim_end_matches(".html").to_string(), content));
                }
            }
        }
        Ok(results)
    })
}

async fn read_and_minify_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await
        .with_context(|| format!("Failed to read file: {:?}", path))?;

    let cfg = Cfg::new();
    let minified = minify(content.as_bytes(), &cfg);
    let minified_str = String::from_utf8(minified)
        .context("Failed to convert minified content to String")?;

    Ok(minified_str)
}

#[tokio::main]
async fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let dir = format!("{}/../templates", current_dir.to_string_lossy());
    println!("Reading files from: {}", dir);

    let compressed_files = compress_html_files(&dir, &dir).await?;

    let template_content = format!(
        r#"use tera::Tera;
use anyhow::Result;

pub fn get_tera() -> Result<Tera> {{
    let mut tera = Tera::default();
    tera.add_raw_templates(vec!{:#?})?;
    Ok(tera)
}}
"#,
        compressed_files
    );

    let output_path = format!("{}/../src/templates.rs", current_dir.to_string_lossy());
    fs::write(output_path, template_content)
        .await
        .context("Failed to write to src/templates.rs")?;

    Ok(())
}
