use actix_web::HttpResponse;

pub trait WebResourceProvider: Send + Sync {
    fn get_name(&self) -> String;

    fn handle_web_resource(&self, path: String) -> HttpResponse;
}
