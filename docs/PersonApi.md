# \PersonApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_person_by_person_id**](PersonApi.md#get_person_by_person_id) | **GET** /person/{personId} | Get person details
[**get_person_combined_credits**](PersonApi.md#get_person_combined_credits) | **GET** /person/{personId}/combined_credits | Get combined credits



## get_person_by_person_id

> models::PersonDetails get_person_by_person_id(person_id, language)
Get person details

Returns person details based on provided personId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **f64** |  | [required] |
**language** | Option<**String**> |  |  |

### Return type

[**models::PersonDetails**](PersonDetails.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_combined_credits

> models::GetPersonCombinedCredits2XxResponse get_person_combined_credits(person_id, language)
Get combined credits

Returns the person's combined credits based on the provided personId in a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **f64** |  | [required] |
**language** | Option<**String**> |  |  |

### Return type

[**models::GetPersonCombinedCredits2XxResponse**](GetPersonCombined_credits_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

