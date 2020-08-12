# \PingApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ping**](PingApi.md#ping) | **get** /ping | Check the API is up
[**ping404**](PingApi.md#ping404) | **get** /ping/not-found | Get a test 404 response



## ping

> crate::models::PingResponse ping()
Check the API is up

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PingResponse**](PingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping404

> crate::models::Error ping404()
Get a test 404 response

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Error**](Error.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

