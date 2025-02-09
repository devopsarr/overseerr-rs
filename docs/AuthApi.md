# \AuthApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_auth_local**](AuthApi.md#create_auth_local) | **POST** /auth/local | Sign in using a local account
[**create_auth_logout**](AuthApi.md#create_auth_logout) | **POST** /auth/logout | Sign out and clear session cookie
[**create_auth_plex**](AuthApi.md#create_auth_plex) | **POST** /auth/plex | Sign in using a Plex token
[**get_auth_me**](AuthApi.md#get_auth_me) | **GET** /auth/me | Get logged-in user



## create_auth_local

> models::User create_auth_local(create_auth_local_request)
Sign in using a local account

Takes an `email` and a `password` to log the user in. Generates a session cookie for use in further requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_auth_local_request** | [**CreateAuthLocalRequest**](CreateAuthLocalRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_auth_logout

> models::CreateAuthLogout2XxResponse create_auth_logout()
Sign out and clear session cookie

Completely clear the session cookie and associated values, effectively signing the user out.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CreateAuthLogout2XxResponse**](CreateAuthLogout_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_auth_plex

> models::User create_auth_plex(create_auth_plex_request)
Sign in using a Plex token

Takes an `authToken` (Plex token) to log the user in. Generates a session cookie for use in further requests. If the user does not exist, and there are no other users, then a user will be created with full admin privileges. If a user logs in with access to the main Plex server, they will also have an account created, but without any permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_auth_plex_request** | [**CreateAuthPlexRequest**](CreateAuthPlexRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_me

> models::User get_auth_me()
Get logged-in user

Returns the currently logged-in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

