# \UsersApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_auth_reset_password**](UsersApi.md#create_auth_reset_password) | **POST** /auth/reset-password | Send a reset password email
[**create_auth_reset_password_by_guid**](UsersApi.md#create_auth_reset_password_by_guid) | **POST** /auth/reset-password/{guid} | Reset the password for a user
[**create_user**](UsersApi.md#create_user) | **POST** /user | Create new user
[**create_user_import_from_plex**](UsersApi.md#create_user_import_from_plex) | **POST** /user/import-from-plex | Import all users from Plex
[**create_user_register_push_subscription**](UsersApi.md#create_user_register_push_subscription) | **POST** /user/registerPushSubscription | Register a web push /user/registerPushSubscription
[**create_user_settings_main**](UsersApi.md#create_user_settings_main) | **POST** /user/{userId}/settings/main | Update general settings for a user
[**create_user_settings_notifications**](UsersApi.md#create_user_settings_notifications) | **POST** /user/{userId}/settings/notifications | Update notification settings for a user
[**create_user_settings_password**](UsersApi.md#create_user_settings_password) | **POST** /user/{userId}/settings/password | Update password for a user
[**create_user_settings_permissions**](UsersApi.md#create_user_settings_permissions) | **POST** /user/{userId}/settings/permissions | Update permission settings for a user
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /user/{userId} | Delete user by ID
[**get_user**](UsersApi.md#get_user) | **GET** /user | Get all users
[**get_user_by_user_id**](UsersApi.md#get_user_by_user_id) | **GET** /user/{userId} | Get user by ID
[**get_user_quota**](UsersApi.md#get_user_quota) | **GET** /user/{userId}/quota | Get quotas for a specific user
[**get_user_requests**](UsersApi.md#get_user_requests) | **GET** /user/{userId}/requests | Get requests for a specific user
[**get_user_settings_main**](UsersApi.md#get_user_settings_main) | **GET** /user/{userId}/settings/main | Get general settings for a user
[**get_user_settings_notifications**](UsersApi.md#get_user_settings_notifications) | **GET** /user/{userId}/settings/notifications | Get notification settings for a user
[**get_user_settings_password**](UsersApi.md#get_user_settings_password) | **GET** /user/{userId}/settings/password | Get password page informatiom
[**get_user_settings_permissions**](UsersApi.md#get_user_settings_permissions) | **GET** /user/{userId}/settings/permissions | Get permission settings for a user
[**get_user_watch_data**](UsersApi.md#get_user_watch_data) | **GET** /user/{userId}/watch_data | Get watch data
[**get_user_watchlist**](UsersApi.md#get_user_watchlist) | **GET** /user/{userId}/watchlist | Get the Plex watchlist for a specific user
[**put_user**](UsersApi.md#put_user) | **PUT** /user | Update batch of users
[**update_user**](UsersApi.md#update_user) | **PUT** /user/{userId} | Update a user by user ID



## create_auth_reset_password

> models::CreateAuthLogout2XxResponse create_auth_reset_password(create_auth_reset_password_request)
Send a reset password email

Sends a reset password email to the email if the user exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_auth_reset_password_request** | [**CreateAuthResetPasswordRequest**](CreateAuthResetPasswordRequest.md) |  | [required] |

### Return type

[**models::CreateAuthLogout2XxResponse**](CreateAuthLogout_2XX_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_auth_reset_password_by_guid

> models::CreateAuthLogout2XxResponse create_auth_reset_password_by_guid(guid, create_auth_reset_password_by_guid_request)
Reset the password for a user

Resets the password for a user if the given guid is connected to a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guid** | **String** |  | [required] |
**create_auth_reset_password_by_guid_request** | [**CreateAuthResetPasswordByGuidRequest**](CreateAuthResetPasswordByGuidRequest.md) |  | [required] |

### Return type

[**models::CreateAuthLogout2XxResponse**](CreateAuthLogout_2XX_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::User create_user(create_user_request)
Create new user

Creates a new user. Requires the `MANAGE_USERS` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_import_from_plex

> Vec<models::User> create_user_import_from_plex(create_user_import_from_plex_request)
Import all users from Plex

Fetches and imports users from the Plex server. If a list of Plex IDs is provided in the request body, only the specified users will be imported. Otherwise, all users will be imported.  Requires the `MANAGE_USERS` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_import_from_plex_request** | Option<[**CreateUserImportFromPlexRequest**](CreateUserImportFromPlexRequest.md)> |  |  |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_register_push_subscription

> create_user_register_push_subscription(create_user_register_push_subscription_request)
Register a web push /user/registerPushSubscription

Registers a web push subscription for the logged-in user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_register_push_subscription_request** | [**CreateUserRegisterPushSubscriptionRequest**](CreateUserRegisterPushSubscriptionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_settings_main

> models::GetUserSettingsMain2XxResponse create_user_settings_main(user_id, create_user_settings_main_request)
Update general settings for a user

Updates and returns general settings for a specific user. Requires `MANAGE_USERS` permission if editing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |
**create_user_settings_main_request** | [**CreateUserSettingsMainRequest**](CreateUserSettingsMainRequest.md) |  | [required] |

### Return type

[**models::GetUserSettingsMain2XxResponse**](GetUserSettingsMain_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_settings_notifications

> models::UserSettingsNotifications create_user_settings_notifications(user_id, user_settings_notifications)
Update notification settings for a user

Updates and returns notification settings for a specific user. Requires `MANAGE_USERS` permission if editing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |
**user_settings_notifications** | [**UserSettingsNotifications**](UserSettingsNotifications.md) |  | [required] |

### Return type

[**models::UserSettingsNotifications**](UserSettingsNotifications.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_settings_password

> create_user_settings_password(user_id, create_user_settings_password_request)
Update password for a user

Updates a user's password. Requires `MANAGE_USERS` permission if editing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |
**create_user_settings_password_request** | [**CreateUserSettingsPasswordRequest**](CreateUserSettingsPasswordRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_settings_permissions

> models::GetUserSettingsPermissions2XxResponse create_user_settings_permissions(user_id, create_user_settings_permissions_request)
Update permission settings for a user

Updates and returns permission settings for a specific user. Requires `MANAGE_USERS` permission if editing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |
**create_user_settings_permissions_request** | [**CreateUserSettingsPermissionsRequest**](CreateUserSettingsPermissionsRequest.md) |  | [required] |

### Return type

[**models::GetUserSettingsPermissions2XxResponse**](GetUserSettingsPermissions_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> models::User delete_user(user_id)
Delete user by ID

Deletes the user with the provided userId. Requires the `MANAGE_USERS` permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::GetUser2XxResponse get_user(take, skip, sort)
Get all users

Returns all users in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**take** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**sort** | Option<**String**> |  |  |[default to created]

### Return type

[**models::GetUser2XxResponse**](GetUser_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_user_id

> models::User get_user_by_user_id(user_id)
Get user by ID

Retrieves user details in a JSON object. Requires the `MANAGE_USERS` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_quota

> models::GetUserQuota2XxResponse get_user_quota(user_id)
Get quotas for a specific user

Returns quota details for a user in a JSON object. Requires `MANAGE_USERS` permission if viewing other users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::GetUserQuota2XxResponse**](GetUserQuota_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_requests

> models::GetUserRequests2XxResponse get_user_requests(user_id, take, skip)
Get requests for a specific user

Retrieves a user's requests in a JSON object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |
**take** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetUserRequests2XxResponse**](GetUserRequests_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings_main

> models::GetUserSettingsMain2XxResponse get_user_settings_main(user_id)
Get general settings for a user

Returns general settings for a specific user. Requires `MANAGE_USERS` permission if viewing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::GetUserSettingsMain2XxResponse**](GetUserSettingsMain_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings_notifications

> models::UserSettingsNotifications get_user_settings_notifications(user_id)
Get notification settings for a user

Returns notification settings for a specific user. Requires `MANAGE_USERS` permission if viewing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::UserSettingsNotifications**](UserSettingsNotifications.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings_password

> models::GetUserSettingsPassword2XxResponse get_user_settings_password(user_id)
Get password page informatiom

Returns important data for the password page to function correctly. Requires `MANAGE_USERS` permission if viewing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::GetUserSettingsPassword2XxResponse**](GetUserSettingsPassword_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings_permissions

> models::GetUserSettingsPermissions2XxResponse get_user_settings_permissions(user_id)
Get permission settings for a user

Returns permission settings for a specific user. Requires `MANAGE_USERS` permission if viewing other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::GetUserSettingsPermissions2XxResponse**](GetUserSettingsPermissions_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_watch_data

> models::GetUserWatchData2XxResponse get_user_watch_data(user_id)
Get watch data

Returns play count, play duration, and recently watched media.  Requires the `ADMIN` permission to fetch results for other users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |

### Return type

[**models::GetUserWatchData2XxResponse**](GetUserWatch_data_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_watchlist

> models::GetUserWatchlist2XxResponse get_user_watchlist(user_id, page)
Get the Plex watchlist for a specific user

Retrieves a user's Plex Watchlist in a JSON object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |
**page** | Option<**f64**> |  |  |[default to 1]

### Return type

[**models::GetUserWatchlist2XxResponse**](GetUserWatchlist_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user

> Vec<models::User> put_user(put_user_request)
Update batch of users

Update users with given IDs with provided values in request `body.settings`. You cannot update users' Plex tokens through this request.  Requires the `MANAGE_USERS` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_user_request** | [**PutUserRequest**](PutUserRequest.md) |  | [required] |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::User update_user(user_id, user)
Update a user by user ID

Update a user with the provided values. You cannot update a user's Plex token through this request.  Requires the `MANAGE_USERS` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f64** |  | [required] |
**user** | [**User**](User.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

