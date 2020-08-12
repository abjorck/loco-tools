# TranslationError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique asset ID for current translation | 
**status** | **String** | Status of translation as string compatible with export status parameter | 
**source** | **String** | Source text in default project locale | 
**target** | Option<**String**> | Translated text in current target locale (unless locale is source) | [optional]
**errors** | **Vec<String>** | Validation errors as simple message strings | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


