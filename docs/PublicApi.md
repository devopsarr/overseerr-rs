# \PublicApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_status**](PublicApi.md#get_status) | **GET** /status | Get Overseerr status
[**get_status_appdata**](PublicApi.md#get_status_appdata) | **GET** /status/appdata | Get application data volume status



## get_status

> models::GetStatus2XxResponse get_status()
Get Overseerr status

Returns the current Overseerr status in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetStatus2XxResponse**](GetStatus_2XX_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status_appdata

> models::GetStatusAppdata2XxResponse get_status_appdata()
Get application data volume status

For Docker installs, returns whether or not the volume mount was configured properly. Always returns true for non-Docker installs.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetStatusAppdata2XxResponse**](GetStatusAppdata_2XX_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

