# \AuthApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_verify**](AuthApi.md#auth_verify) | **get** /auth/verify | Verify an API project key



## auth_verify

> crate::models::Credentials auth_verify(key)
Verify an API project key

<p>Loco API keys authenticate your user account for accessing a specific project.<br />This endpoint verifies an API key and returns the authenticated user, account and project.</p> <p>If you want to verify whether the key has write access, just send this endpoint a POST request instead. A read-only key will give 403 for any POST request.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Credentials**](Credentials.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

