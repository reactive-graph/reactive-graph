# Web Resource Provider

Plugins can implement the trait `WebResourceProvider` in order to register handlers for endpoints.

## Use Cases

* Deliver static files & web applications
* Provide REST endpoints

## Trait `WebResourceProvider`

```rust
impl WebResourceProvider for ExampleWebResourceProviderImpl {
    fn get_base_path(&self) -> String {
        String::from("example")
    }

    fn handle_web_resource(
        &self,
        path: String,
        _request: Request<HttpBody>,
    ) -> Result<Response<HttpBody>> {
        let asset = FlowEditorWebResourceAsset::get(path.as_ref());
        match asset {
            Some(asset) => {
                let body: HttpBody = match asset.data {
                    Cow::Borrowed(bytes) => HttpBody::Binary(bytes.to_vec()),
                    Cow::Owned(bytes) => HttpBody::Binary(bytes.to_vec()),
                };
                let mime_type = from_path(path.as_str()).first_or_octet_stream();
                Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, mime_type.to_string())
                    .body(body)
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(HttpBody::None),
        }
    }
}
```
