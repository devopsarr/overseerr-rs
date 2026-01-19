# MediaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **f64** |  | [readonly]
**status** | **f64** | Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED | [readonly]
**media** | Option<[**models::MediaInfo**](MediaInfo.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**requested_by** | Option<[**models::User**](User.md)> |  | [optional]
**modified_by** | Option<[**models::MediaRequestModifiedBy**](MediaRequestModifiedBy.md)> |  | [optional]
**is4k** | Option<**bool**> |  | [optional]
**server_id** | Option<**f64**> |  | [optional]
**profile_id** | Option<**f64**> |  | [optional]
**root_folder** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


