# \SearchApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_discover_keyword_movies**](SearchApi.md#get_discover_keyword_movies) | **GET** /discover/keyword/{keywordId}/movies | Get movies from keyword
[**get_discover_movies**](SearchApi.md#get_discover_movies) | **GET** /discover/movies | Discover movies
[**get_discover_movies_genre_by_genre_id**](SearchApi.md#get_discover_movies_genre_by_genre_id) | **GET** /discover/movies/genre/{genreId} | Discover movies by genre
[**get_discover_movies_language_by_language**](SearchApi.md#get_discover_movies_language_by_language) | **GET** /discover/movies/language/{language} | Discover movies by original language
[**get_discover_movies_studio_by_studio_id**](SearchApi.md#get_discover_movies_studio_by_studio_id) | **GET** /discover/movies/studio/{studioId} | Discover movies by studio
[**get_discover_movies_upcoming**](SearchApi.md#get_discover_movies_upcoming) | **GET** /discover/movies/upcoming | Upcoming movies
[**get_discover_trending**](SearchApi.md#get_discover_trending) | **GET** /discover/trending | Trending movies and TV
[**get_discover_tv**](SearchApi.md#get_discover_tv) | **GET** /discover/tv | Discover TV shows
[**get_discover_tv_genre_by_genre_id**](SearchApi.md#get_discover_tv_genre_by_genre_id) | **GET** /discover/tv/genre/{genreId} | Discover TV shows by genre
[**get_discover_tv_language_by_language**](SearchApi.md#get_discover_tv_language_by_language) | **GET** /discover/tv/language/{language} | Discover TV shows by original language
[**get_discover_tv_network_by_network_id**](SearchApi.md#get_discover_tv_network_by_network_id) | **GET** /discover/tv/network/{networkId} | Discover TV shows by network
[**get_discover_tv_upcoming**](SearchApi.md#get_discover_tv_upcoming) | **GET** /discover/tv/upcoming | Discover Upcoming TV shows
[**get_discover_watchlist**](SearchApi.md#get_discover_watchlist) | **GET** /discover/watchlist | Get the Plex watchlist.
[**get_search**](SearchApi.md#get_search) | **GET** /search | Search for movies, TV shows, or people
[**get_search_company**](SearchApi.md#get_search_company) | **GET** /search/company | Search for companies
[**get_search_keyword**](SearchApi.md#get_search_keyword) | **GET** /search/keyword | Search for keywords
[**list_discover_genreslider_movie**](SearchApi.md#list_discover_genreslider_movie) | **GET** /discover/genreslider/movie | Get genre slider data for movies
[**list_discover_genreslider_tv**](SearchApi.md#list_discover_genreslider_tv) | **GET** /discover/genreslider/tv | Get genre slider data for TV series



## get_discover_keyword_movies

> models::GetDiscoverMovies2XxResponse get_discover_keyword_movies(keyword_id, page, language)
Get movies from keyword

Returns list of movies based on the provided keyword ID a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keyword_id** | **f64** |  | [required] |
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


## get_discover_movies

> models::GetDiscoverMovies2XxResponse get_discover_movies(page, language, genre, studio, keywords, sort_by, primary_release_date_gte, primary_release_date_lte, with_runtime_gte, with_runtime_lte, vote_average_gte, vote_average_lte, vote_count_gte, vote_count_lte, watch_region, watch_providers)
Discover movies

Returns a list of movies in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |
**genre** | Option<**String**> |  |  |
**studio** | Option<**f64**> |  |  |
**keywords** | Option<**String**> |  |  |
**sort_by** | Option<**String**> |  |  |
**primary_release_date_gte** | Option<**String**> |  |  |
**primary_release_date_lte** | Option<**String**> |  |  |
**with_runtime_gte** | Option<**f64**> |  |  |
**with_runtime_lte** | Option<**f64**> |  |  |
**vote_average_gte** | Option<**f64**> |  |  |
**vote_average_lte** | Option<**f64**> |  |  |
**vote_count_gte** | Option<**f64**> |  |  |
**vote_count_lte** | Option<**f64**> |  |  |
**watch_region** | Option<**String**> |  |  |
**watch_providers** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverMovies2XxResponse**](GetDiscoverMovies_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_movies_genre_by_genre_id

> models::GetDiscoverMoviesGenreByGenreId2XxResponse get_discover_movies_genre_by_genre_id(genre_id, page, language)
Discover movies by genre

Returns a list of movies based on the provided genre ID in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverMoviesGenreByGenreId2XxResponse**](GetDiscoverMoviesGenreByGenreId_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_movies_language_by_language

> models::GetDiscoverMoviesLanguageByLanguage2XxResponse get_discover_movies_language_by_language(language, page, language2)
Discover movies by original language

Returns a list of movies based on the provided ISO 639-1 language code in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language2** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverMoviesLanguageByLanguage2XxResponse**](GetDiscoverMoviesLanguageByLanguage_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_movies_studio_by_studio_id

> models::GetDiscoverMoviesStudioByStudioId2XxResponse get_discover_movies_studio_by_studio_id(studio_id, page, language)
Discover movies by studio

Returns a list of movies based on the provided studio ID in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**studio_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverMoviesStudioByStudioId2XxResponse**](GetDiscoverMoviesStudioByStudioId_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_movies_upcoming

> models::GetDiscoverMovies2XxResponse get_discover_movies_upcoming(page, language)
Upcoming movies

Returns a list of movies in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_discover_trending

> models::GetSearch2XxResponse get_discover_trending(page, language)
Trending movies and TV

Returns a list of movies and TV shows in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetSearch2XxResponse**](GetSearch_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_tv

> models::GetDiscoverTv2XxResponse get_discover_tv(page, language, genre, network, keywords, sort_by, first_air_date_gte, first_air_date_lte, with_runtime_gte, with_runtime_lte, vote_average_gte, vote_average_lte, vote_count_gte, vote_count_lte, watch_region, watch_providers)
Discover TV shows

Returns a list of TV shows in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |
**genre** | Option<**String**> |  |  |
**network** | Option<**f64**> |  |  |
**keywords** | Option<**String**> |  |  |
**sort_by** | Option<**String**> |  |  |
**first_air_date_gte** | Option<**String**> |  |  |
**first_air_date_lte** | Option<**String**> |  |  |
**with_runtime_gte** | Option<**f64**> |  |  |
**with_runtime_lte** | Option<**f64**> |  |  |
**vote_average_gte** | Option<**f64**> |  |  |
**vote_average_lte** | Option<**f64**> |  |  |
**vote_count_gte** | Option<**f64**> |  |  |
**vote_count_lte** | Option<**f64**> |  |  |
**watch_region** | Option<**String**> |  |  |
**watch_providers** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverTv2XxResponse**](GetDiscoverTv_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_tv_genre_by_genre_id

> models::GetDiscoverTvGenreByGenreId2XxResponse get_discover_tv_genre_by_genre_id(genre_id, page, language)
Discover TV shows by genre

Returns a list of TV shows based on the provided genre ID in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverTvGenreByGenreId2XxResponse**](GetDiscoverTvGenreByGenreId_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_tv_language_by_language

> models::GetDiscoverTvLanguageByLanguage2XxResponse get_discover_tv_language_by_language(language, page, language2)
Discover TV shows by original language

Returns a list of TV shows based on the provided ISO 639-1 language code in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language2** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverTvLanguageByLanguage2XxResponse**](GetDiscoverTvLanguageByLanguage_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_tv_network_by_network_id

> models::GetDiscoverTvNetworkByNetworkId2XxResponse get_discover_tv_network_by_network_id(network_id, page, language)
Discover TV shows by network

Returns a list of TV shows based on the provided network ID in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetDiscoverTvNetworkByNetworkId2XxResponse**](GetDiscoverTvNetworkByNetworkId_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_tv_upcoming

> models::GetDiscoverTv2XxResponse get_discover_tv_upcoming(page, language)
Discover Upcoming TV shows

Returns a list of upcoming TV shows in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_discover_watchlist

> models::GetUserWatchlist2XxResponse get_discover_watchlist(page)
Get the Plex watchlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |[default to 1]

### Return type

[**models::GetUserWatchlist2XxResponse**](GetUserWatchlist_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search

> models::GetSearch2XxResponse get_search(query, page, language)
Search for movies, TV shows, or people

Returns a list of movies, TV shows, or people a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]
**language** | Option<**String**> |  |  |

### Return type

[**models::GetSearch2XxResponse**](GetSearch_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_company

> models::GetSearchCompany2XxResponse get_search_company(query, page)
Search for companies

Returns a list of TMDB companies matching the search query. (Will not return origin country)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]

### Return type

[**models::GetSearchCompany2XxResponse**](GetSearchCompany_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_keyword

> models::GetSearchKeyword2XxResponse get_search_keyword(query, page)
Search for keywords

Returns a list of TMDB keywords matching the search query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]

### Return type

[**models::GetSearchKeyword2XxResponse**](GetSearchKeyword_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_discover_genreslider_movie

> Vec<models::ListDiscoverGenresliderMovie2XxResponseInner> list_discover_genreslider_movie(language)
Get genre slider data for movies

Returns a list of genres with backdrops attached

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | Option<**String**> |  |  |

### Return type

[**Vec<models::ListDiscoverGenresliderMovie2XxResponseInner>**](ListDiscoverGenresliderMovie_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_discover_genreslider_tv

> Vec<models::ListDiscoverGenresliderMovie2XxResponseInner> list_discover_genreslider_tv(language)
Get genre slider data for TV series

Returns a list of genres with backdrops attached

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | Option<**String**> |  |  |

### Return type

[**Vec<models::ListDiscoverGenresliderMovie2XxResponseInner>**](ListDiscoverGenresliderMovie_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

