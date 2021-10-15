use bindings::*;
use Microsoft::Web::WebView2::Core::*;

fn main() -> windows::Result<()> {
    let options = CoreWebView2EnvironmentOptions::new()?;

    options.SetLanguage("croak")?;

    println!("{}", options.Language()?);

    Ok(())
}
