/*
 * Loco REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.24
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `flag_translation`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlagTranslationError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_translation`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTranslationError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_translations`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTranslationsError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `translate`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranslateError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `unflag_translation`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnflagTranslationError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `untranslate`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UntranslateError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}


    pub async fn flag_translation(configuration: &configuration::Configuration, id: &str, locale: &str, flag: &str, key: Option<&str>) -> Result<crate::models::Success, Error<FlagTranslationError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/translations/{id}/{locale}/flag", configuration.base_path, id=crate::apis::urlencode(id), locale=crate::apis::urlencode(locale));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        let mut form = reqwest::multipart::Form::new();
        form = form.text("flag", flag.to_string());
        req_builder = req_builder.multipart(form);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FlagTranslationError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_translation(configuration: &configuration::Configuration, id: &str, locale: &str, key: Option<&str>) -> Result<crate::models::Translation, Error<GetTranslationError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/translations/{id}/{locale}", configuration.base_path, id=crate::apis::urlencode(id), locale=crate::apis::urlencode(locale));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetTranslationError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_translations(configuration: &configuration::Configuration, id: &str, key: Option<&str>) -> Result<Vec<crate::models::Translation>, Error<GetTranslationsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/translations/{id}.json", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetTranslationsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn translate(configuration: &configuration::Configuration, id: &str, locale: &str, key: Option<&str>, data: Option<&str>) -> Result<crate::models::Translation, Error<TranslateError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/translations/{id}/{locale}", configuration.base_path, id=crate::apis::urlencode(id), locale=crate::apis::urlencode(locale));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&data);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<TranslateError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn unflag_translation(configuration: &configuration::Configuration, id: &str, locale: &str, key: Option<&str>) -> Result<crate::models::Success, Error<UnflagTranslationError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/translations/{id}/{locale}/flag", configuration.base_path, id=crate::apis::urlencode(id), locale=crate::apis::urlencode(locale));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UnflagTranslationError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn untranslate(configuration: &configuration::Configuration, id: &str, locale: &str, key: Option<&str>) -> Result<crate::models::Success, Error<UntranslateError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/translations/{id}/{locale}", configuration.base_path, id=crate::apis::urlencode(id), locale=crate::apis::urlencode(locale));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UntranslateError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

