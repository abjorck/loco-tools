# ImportResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **i32** | Whether import was synchronous (200) or asynchronous (201) | 
**message** | **String** | Summary of import result | 
**locales** | [**Vec<crate::models::Locale>**](Locale.md) | Locales identified according to `index` and `locale` parameters | 
**assets** | [**Vec<crate::models::Asset>**](Asset.md) | New assets added to project (always empty when async=1) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


