# \MoviesApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_movie_by_movie_id**](MoviesApi.md#get_movie_by_movie_id) | **GET** /movie/{movieId} | Get movie details
[**get_movie_ratings**](MoviesApi.md#get_movie_ratings) | **GET** /movie/{movieId}/ratings | Get movie ratings
[**get_movie_ratingscombined**](MoviesApi.md#get_movie_ratingscombined) | **GET** /movie/{movieId}/ratingscombined | Get RT and IMDB movie ratings combined
[**get_movie_recommendations**](MoviesApi.md#get_movie_recommendations) | **GET** /movie/{movieId}/recommendations | Get recommended movies
[**get_movie_similar**](MoviesApi.md#get_movie_similar) | **GET** /movie/{movieId}/similar | Get similar movies



## get_movie_by_movie_id

> models::MovieDetails get_movie_by_movie_id(movie_id, language)
Get movie details

Returns full movie details in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **f64** |  | [required] |
**language** | Option<**String**> |  |  |

### Return type

[**models::MovieDetails**](MovieDetails.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_ratings

> models::GetMovieRatings2XxResponse get_movie_ratings(movie_id)
Get movie ratings

Returns ratings based on the provided movieId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **f64** |  | [required] |

### Return type

[**models::GetMovieRatings2XxResponse**](GetMovieRatings_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_ratingscombined

> models::GetMovieRatingscombined2XxResponse get_movie_ratingscombined(movie_id)
Get RT and IMDB movie ratings combined

Returns ratings from RottenTomatoes and IMDB based on the provided movieId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **f64** |  | [required] |

### Return type

[**models::GetMovieRatingscombined2XxResponse**](GetMovieRatingscombined_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_recommendations

> models::GetDiscoverMovies2XxResponse get_movie_recommendations(movie_id, page, language)
Get recommended movies

Returns list of recommended movies based on provided movie ID in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **f64** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverMovies2XxResponse**](GetDiscoverMovies_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_similar

> models::GetDiscoverMovies2XxResponse get_movie_similar(movie_id, page, language)
Get similar movies

Returns list of similar movies based on the provided movieId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **f64** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverMovies2XxResponse**](GetDiscoverMovies_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

