# \AssetsApi

All URIs are relative to *https://localise.biz/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_asset**](AssetsApi.md#create_asset) | **post** /assets | Add a new translatable asset
[**create_plural**](AssetsApi.md#create_plural) | **post** /assets/{id}/plurals | Add a new plural form of an existing asset
[**delete_asset**](AssetsApi.md#delete_asset) | **delete** /assets/{id}.json | Delete an asset permanently
[**get_asset**](AssetsApi.md#get_asset) | **get** /assets/{id}.json | Get a project asset
[**get_asset_plurals**](AssetsApi.md#get_asset_plurals) | **get** /assets/{id}/plurals | Get plural forms of an asset
[**get_assets**](AssetsApi.md#get_assets) | **get** /assets | List all translatable assets in your project
[**patch_asset**](AssetsApi.md#patch_asset) | **patch** /assets/{id}.json | Modify a single asset
[**tag_asset**](AssetsApi.md#tag_asset) | **post** /assets/{id}/tags | Tag an asset
[**unlink_plural**](AssetsApi.md#unlink_plural) | **delete** /assets/{id}/plurals/{pid}.json | Unlinks a plural form of an existing asset
[**untag_asset**](AssetsApi.md#untag_asset) | **delete** /assets/{id}/tags/{tag}.json | Untag an asset



## create_asset

> crate::models::Asset create_asset(key, id, text, _type, context, notes, default)
Add a new translatable asset

<p>Adds a new asset to the currently authenticated project. When created successfully the response will be 201 (created).</p> <p>Specifying <code>{text}</code> also creates an initial source language translation (unless you also specify <code>default=untranslated</code>). Note that the <code>{text}</code> parameter was previously called \"name\". For legacy compatibility the old parameter will continue to work.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |
**id** | Option<**String**> | Unique asset ID up to 999 bytes (leave blank to auto-generate) |  |
**text** | Option<**String**> | Initial source language translation (required if <code>{id}</code> is empty) |  |
**_type** | Option<**String**> | Content type for all translations of the new asset |  |[default to text]
**context** | Option<**String**> | Optional context descriptor |  |
**notes** | Option<**String**> | Optional notes for translators |  |
**default** | Option<**String**> | Status of the source language translation specified in <code>{text}</code> |  |[default to translated]

### Return type

[**crate::models::Asset**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_plural

> crate::models::Asset create_plural(id, key, text, pid)
Add a new plural form of an existing asset

<p>Creates a translatable asset that's a plural form of an existing asset.</p> <p>The singular form asset specified as <code>{id}</code> must already exist, but the plural form will created as a new asset.</p> <p>You can bind an existing asset as the new plural by specifying <code>{pid}</code> in which case <code>{text}</code> will be ignored.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |
**text** | Option<**String**> | Initial source language translation of new asset (ignored when linking existing asset) |  |
**pid** | Option<**String**> | Unique asset ID for new or existing plural form (up to 999 bytes) |  |

### Return type

[**crate::models::Asset**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_asset

> crate::models::Success delete_asset(id, key)
Delete an asset permanently

<p>Deletes an asset from the currently authenticated project.</p> <p><strong>Warning</strong>: This will permanently delete all translations made of this asset across all locales</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset

> crate::models::Asset get_asset(id, key)
Get a project asset

Gets a single asset in currently authenticated project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Asset**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_plurals

> Vec<crate::models::Asset> get_asset_plurals(id, key)
Get plural forms of an asset

<p>Lists all assets that are a plural form of the current asset.</p> <p>This list does <strong>not</strong> include the singular form itself.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**Vec<crate::models::Asset>**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assets

> Vec<crate::models::Asset> get_assets(filter, key)
List all translatable assets in your project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter assets by comma-separated tag names. Match any tag with `*` and negate tags by prefixing with `!` |  |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**Vec<crate::models::Asset>**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_asset

> crate::models::Asset patch_asset(id, data, key)
Modify a single asset

<p>Modifies the properties of an asset in the currently authenticated project and returns the full, modified object.</p> <p>To modify tags, see the <a href=\"https://localise.biz/api/docs/assets#tagasset\">tagging endpoints</a>. To modify plural forms, see the <a href=\"https://localise.biz/api/docs/assets#getassetplurals\">plural endpoints</a></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**data** | [**AssetPatch**](AssetPatch.md) | Partial asset data to update | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Asset**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_asset

> crate::models::Asset tag_asset(id, name, key)
Tag an asset

<p>Tags an asset with a new or existing term. If the tag doesn't exist it will be created.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |
**name** | **String** | Name of new or existing tag | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Asset**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_plural

> crate::models::Success unlink_plural(pid, id, key)
Unlinks a plural form of an existing asset

<p>Reverts an asset from being a plural form to being a singular asset on its own.</p> <p>This action does <strong>not</strong> delete any assets.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of asset to unlink | [required] |
**id** | **String** | Asset ID | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## untag_asset

> crate::models::Success untag_asset(tag, id, key)
Untag an asset

<p>Removes a single tag from the given asset.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Term to remove from asset's tags | [required] |
**id** | **String** | Asset ID | [required] |
**key** | Option<**String**> | API key - required if not sent in Authorization header |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

