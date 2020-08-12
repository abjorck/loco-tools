# \ExportApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_all**](ExportApi.md#export_all) | **get** /export/all.{ext} | Export your whole project to a multi-locale language pack
[**export_archive**](ExportApi.md#export_archive) | **get** /export/archive/{ext}.zip | Export all locales to a zip archive
[**export_locale**](ExportApi.md#export_locale) | **get** /export/locale/{locale}.{ext} | Export a single locale to a language pack
[**export_template**](ExportApi.md#export_template) | **get** /export/template.{ext} | Export a template containing only source keys



## export_all

> String export_all(ext, format, filter, index, source, fallback, order, printf, charset, breaks, no_comments, no_folding, key)
Export your whole project to a multi-locale language pack

<p>Export all translations from your project to a multi-locale language pack.</p> <p>Note that not all formats support multiple locales in the same file. See <code>/api/export/archive</code> for exporting separate files, and <code>/api/export/locale</code> for exporting one language at a time.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ext** | **String** | Target file type specified as a file extension | [required] |[default to json]
**format** | Option<**String**> | More specific format of multi-locale file type. e.g. <code>rails</code> applies to <code>yml</code> |  |
**filter** | Option<**String**> | Filter assets by comma-separated tag names. Match any tag with `*` and negate tags by prefixing with `!` |  |
**index** | Option<**String**> | Override default lookup key for the file format: \"id\", \"text\" or a custom alias |  |
**source** | Option<**String**> | Specify alternative source locale instead of project default |  |
**fallback** | Option<**String**> | Fallback locale for untranslated assets, specified as short code. e.g. `en` or `en_GB` |  |
**order** | Option<**String**> | Export translations according to asset order |  |
**printf** | Option<**String**> | Force alternative \"printf\" style. <a href=\"https://localise.biz/help/developers/printf\">See string formatting</a> |  |
**charset** | Option<**String**> | Specify preferred character encoding. Alternative to `Accept-Charset` header but accepts a single value which must be valid. |  |
**breaks** | Option<**String**> | Force platform-specific line-endings. Default is Unix (LF) breaks. |  |
**no_comments** | Option<**bool**> | Disable rendering of redundant inline comments including the Loco banner. |  |
**no_folding** | Option<**bool**> | Protect <a href=\"https://localise.biz/help/developers/asset-ids#folding\">dot-separated keys</a> so that `foo.bar` is not folded into object properties. |  |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_archive

> String export_archive(ext, format, filter, index, source, namespace, fallback, order, status, path, printf, charset, breaks, no_comments, no_folding, key)
Export all locales to a zip archive

Export all translations from your project to an archive of individual locale files. You can also specify .tar instead of .zip.<br />If you're exporting to a format that supports multiple locales per file, you can use the <code>/export/all</code> method instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ext** | **String** | Target file type specified as a file extension | [required] |[default to json]
**format** | Option<**String**> | More specific format of file type. e.g. <code>symfony</code> applies to <code>php</code>, <code>xlf</code> &amp; <code>yml</code> |  |
**filter** | Option<**String**> | Filter assets by comma-separated tag names. Match any tag with `*` and negate tags by prefixing with `!` |  |
**index** | Option<**String**> | Override default lookup key for the file format: \"id\", \"text\" or a custom alias |  |
**source** | Option<**String**> | Specify alternative source locale instead of project default |  |
**namespace** | Option<**String**> | Override the project name for some language packs that use it as a key prefix |  |
**fallback** | Option<**String**> | Fallback locale for untranslated assets, specified as short code. e.g. `en` or `en_GB` |  |
**order** | Option<**String**> | Export translations according to asset order |  |
**status** | Option<**String**> | Export translations with a specific status or flag. Negate values by prefixing with `!`. e.g. \"translated\", or \"!fuzzy\". |  |
**path** | Option<**String**> | Custom pattern for file paths. <a href=\"https://localise.biz/help/developers/locales/export-paths\">See syntax</a> |  |
**printf** | Option<**String**> | Force alternative \"printf\" style. <a href=\"https://localise.biz/help/developers/printf\">See string formatting</a> |  |
**charset** | Option<**String**> | Specify preferred character encoding. Alternative to `Accept-Charset` header but accepts a single value which must be valid. |  |
**breaks** | Option<**String**> | Force platform-specific line-endings. Default is Unix (LF) breaks. |  |
**no_comments** | Option<**bool**> | Disable rendering of redundant inline comments including the Loco banner. |  |
**no_folding** | Option<**bool**> | Protect <a href=\"https://localise.biz/help/developers/asset-ids#folding\">dot-separated keys</a> so that `foo.bar` is not folded into object properties. |  |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_locale

> String export_locale(locale, ext, format, filter, index, source, namespace, fallback, order, status, printf, charset, breaks, no_comments, no_folding, key)
Export a single locale to a language pack

<p>Various export file types are supported with format variations for some types. <a href=\"https://localise.biz/api#formats\">See supported formats</a>.</p> <p>See <a href=\"https://localise.biz/help/formats/exporting#options\">available export options</a> for more detail on these parameters.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** | Locale to export, specified as short code or language tag. e.g. `en` or `en_GB` | [required] |
**ext** | **String** | Target file type specified as a file extension | [required] |[default to json]
**format** | Option<**String**> | More specific format of file type. e.g. <code>symfony</code> applies to <code>php</code>, <code>xlf</code> &amp; <code>yml</code> |  |
**filter** | Option<**String**> | Filter assets by comma-separated tag names. Match any tag with `*` and negate tags by prefixing with `!` |  |
**index** | Option<**String**> | Override default lookup key for the file format: \"id\", \"text\" or a custom alias |  |
**source** | Option<**String**> | Specify alternative source locale instead of project default |  |
**namespace** | Option<**String**> | Override the project name for some language packs that use it as a key prefix |  |
**fallback** | Option<**String**> | Fallback locale for untranslated assets, specified as short code. e.g. `en` or `en_GB` |  |
**order** | Option<**String**> | Export translations according to asset order |  |
**status** | Option<**String**> | Export translations with a specific status or flag. Negate values by prefixing with `!`. e.g. \"translated\", or \"!fuzzy\". |  |
**printf** | Option<**String**> | Force alternative \"printf\" style. <a href=\"https://localise.biz/help/developers/printf\">See string formatting</a> |  |
**charset** | Option<**String**> | Specify preferred character encoding. Alternative to `Accept-Charset` header but accepts a single value which must be valid. |  |
**breaks** | Option<**String**> | Force platform-specific line-endings. Default is Unix (LF) breaks. |  |
**no_comments** | Option<**bool**> | Disable rendering of redundant inline comments including the Loco banner. |  |
**no_folding** | Option<**bool**> | Protect <a href=\"https://localise.biz/help/developers/asset-ids#folding\">dot-separated keys</a> so that `foo.bar` is not folded into object properties. |  |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_template

> String export_template(ext, format, filter, index, source, namespace, fallback, order, status, printf, charset, breaks, no_comments, no_folding, key)
Export a template containing only source keys

<p>This is different to exporting just your source language translations, because it only exports the left hand side of each mapping.</p> <p>See <a href=\"https://localise.biz/api#formats\">supported export formats</a> and more detail on <a href=\"https://localise.biz/help/formats/exporting#options\">the available options</a>.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ext** | **String** | Target file type specified as a file extension | [required] |[default to json]
**format** | Option<**String**> | More specific format of file type. e.g. <code>symfony</code> applies to <code>php</code>, <code>xlf</code> &amp; <code>yml</code> |  |
**filter** | Option<**String**> | Filter assets by comma-separated tag names. Match any tag with `*` and negate tags by prefixing with `!` |  |
**index** | Option<**String**> | Override default lookup key for the file format: \"id\", \"text\" or a custom alias |  |
**source** | Option<**String**> | Specify alternative source locale instead of project default |  |
**namespace** | Option<**String**> | Override the project name for some language packs that use it as a key prefix |  |
**fallback** | Option<**String**> | Fallback locale for untranslated assets, specified as short code. e.g. `en` or `en_GB` |  |
**order** | Option<**String**> | Export translations according to asset order |  |
**status** | Option<**String**> | Export translations with a specific status or flag. Negate values by prefixing with `!`. e.g. \"translated\", or \"!fuzzy\". |  |
**printf** | Option<**String**> | Force alternative \"printf\" style. <a href=\"https://localise.biz/help/developers/printf\">See string formatting</a> |  |
**charset** | Option<**String**> | Specify preferred character encoding. Alternative to `Accept-Charset` header but accepts a single value which must be valid. |  |
**breaks** | Option<**String**> | Force platform-specific line-endings. Default is Unix (LF) breaks. |  |
**no_comments** | Option<**bool**> | Disable rendering of redundant inline comments including the Loco banner. |  |
**no_folding** | Option<**bool**> | Protect <a href=\"https://localise.biz/help/developers/asset-ids#folding\">dot-separated keys</a> so that `foo.bar` is not folded into object properties. |  |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

