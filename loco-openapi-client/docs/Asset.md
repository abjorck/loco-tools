# Asset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique asset identifier | 
**_type** | **String** | Content type for translations | 
**context** | **String** | Optional context descriptor | 
**notes** | **String** | Optional notes for translators | 
**modified** | **String** | Time last modified in UTC | 
**plurals** | **i32** | Number of associated plural forms | 
**tags** | **Vec<String>** | Array of terms asset is tagged with | 
**aliases** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [optional]
**progress** | Option<[**crate::models::AssetProgress**](AssetProgress.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


