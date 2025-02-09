# \RequestApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_request**](RequestApi.md#create_request) | **POST** /request | Create new request
[**create_request_by_status**](RequestApi.md#create_request_by_status) | **POST** /request/{requestId}/{status} | Update a request's status
[**create_request_retry**](RequestApi.md#create_request_retry) | **POST** /request/{requestId}/retry | Retry failed request
[**delete_request**](RequestApi.md#delete_request) | **DELETE** /request/{requestId} | Delete request
[**get_request**](RequestApi.md#get_request) | **GET** /request | Get all requests
[**get_request_by_request_id**](RequestApi.md#get_request_by_request_id) | **GET** /request/{requestId} | Get MediaRequest
[**get_request_count**](RequestApi.md#get_request_count) | **GET** /request/count | Gets request counts
[**update_request**](RequestApi.md#update_request) | **PUT** /request/{requestId} | Update MediaRequest



## create_request

> models::MediaRequest create_request(create_request_request)
Create new request

Creates a new request with the provided media ID and type. The `REQUEST` permission is required.  If the user has the `ADMIN` or `AUTO_APPROVE` permissions, their request will be auomatically approved. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_request_request** | [**CreateRequestRequest**](CreateRequestRequest.md) |  | [required] |

### Return type

[**models::MediaRequest**](MediaRequest.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_request_by_status

> models::MediaRequest create_request_by_status(request_id, status)
Update a request's status

Updates a request's status to approved or declined. Also returns the request in a JSON object.  Requires the `MANAGE_REQUESTS` permission or `ADMIN`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Request ID | [required] |
**status** | **String** | New status | [required] |

### Return type

[**models::MediaRequest**](MediaRequest.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_request_retry

> models::MediaRequest create_request_retry(request_id)
Retry failed request

Retries a request by resending requests to Sonarr or Radarr.  Requires the `MANAGE_REQUESTS` permission or `ADMIN`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Request ID | [required] |

### Return type

[**models::MediaRequest**](MediaRequest.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_request

> delete_request(request_id)
Delete request

Removes a request. If the user has the `MANAGE_REQUESTS` permission, any request can be removed. Otherwise, only pending requests can be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Request ID | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_request

> models::GetUserRequests2XxResponse get_request(take, skip, filter, sort, requested_by)
Get all requests

Returns all requests if the user has the `ADMIN` or `MANAGE_REQUESTS` permissions. Otherwise, only the logged-in user's requests are returned.  If the `requestedBy` parameter is specified, only requests from that particular user ID will be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**take** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**filter** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |[default to added]
**requested_by** | Option<**f64**> |  |  |

### Return type

[**models::GetUserRequests2XxResponse**](GetUserRequests_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_request_by_request_id

> models::MediaRequest get_request_by_request_id(request_id)
Get MediaRequest

Returns a specific MediaRequest in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Request ID | [required] |

### Return type

[**models::MediaRequest**](MediaRequest.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_request_count

> models::GetRequestCount2XxResponse get_request_count()
Gets request counts

Returns the number of pending and approved requests. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetRequestCount2XxResponse**](GetRequestCount_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_request

> models::MediaRequest update_request(request_id, update_request_request)
Update MediaRequest

Updates a specific media request and returns the request in a JSON object. Requires the `MANAGE_REQUESTS` permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Request ID | [required] |
**update_request_request** | [**UpdateRequestRequest**](UpdateRequestRequest.md) |  | [required] |

### Return type

[**models::MediaRequest**](MediaRequest.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

