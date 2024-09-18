use tera::Tera;
use anyhow::Result;

pub fn get_tera() -> Result<Tera> {
    let tera = Tera::new("templates/**/*.html")?;
    Ok(tera)
}
