# \IssueApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_issue**](IssueApi.md#create_issue) | **POST** /issue | Create new issue
[**create_issue_by_status**](IssueApi.md#create_issue_by_status) | **POST** /issue/{issueId}/{status} | Update an issue's status
[**create_issue_comment**](IssueApi.md#create_issue_comment) | **POST** /issue/{issueId}/comment | Create a comment
[**delete_issue**](IssueApi.md#delete_issue) | **DELETE** /issue/{issueId} | Delete issue
[**delete_issue_comment**](IssueApi.md#delete_issue_comment) | **DELETE** /issueComment/{commentId} | Delete issue comment
[**get_issue**](IssueApi.md#get_issue) | **GET** /issue | Get all issues
[**get_issue_by_issue_id**](IssueApi.md#get_issue_by_issue_id) | **GET** /issue/{issueId} | Get issue
[**get_issue_comment_by_comment_id**](IssueApi.md#get_issue_comment_by_comment_id) | **GET** /issueComment/{commentId} | Get issue comment
[**get_issue_count**](IssueApi.md#get_issue_count) | **GET** /issue/count | Gets issue counts
[**update_issue_comment**](IssueApi.md#update_issue_comment) | **PUT** /issueComment/{commentId} | Update issue comment



## create_issue

> models::Issue create_issue(create_issue_request)
Create new issue

Creates a new issue 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_issue_request** | [**CreateIssueRequest**](CreateIssueRequest.md) |  | [required] |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_issue_by_status

> models::Issue create_issue_by_status(issue_id, status)
Update an issue's status

Updates an issue's status to approved or declined. Also returns the issue in a JSON object.  Requires the `MANAGE_ISSUES` permission or `ADMIN`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | Issue ID | [required] |
**status** | **String** | New status | [required] |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_issue_comment

> models::Issue create_issue_comment(issue_id, create_issue_comment_request)
Create a comment

Creates a comment and returns associated issue in JSON format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **f64** |  | [required] |
**create_issue_comment_request** | [**CreateIssueCommentRequest**](CreateIssueCommentRequest.md) |  | [required] |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_issue

> delete_issue(issue_id)
Delete issue

Removes an issue. If the user has the `MANAGE_ISSUES` permission, any issue can be removed. Otherwise, only a users own issues can be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | Issue ID | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_issue_comment

> delete_issue_comment(comment_id)
Delete issue comment

Deletes an issue comment. Only users with `MANAGE_ISSUES` or the user who created the comment can perform this action. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | Issue Comment ID | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue

> models::GetIssue2XxResponse get_issue(take, skip, sort, filter, requested_by)
Get all issues

Returns a list of issues in JSON format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**take** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**sort** | Option<**String**> |  |  |[default to added]
**filter** | Option<**String**> |  |  |[default to open]
**requested_by** | Option<**f64**> |  |  |

### Return type

[**models::GetIssue2XxResponse**](GetIssue_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_by_issue_id

> models::Issue get_issue_by_issue_id(issue_id)
Get issue

Returns a single issue in JSON format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **f64** |  | [required] |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_comment_by_comment_id

> models::IssueComment get_issue_comment_by_comment_id(comment_id)
Get issue comment

Returns a single issue comment in JSON format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |

### Return type

[**models::IssueComment**](IssueComment.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_count

> models::GetIssueCount2XxResponse get_issue_count()
Gets issue counts

Returns the number of open and closed issues, as well as the number of issues of each type. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetIssueCount2XxResponse**](GetIssueCount_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_issue_comment

> models::IssueComment update_issue_comment(comment_id, update_issue_comment_request)
Update issue comment

Updates and returns a single issue comment in JSON format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**update_issue_comment_request** | [**UpdateIssueCommentRequest**](UpdateIssueCommentRequest.md) |  | [required] |

### Return type

[**models::IssueComment**](IssueComment.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

