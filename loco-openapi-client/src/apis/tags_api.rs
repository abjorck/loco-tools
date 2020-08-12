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


/// struct for typed errors of method `create_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTagError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_tags`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagsError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchTagError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `tag_assets`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagAssetsError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}


    pub async fn create_tag(configuration: &configuration::Configuration, name: &str, key: Option<&str>) -> Result<crate::models::Success, Error<CreateTagError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/tags.json", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", name.to_string());
        req_builder = req_builder.multipart(form);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<CreateTagError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn delete_tag(configuration: &configuration::Configuration, tag: &str, key: Option<&str>) -> Result<crate::models::Success, Error<DeleteTagError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/tags/{tag}.json", configuration.base_path, tag=crate::apis::urlencode(tag));
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
            let entity: Option<DeleteTagError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_tags(configuration: &configuration::Configuration, key: Option<&str>) -> Result<Vec<String>, Error<GetTagsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/tags.json", configuration.base_path);
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
            let entity: Option<GetTagsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn patch_tag(configuration: &configuration::Configuration, tag: &str, data: crate::models::TagPatch, key: Option<&str>) -> Result<crate::models::Success, Error<PatchTagError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/tags/{tag}.json", configuration.base_path, tag=crate::apis::urlencode(tag));
        let mut req_builder = client.patch(uri_str.as_str());

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
            let entity: Option<PatchTagError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn tag_assets(configuration: &configuration::Configuration, tag: &str, data: &str, key: Option<&str>) -> Result<crate::models::Success, Error<TagAssetsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/tags/{tag}.json", configuration.base_path, tag=crate::apis::urlencode(tag));
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
            let entity: Option<TagAssetsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }
