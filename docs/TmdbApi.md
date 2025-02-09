# \TmdbApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_network_by_network_id**](TmdbApi.md#get_network_by_network_id) | **GET** /network/{networkId} | Get TV network details
[**get_studio_by_studio_id**](TmdbApi.md#get_studio_by_studio_id) | **GET** /studio/{studioId} | Get movie studio details
[**list_backdrops**](TmdbApi.md#list_backdrops) | **GET** /backdrops | Get backdrops of trending items
[**list_genres_movie**](TmdbApi.md#list_genres_movie) | **GET** /genres/movie | Get list of official TMDB movie genres
[**list_genres_tv**](TmdbApi.md#list_genres_tv) | **GET** /genres/tv | Get list of official TMDB movie genres
[**list_languages**](TmdbApi.md#list_languages) | **GET** /languages | Languages supported by TMDB
[**list_regions**](TmdbApi.md#list_regions) | **GET** /regions | Regions supported by TMDB



## get_network_by_network_id

> models::ProductionCompany get_network_by_network_id(network_id)
Get TV network details

Returns TV network details in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **f64** |  | [required] |

### Return type

[**models::ProductionCompany**](ProductionCompany.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_studio_by_studio_id

> models::ProductionCompany get_studio_by_studio_id(studio_id)
Get movie studio details

Returns movie studio details in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**studio_id** | **f64** |  | [required] |

### Return type

[**models::ProductionCompany**](ProductionCompany.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_backdrops

> Vec<String> list_backdrops()
Get backdrops of trending items

Returns a list of backdrop image paths in a JSON array.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_genres_movie

> Vec<models::ListGenresMovie2XxResponseInner> list_genres_movie(language)
Get list of official TMDB movie genres

Returns a list of genres in a JSON array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | Option<**String**> |  |  |

### Return type

[**Vec<models::ListGenresMovie2XxResponseInner>**](ListGenresMovie_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_genres_tv

> Vec<models::ListGenresTv2XxResponseInner> list_genres_tv(language)
Get list of official TMDB movie genres

Returns a list of genres in a JSON array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | Option<**String**> |  |  |

### Return type

[**Vec<models::ListGenresTv2XxResponseInner>**](ListGenresTv_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_languages

> Vec<models::ListLanguages2XxResponseInner> list_languages()
Languages supported by TMDB

Returns a list of languages in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ListLanguages2XxResponseInner>**](ListLanguages_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_regions

> Vec<models::ListRegions2XxResponseInner> list_regions()
Regions supported by TMDB

Returns a list of regions in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ListRegions2XxResponseInner>**](ListRegions_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

