# \ImportApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**import**](ImportApi.md#import) | **post** /import/{ext} | Import assets and translations from a language pack file
[**import_progress**](ImportApi.md#import_progress) | **get** /import/progress/{id} | Check the progress of an asynchronous import



## import

> crate::models::ImportResult import(ext, data, index, locale, _async, path, ignore_new, ignore_existing, tag_new, tag_all, untag_all, tag_updated, untag_updated, tag_absent, untag_absent, delete_absent, key)
Import assets and translations from a language pack file

<p>The import API loads translations from various language pack formats into the currently authenticated project.</p> <p>Take note of how the <code>index</code> and <code>locale</code> parameters are used to describe how your file will be imported.  By leaving these fields empty Loco will try to guess your intentions, but it's advisable to specify all parameters if in any doubt. <a href=\"https://localise.biz/api#imports\">See examples</a>.</p> <p>It's recommended that you set <code>async=1</code> for large files. This will cause the import to run in the background. When running asynchronously you will receive a 201 code (instead of 200) and the <code>Location</code> header will contain a URI for checking progress.</p> <p>When running synchronously the response object will include a list of all new assets add to the project.  Additionally all responses will include the state of each locale referenced by the import (source and target).</p> <p>See <a href=\"https://localise.biz/help/formats/importing/settings\">Import settings</a> for full details on all these options.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ext** | **String** | File extension for the type of file data you're importing | [required] |
**data** | **String** | Raw source of file being imported | [required] |
**index** | Option<**String**> | Specify whether translations in your file are indexed by generic IDs or human-readable source text |  |
**locale** | Option<**String**> | Specify target locale if importing translations |  |
**_async** | Option<**bool**> | Specify that import should be done asynchronously (recommended for large files) |  |
**path** | Option<**String**> | Specify original file path for source code references (excluding line number) |  |
**ignore_new** | Option<**bool**> | Specify that new assets will NOT be added to the project |  |
**ignore_existing** | Option<**bool**> | Specify that existing assets encountered in the file will NOT be updated |  |
**tag_new** | Option<**String**> | Tag any NEW assets added during the import with the given tags (comma separated) |  |
**tag_all** | Option<**String**> | Tag ALL assets in the file with the given tags (comma separated) |  |
**untag_all** | Option<**String**> | Remove existing tags from any assets matched in the imported file (comma separated) |  |
**tag_updated** | Option<**String**> | Tag existing assets that are MODIFIED by this import |  |
**untag_updated** | Option<**String**> | Remove existing tags from assets that are MODIFIED during import |  |
**tag_absent** | Option<**String**> | Tag existing assets in the project that are NOT found in the imported file |  |
**untag_absent** | Option<**String**> | Remove existing tags from assets NOT found in the imported file |  |
**delete_absent** | Option<**bool**> | Permanently DELETES project assets NOT found in the file (use with extreme caution) |  |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::ImportResult**](ImportResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_progress

> crate::models::AsyncProgress import_progress(id, key)
Check the progress of an asynchronous import

<p>If you specified <code>async=1</code> in your original import API request, you can check on its progress with this endpoint.</p> <p>The full URL including job identifier will have been provided in the Location header of your original import response</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Job identifier from original import action | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::AsyncProgress**](AsyncProgress.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

