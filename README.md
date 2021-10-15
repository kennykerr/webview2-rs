# webview2-rs

This is a simple example of using [the `windows` crate](https://github.com/microsoft/windows-rs) with WinRT dependencies like [WebView2](https://www.nuget.org/packages/Microsoft.Web.WebView2) where that component provides its own winmd for describing its API surface as well as a runtime DLL that must be deployed with the app. It is further complicated as WebView2 requires the [VCRTForwarders](https://www.nuget.org/packages/Microsoft.VCRTForwarders.140/) in order to load. This repo provides an example of how this can be achieved.

Here I have simply unpacked the respective dependencies and placed them in the well-known `.windows` folder that the `windows` crate expects.
