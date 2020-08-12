# \TranslationsApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**flag_translation**](TranslationsApi.md#flag_translation) | **post** /translations/{id}/{locale}/flag | Flag a translation in a given locale
[**get_translation**](TranslationsApi.md#get_translation) | **get** /translations/{id}/{locale} | Get a single translation
[**get_translations**](TranslationsApi.md#get_translations) | **get** /translations/{id}.json | Get all translations of an asset
[**translate**](TranslationsApi.md#translate) | **post** /translations/{id}/{locale} | Add a new translation in a given locale
[**unflag_translation**](TranslationsApi.md#unflag_translation) | **delete** /translations/{id}/{locale}/flag | Clear flag from a translation
[**untranslate**](TranslationsApi.md#untranslate) | **delete** /translations/{id}/{locale} | Erase translation data in a single locale



## flag_translation

> crate::models::Success flag_translation(id, locale, flag, key)
Flag a translation in a given locale

<p>Flag a single translation or set its status in a given locale.</p> <p>The <code>flag</code> parameter can be any flag (e.g. fuzzy) a status (e.g. translated) or a redundant state (e.g. blank translation). <a href=\"https://localise.biz/help/glossary/flag\">See translation states</a>.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**locale** | **String** | Locale short code, or language tag | [required] |
**flag** | **String** | Flag or status to set, e.g. \\\"fuzzy\\\" | [required] |[default to fuzzy]
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translation

> crate::models::Translation get_translation(id, locale, key)
Get a single translation

<p>Gets a single translation in currently authenticated project.</p> <p>Translations implicitly exist as long as the asset and locale are in your project.</p> <p>Translations marked as complete have the <code>translated</code> field set to <code>true</code>.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**locale** | **String** | Locale short code, or language tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Translation**](Translation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translations

> Vec<crate::models::Translation> get_translations(id, key)
Get all translations of an asset

<p>Gets all translations of an asset across the currently authenticated project's locales.</p> <p>Locales not yet translated are included, but their <code>translated</code> field will be set to <code>false</code>.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**Vec<crate::models::Translation>**](Translation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate

> crate::models::Translation translate(id, locale, key, data)
Add a new translation in a given locale

<p>Translates a single asset in a single locale in the currently authenticated project.</p> <p>If the asset is already translated, a new revision will be added and the <code>revision</code> field incremented.</p> <p>If the asset is untranslated the locale must have already been added to the project.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**locale** | **String** | Locale short code, or language tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |
**data** | Option<**String**> | Raw value of new translation. Sending empty marks as 'untranslated' |  |

### Return type

[**crate::models::Translation**](Translation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain, text/html, multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unflag_translation

> crate::models::Success unflag_translation(id, locale, key)
Clear flag from a translation

<p>Removes current flag from a translation marked as incomplete or in error.</p> <p>It's not necessary to specify which flag to remove, because there can be only one.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
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


## untranslate

> crate::models::Success untranslate(id, locale, key)
Erase translation data in a single locale

<p>Erases translation data of a localized asset in the currently authenticated project.</p> <p><strong>Warning</strong>: Erasing is not the same as setting an empty translation.  This operation clears the asset's translation history and user comments for the given locale.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
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

