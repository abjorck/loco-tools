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


/// struct for typed errors of method `import`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportError {
    Status401(),
    Status403(),
    Status422(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `import_progress`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportProgressError {
    Status401(),
    Status403(),
    Status422(),
    UnknownValue(serde_json::Value),
}


    pub async fn import(configuration: &configuration::Configuration, ext: &str, data: &str, index: Option<&str>, locale: Option<&str>, _async: Option<bool>, path: Option<&str>, ignore_new: Option<bool>, ignore_existing: Option<bool>, tag_new: Option<&str>, tag_all: Option<&str>, untag_all: Option<&str>, tag_updated: Option<&str>, untag_updated: Option<&str>, tag_absent: Option<&str>, untag_absent: Option<&str>, delete_absent: Option<bool>, key: Option<&str>) -> Result<crate::models::ImportResult, Error<ImportError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/import/{ext}", configuration.base_path, ext=crate::apis::urlencode(ext));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("index", &s.to_string())]);
        }
        if let Some(ref s) = locale {
            req_builder = req_builder.query(&[("locale", &s.to_string())]);
        }
        if let Some(ref s) = _async {
            req_builder = req_builder.query(&[("async", &s.to_string())]);
        }
        if let Some(ref s) = path {
            req_builder = req_builder.query(&[("path", &s.to_string())]);
        }
        if let Some(ref s) = ignore_new {
            req_builder = req_builder.query(&[("ignore-new", &s.to_string())]);
        }
        if let Some(ref s) = ignore_existing {
            req_builder = req_builder.query(&[("ignore-existing", &s.to_string())]);
        }
        if let Some(ref s) = tag_new {
            req_builder = req_builder.query(&[("tag-new", &s.to_string())]);
        }
        if let Some(ref s) = tag_all {
            req_builder = req_builder.query(&[("tag-all", &s.to_string())]);
        }
        if let Some(ref s) = untag_all {
            req_builder = req_builder.query(&[("untag-all", &s.to_string())]);
        }
        if let Some(ref s) = tag_updated {
            req_builder = req_builder.query(&[("tag-updated", &s.to_string())]);
        }
        if let Some(ref s) = untag_updated {
            req_builder = req_builder.query(&[("untag-updated", &s.to_string())]);
        }
        if let Some(ref s) = tag_absent {
            req_builder = req_builder.query(&[("tag-absent", &s.to_string())]);
        }
        if let Some(ref s) = untag_absent {
            req_builder = req_builder.query(&[("untag-absent", &s.to_string())]);
        }
        if let Some(ref s) = delete_absent {
            req_builder = req_builder.query(&[("delete-absent", &s.to_string())]);
        }
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
            let entity: Option<ImportError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn import_progress(configuration: &configuration::Configuration, id: &str, key: Option<&str>) -> Result<crate::models::AsyncProgress, Error<ImportProgressError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/import/progress/{id}", configuration.base_path, id=crate::apis::urlencode(id));
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
            let entity: Option<ImportProgressError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

