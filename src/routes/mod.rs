use lazy_static::lazy_static;
use tera::Tera;

mod health_check;
mod home;
mod login;
mod newsletter;
mod subscriptions;
mod subscriptions_confirm;

pub use health_check::*;
pub use home::*;
pub use login::*;
pub use newsletter::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
