use bindings::*;
use microsoft::web::web_view2::core::*;

fn main() -> winrt::Result<()> {
    let options = CoreWebView2EnvironmentOptions::new()?;

    options.set_language("croak")?;

    println!("{}", options.language()?);

    Ok(())
}
