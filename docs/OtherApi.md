# \OtherApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_keyword_by_keyword_id**](OtherApi.md#get_keyword_by_keyword_id) | **GET** /keyword/{keywordId} | Get keyword
[**list_watchproviders_movies**](OtherApi.md#list_watchproviders_movies) | **GET** /watchproviders/movies | Get watch provider movies
[**list_watchproviders_regions**](OtherApi.md#list_watchproviders_regions) | **GET** /watchproviders/regions | Get watch provider regions
[**list_watchproviders_tv**](OtherApi.md#list_watchproviders_tv) | **GET** /watchproviders/tv | Get watch provider series



## get_keyword_by_keyword_id

> models::Keyword get_keyword_by_keyword_id(keyword_id)
Get keyword

Returns a single keyword in JSON format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keyword_id** | **f64** |  | [required] |

### Return type

[**models::Keyword**](Keyword.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_watchproviders_movies

> Vec<models::WatchProviderDetails> list_watchproviders_movies(watch_region)
Get watch provider movies

Returns a list of all available watch providers for movies. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_region** | **String** |  | [required] |

### Return type

[**Vec<models::WatchProviderDetails>**](WatchProviderDetails.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_watchproviders_regions

> Vec<models::WatchProviderRegion> list_watchproviders_regions()
Get watch provider regions

Returns a list of all available watch provider regions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WatchProviderRegion>**](WatchProviderRegion.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_watchproviders_tv

> Vec<models::WatchProviderDetails> list_watchproviders_tv(watch_region)
Get watch provider series

Returns a list of all available watch providers for series. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_region** | **String** |  | [required] |

### Return type

[**Vec<models::WatchProviderDetails>**](WatchProviderDetails.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

