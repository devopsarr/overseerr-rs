# \CollectionApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_collection_by_collection_id**](CollectionApi.md#get_collection_by_collection_id) | **GET** /collection/{collectionId} | Get collection details



## get_collection_by_collection_id

> models::Collection get_collection_by_collection_id(collection_id, language)
Get collection details

Returns full collection details in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **f64** |  | [required] |
**language** | Option<**String**> |  |  |

### Return type

[**models::Collection**](Collection.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

