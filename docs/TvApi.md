# \TvApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tv_by_tv_id**](TvApi.md#get_tv_by_tv_id) | **GET** /tv/{tvId} | Get TV details
[**get_tv_ratings**](TvApi.md#get_tv_ratings) | **GET** /tv/{tvId}/ratings | Get TV ratings
[**get_tv_recommendations**](TvApi.md#get_tv_recommendations) | **GET** /tv/{tvId}/recommendations | Get recommended TV series
[**get_tv_season_by_season_id**](TvApi.md#get_tv_season_by_season_id) | **GET** /tv/{tvId}/season/{seasonId} | Get season details and episode list
[**get_tv_similar**](TvApi.md#get_tv_similar) | **GET** /tv/{tvId}/similar | Get similar TV series



## get_tv_by_tv_id

> models::TvDetails get_tv_by_tv_id(tv_id, language)
Get TV details

Returns full TV details in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **f64** |  | [required] |
**language** | Option<**String**> |  |  |

### Return type

[**models::TvDetails**](TvDetails.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_ratings

> models::GetTvRatings2XxResponse get_tv_ratings(tv_id)
Get TV ratings

Returns ratings based on provided tvId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **f64** |  | [required] |

### Return type

[**models::GetTvRatings2XxResponse**](GetTvRatings_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_recommendations

> models::GetDiscoverTv2XxResponse get_tv_recommendations(tv_id, page, language)
Get recommended TV series

Returns list of recommended TV series based on the provided tvId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **f64** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverTv2XxResponse**](GetDiscoverTv_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_season_by_season_id

> models::Season get_tv_season_by_season_id(tv_id, season_id, language)
Get season details and episode list

Returns season details with a list of episodes in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **f64** |  | [required] |
**season_id** | **f64** |  | [required] |
**language** | Option<**String**> |  |  |

### Return type

[**models::Season**](Season.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tv_similar

> models::GetDiscoverTv2XxResponse get_tv_similar(tv_id, page, language)
Get similar TV series

Returns list of similar TV series based on the provided tvId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tv_id** | **f64** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverTv2XxResponse**](GetDiscoverTv_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

