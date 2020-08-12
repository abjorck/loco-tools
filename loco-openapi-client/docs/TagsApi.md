# \TagsApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag**](TagsApi.md#create_tag) | **post** /tags.json | Create a new tag
[**delete_tag**](TagsApi.md#delete_tag) | **delete** /tags/{tag}.json | Delete an existing tag
[**get_tags**](TagsApi.md#get_tags) | **get** /tags.json | Get project tags
[**patch_tag**](TagsApi.md#patch_tag) | **patch** /tags/{tag}.json | Modify a single tag
[**tag_assets**](TagsApi.md#tag_assets) | **post** /tags/{tag}.json | Add multiple assets to an existing tag



## create_tag

> crate::models::Success create_tag(name, key)
Create a new tag

Adds a new tag to the currently authenticated project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of new tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> crate::models::Success delete_tag(tag, key)
Delete an existing tag

Deletes an existing tag in the currently authenticated project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Name of a single asset tag. | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> Vec<String> get_tags(key)
Get project tags

Lists all tags in currently authenticated project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_tag

> crate::models::Success patch_tag(tag, data, key)
Modify a single tag

Renames an existing tag in the currently authenticated project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Name of a single asset tag. | [required] |
**data** | [**TagPatch**](TagPatch.md) | Partial tag data to update (currently name only) | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_assets

> crate::models::Success tag_assets(tag, data, key)
Add multiple assets to an existing tag

Tag multiple assets at once by posting their unique IDs to the tag's resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Name of a single asset tag. | [required] |
**data** | **String** | Comma separated list of unique asset IDs | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

