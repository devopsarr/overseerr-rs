# \ServiceApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_service_radarr_by_radarr_id**](ServiceApi.md#get_service_radarr_by_radarr_id) | **GET** /service/radarr/{radarrId} | Get Radarr server quality profiles and root folders
[**get_service_sonarr_by_sonarr_id**](ServiceApi.md#get_service_sonarr_by_sonarr_id) | **GET** /service/sonarr/{sonarrId} | Get Sonarr server quality profiles and root folders
[**list_service_radarr**](ServiceApi.md#list_service_radarr) | **GET** /service/radarr | Get non-sensitive Radarr server list
[**list_service_sonarr**](ServiceApi.md#list_service_sonarr) | **GET** /service/sonarr | Get non-sensitive Sonarr server list
[**list_service_sonarr_lookup_by_tmdb_id**](ServiceApi.md#list_service_sonarr_lookup_by_tmdb_id) | **GET** /service/sonarr/lookup/{tmdbId} | Get series from Sonarr



## get_service_radarr_by_radarr_id

> models::GetServiceRadarrByRadarrId2XxResponse get_service_radarr_by_radarr_id(radarr_id)
Get Radarr server quality profiles and root folders

Returns a Radarr server's quality profile and root folder details in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**radarr_id** | **f64** |  | [required] |

### Return type

[**models::GetServiceRadarrByRadarrId2XxResponse**](GetServiceRadarrByRadarrId_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_sonarr_by_sonarr_id

> models::GetServiceSonarrBySonarrId2XxResponse get_service_sonarr_by_sonarr_id(sonarr_id)
Get Sonarr server quality profiles and root folders

Returns a Sonarr server's quality profile and root folder details in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sonarr_id** | **f64** |  | [required] |

### Return type

[**models::GetServiceSonarrBySonarrId2XxResponse**](GetServiceSonarrBySonarrId_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_radarr

> Vec<models::RadarrSettings> list_service_radarr()
Get non-sensitive Radarr server list

Returns a list of Radarr server IDs and names in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RadarrSettings>**](RadarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_sonarr

> Vec<models::SonarrSettings> list_service_sonarr()
Get non-sensitive Sonarr server list

Returns a list of Sonarr server IDs and names in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SonarrSettings>**](SonarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_sonarr_lookup_by_tmdb_id

> Vec<models::SonarrSeries> list_service_sonarr_lookup_by_tmdb_id(tmdb_id)
Get series from Sonarr

Returns a list of series returned by searching for the name in Sonarr.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | **f64** |  | [required] |

### Return type

[**Vec<models::SonarrSeries>**](SonarrSeries.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

