# PluralTranslation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Asset ID | 
**status** | Option<**String**> | Status of translation as string compatible with export status parameter | [optional]
**flagged** | **bool** | Whether translation is flagged as requiring attention | 
**translated** | **bool** | Whether asset is translated and contributing to project completion | 
**translation** | **String** | Translated text in specified locale | 
**revision** | **i32** | Number of edits made, zero if never translated | 
**comments** | **i32** | Number of comments available | 
**modified** | Option<**String**> | Time last modified in UTC, null if translation doesn't exist | [optional]
**author** | Option<[**crate::models::User**](User.md)> |  | [optional]
**flagger** | Option<[**crate::models::User**](User.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


