# \LocalesApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_locale**](LocalesApi.md#create_locale) | **post** /locales | Add a new project locale
[**delete_locale**](LocalesApi.md#delete_locale) | **delete** /locales/{locale} | Delete a project locale
[**get_locale**](LocalesApi.md#get_locale) | **get** /locales/{locale} | Get a project locale
[**get_locale_errors**](LocalesApi.md#get_locale_errors) | **get** /locales/{locale}/errors | Get validation errors for a project locale
[**get_locale_progress**](LocalesApi.md#get_locale_progress) | **get** /locales/{locale}/progress | Get more detailed translation progress for a project locale
[**get_locales**](LocalesApi.md#get_locales) | **get** /locales | List all locales in your project
[**patch_locale**](LocalesApi.md#patch_locale) | **patch** /locales/{locale} | Modify a project locale



## create_locale

> crate::models::Locale create_locale(code, key)
Add a new project locale

<p>Adds a new locale to the currently authenticated project.</p> <p>If the locale is created successfully the response will be 201 (created).</p> <p>If the locale already exists the response will be 409 (conflict).</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Short code, or language tag for new locale | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Locale**](Locale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_locale

> crate::models::Success delete_locale(locale, key)
Delete a project locale

<p>Delete a locale from currently authenticated project.</p> <p><strong>Warning</strong>: This will permanently delete any translations made in the specified locale across your project</p> <p>Note that you cannot delete your source locale.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** | Locale short code, or language tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locale

> crate::models::Locale get_locale(locale, key)
Get a project locale

Gets a single locale in currently authenticated project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** | Locale short code, or language tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Locale**](Locale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locale_errors

> Vec<crate::models::TranslationError> get_locale_errors(locale, key)
Get validation errors for a project locale

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** | Locale short code, or language tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**Vec<crate::models::TranslationError>**](TranslationError.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locale_progress

> serde_json::Value get_locale_progress(locale, key)
Get more detailed translation progress for a project locale

<p>Response model is dynamic according to the status/flags available in your project.</p> <p>Status keys such as 'translated' or 'fuzzy' are only included when there are assets with that status and in future may be user-defined.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** | Locale short code, or language tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locales

> Vec<crate::models::Locale> get_locales(key)
List all locales in your project

Lists all locales in currently authenticated project. Your source language will always be the first in the list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**Vec<crate::models::Locale>**](Locale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_locale

> crate::models::Locale patch_locale(locale, data, key)
Modify a project locale

<p>Modifies the properties of a locale in the currently authenticated project.</p> <p>The full, modified locale object is returned.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** | Locale short code, or language tag | [required] |
**data** | [**LocalePatch**](LocalePatch.md) | Partial locale data to update | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Locale**](Locale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

