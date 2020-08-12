use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    assets_api: Box<dyn crate::apis::AssetsApi>,
    auth_api: Box<dyn crate::apis::AuthApi>,
    export_api: Box<dyn crate::apis::ExportApi>,
    import_api: Box<dyn crate::apis::ImportApi>,
    locales_api: Box<dyn crate::apis::LocalesApi>,
    ping_api: Box<dyn crate::apis::PingApi>,
    tags_api: Box<dyn crate::apis::TagsApi>,
    translations_api: Box<dyn crate::apis::TranslationsApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            assets_api: Box::new(crate::apis::AssetsApiClient::new(rc.clone())),
            auth_api: Box::new(crate::apis::AuthApiClient::new(rc.clone())),
            export_api: Box::new(crate::apis::ExportApiClient::new(rc.clone())),
            import_api: Box::new(crate::apis::ImportApiClient::new(rc.clone())),
            locales_api: Box::new(crate::apis::LocalesApiClient::new(rc.clone())),
            ping_api: Box::new(crate::apis::PingApiClient::new(rc.clone())),
            tags_api: Box::new(crate::apis::TagsApiClient::new(rc.clone())),
            translations_api: Box::new(crate::apis::TranslationsApiClient::new(rc.clone())),
        }
    }

    pub fn assets_api(&self) -> &dyn crate::apis::AssetsApi{
        self.assets_api.as_ref()
    }

    pub fn auth_api(&self) -> &dyn crate::apis::AuthApi{
        self.auth_api.as_ref()
    }

    pub fn export_api(&self) -> &dyn crate::apis::ExportApi{
        self.export_api.as_ref()
    }

    pub fn import_api(&self) -> &dyn crate::apis::ImportApi{
        self.import_api.as_ref()
    }

    pub fn locales_api(&self) -> &dyn crate::apis::LocalesApi{
        self.locales_api.as_ref()
    }

    pub fn ping_api(&self) -> &dyn crate::apis::PingApi{
        self.ping_api.as_ref()
    }

    pub fn tags_api(&self) -> &dyn crate::apis::TagsApi{
        self.tags_api.as_ref()
    }

    pub fn translations_api(&self) -> &dyn crate::apis::TranslationsApi{
        self.translations_api.as_ref()
    }

}
