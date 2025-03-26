# \SettingsApi

All URIs are relative to *http://localhost:5055/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cache_flush**](SettingsApi.md#create_cache_flush) | **POST** /settings/cache/{cacheId}/flush | Flush a specific cache
[**create_discover**](SettingsApi.md#create_discover) | **POST** /settings/discover | Batch update all sliders.
[**create_discover_add**](SettingsApi.md#create_discover_add) | **POST** /settings/discover/add | Add a new slider
[**create_initialize**](SettingsApi.md#create_initialize) | **POST** /settings/initialize | Initialize application
[**create_jobs_cancel**](SettingsApi.md#create_jobs_cancel) | **POST** /settings/jobs/{jobId}/cancel | Cancel a specific job
[**create_jobs_run**](SettingsApi.md#create_jobs_run) | **POST** /settings/jobs/{jobId}/run | Invoke a specific job
[**create_jobs_schedule**](SettingsApi.md#create_jobs_schedule) | **POST** /settings/jobs/{jobId}/schedule | Modify job schedule
[**create_main**](SettingsApi.md#create_main) | **POST** /settings/main | Update main settings
[**create_main_regenerate**](SettingsApi.md#create_main_regenerate) | **POST** /settings/main/regenerate | Get main settings with newly-generated API key
[**create_notifications_discord**](SettingsApi.md#create_notifications_discord) | **POST** /settings/notifications/discord | Update Discord notification settings
[**create_notifications_email**](SettingsApi.md#create_notifications_email) | **POST** /settings/notifications/email | Update email notification settings
[**create_notifications_gotify**](SettingsApi.md#create_notifications_gotify) | **POST** /settings/notifications/gotify | Update Gotify notification settings
[**create_notifications_lunasea**](SettingsApi.md#create_notifications_lunasea) | **POST** /settings/notifications/lunasea | Update LunaSea notification settings
[**create_notifications_pushbullet**](SettingsApi.md#create_notifications_pushbullet) | **POST** /settings/notifications/pushbullet | Update Pushbullet notification settings
[**create_notifications_pushover**](SettingsApi.md#create_notifications_pushover) | **POST** /settings/notifications/pushover | Update Pushover notification settings
[**create_notifications_slack**](SettingsApi.md#create_notifications_slack) | **POST** /settings/notifications/slack | Update Slack notification settings
[**create_notifications_telegram**](SettingsApi.md#create_notifications_telegram) | **POST** /settings/notifications/telegram | Update Telegram notification settings
[**create_notifications_webhook**](SettingsApi.md#create_notifications_webhook) | **POST** /settings/notifications/webhook | Update webhook notification settings
[**create_notifications_webpush**](SettingsApi.md#create_notifications_webpush) | **POST** /settings/notifications/webpush | Update Web Push notification settings
[**create_plex**](SettingsApi.md#create_plex) | **POST** /settings/plex | Update Plex settings
[**create_plex_sync**](SettingsApi.md#create_plex_sync) | **POST** /settings/plex/sync | Start full Plex library scan
[**create_radarr**](SettingsApi.md#create_radarr) | **POST** /settings/radarr | Create Radarr instance
[**create_sonarr**](SettingsApi.md#create_sonarr) | **POST** /settings/sonarr | Create Sonarr instance
[**create_tautulli**](SettingsApi.md#create_tautulli) | **POST** /settings/tautulli | Update Tautulli settings
[**delete_discover**](SettingsApi.md#delete_discover) | **DELETE** /settings/discover/{sliderId} | Delete slider by ID
[**delete_radarr**](SettingsApi.md#delete_radarr) | **DELETE** /settings/radarr/{radarrId} | Delete Radarr instance
[**delete_sonarr**](SettingsApi.md#delete_sonarr) | **DELETE** /settings/sonarr/{sonarrId} | Delete Sonarr instance
[**get_about**](SettingsApi.md#get_about) | **GET** /settings/about | Get server stats
[**get_cache**](SettingsApi.md#get_cache) | **GET** /settings/cache | Get a list of active caches
[**get_discover_reset**](SettingsApi.md#get_discover_reset) | **GET** /settings/discover/reset | Reset all discover sliders
[**get_main**](SettingsApi.md#get_main) | **GET** /settings/main | Get main settings
[**get_notifications_discord**](SettingsApi.md#get_notifications_discord) | **GET** /settings/notifications/discord | Get Discord notification settings
[**get_notifications_email**](SettingsApi.md#get_notifications_email) | **GET** /settings/notifications/email | Get email notification settings
[**get_notifications_gotify**](SettingsApi.md#get_notifications_gotify) | **GET** /settings/notifications/gotify | Get Gotify notification settings
[**get_notifications_lunasea**](SettingsApi.md#get_notifications_lunasea) | **GET** /settings/notifications/lunasea | Get LunaSea notification settings
[**get_notifications_pushbullet**](SettingsApi.md#get_notifications_pushbullet) | **GET** /settings/notifications/pushbullet | Get Pushbullet notification settings
[**get_notifications_pushover**](SettingsApi.md#get_notifications_pushover) | **GET** /settings/notifications/pushover | Get Pushover notification settings
[**get_notifications_slack**](SettingsApi.md#get_notifications_slack) | **GET** /settings/notifications/slack | Get Slack notification settings
[**get_notifications_telegram**](SettingsApi.md#get_notifications_telegram) | **GET** /settings/notifications/telegram | Get Telegram notification settings
[**get_notifications_webhook**](SettingsApi.md#get_notifications_webhook) | **GET** /settings/notifications/webhook | Get webhook notification settings
[**get_notifications_webpush**](SettingsApi.md#get_notifications_webpush) | **GET** /settings/notifications/webpush | Get Web Push notification settings
[**get_plex**](SettingsApi.md#get_plex) | **GET** /settings/plex | Get Plex settings
[**get_plex_sync**](SettingsApi.md#get_plex_sync) | **GET** /settings/plex/sync | Get status of full Plex library scan
[**get_public**](SettingsApi.md#get_public) | **GET** /settings/public | Get public settings
[**get_tautulli**](SettingsApi.md#get_tautulli) | **GET** /settings/tautulli | Get Tautulli settings
[**list_discover**](SettingsApi.md#list_discover) | **GET** /settings/discover | Get all discover sliders
[**list_jobs**](SettingsApi.md#list_jobs) | **GET** /settings/jobs | Get scheduled jobs
[**list_logs**](SettingsApi.md#list_logs) | **GET** /settings/logs | Returns logs
[**list_notifications_pushover_sounds**](SettingsApi.md#list_notifications_pushover_sounds) | **GET** /settings/notifications/pushover/sounds | Get Pushover sounds
[**list_plex_devices_servers**](SettingsApi.md#list_plex_devices_servers) | **GET** /settings/plex/devices/servers | Gets the user's available Plex servers
[**list_plex_library**](SettingsApi.md#list_plex_library) | **GET** /settings/plex/library | Get Plex libraries
[**list_plex_users**](SettingsApi.md#list_plex_users) | **GET** /settings/plex/users | Get Plex users
[**list_radarr**](SettingsApi.md#list_radarr) | **GET** /settings/radarr | Get Radarr settings
[**list_radarr_profiles**](SettingsApi.md#list_radarr_profiles) | **GET** /settings/radarr/{radarrId}/profiles | Get available Radarr profiles
[**list_sonarr**](SettingsApi.md#list_sonarr) | **GET** /settings/sonarr | Get Sonarr settings
[**test_notifications_discord**](SettingsApi.md#test_notifications_discord) | **POST** /settings/notifications/discord/test | Test Discord settings
[**test_notifications_email**](SettingsApi.md#test_notifications_email) | **POST** /settings/notifications/email/test | Test email settings
[**test_notifications_gotify**](SettingsApi.md#test_notifications_gotify) | **POST** /settings/notifications/gotify/test | Test Gotify settings
[**test_notifications_lunasea**](SettingsApi.md#test_notifications_lunasea) | **POST** /settings/notifications/lunasea/test | Test LunaSea settings
[**test_notifications_pushbullet**](SettingsApi.md#test_notifications_pushbullet) | **POST** /settings/notifications/pushbullet/test | Test Pushbullet settings
[**test_notifications_pushover**](SettingsApi.md#test_notifications_pushover) | **POST** /settings/notifications/pushover/test | Test Pushover settings
[**test_notifications_slack**](SettingsApi.md#test_notifications_slack) | **POST** /settings/notifications/slack/test | Test Slack settings
[**test_notifications_telegram**](SettingsApi.md#test_notifications_telegram) | **POST** /settings/notifications/telegram/test | Test Telegram settings
[**test_notifications_webhook**](SettingsApi.md#test_notifications_webhook) | **POST** /settings/notifications/webhook/test | Test webhook settings
[**test_notifications_webpush**](SettingsApi.md#test_notifications_webpush) | **POST** /settings/notifications/webpush/test | Test Web Push settings
[**test_radarr**](SettingsApi.md#test_radarr) | **POST** /settings/radarr/test | Test Radarr configuration
[**test_sonarr**](SettingsApi.md#test_sonarr) | **POST** /settings/sonarr/test | Test Sonarr configuration
[**update_discover**](SettingsApi.md#update_discover) | **PUT** /settings/discover/{sliderId} | Update a single slider
[**update_radarr**](SettingsApi.md#update_radarr) | **PUT** /settings/radarr/{radarrId} | Update Radarr instance
[**update_sonarr**](SettingsApi.md#update_sonarr) | **PUT** /settings/sonarr/{sonarrId} | Update Sonarr instance



## create_cache_flush

> create_cache_flush(cache_id)
Flush a specific cache

Flushes all data from the cache ID provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cache_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_discover

> Vec<models::DiscoverSlider> create_discover(discover_slider)
Batch update all sliders.

Batch update all sliders at once. Should also be used for creation. Will only update sliders provided and will not delete any sliders not present in the request. If a slider is missing a required field, it will be ignored. Requires the `ADMIN` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discover_slider** | [**Vec<models::DiscoverSlider>**](DiscoverSlider.md) |  | [required] |

### Return type

[**Vec<models::DiscoverSlider>**](DiscoverSlider.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_discover_add

> models::DiscoverSlider create_discover_add(create_discover_add_request)
Add a new slider

Add a single slider and return the newly created slider. Requires the `ADMIN` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_discover_add_request** | [**CreateDiscoverAddRequest**](CreateDiscoverAddRequest.md) |  | [required] |

### Return type

[**models::DiscoverSlider**](DiscoverSlider.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_initialize

> models::PublicSettings create_initialize()
Initialize application

Sets the app as initialized, allowing the user to navigate to pages other than the setup page.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicSettings**](PublicSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_jobs_cancel

> models::Job create_jobs_cancel(job_id)
Cancel a specific job

Cancels a specific job. Will return the new job status in JSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_jobs_run

> models::Job create_jobs_run(job_id)
Invoke a specific job

Invokes a specific job to run. Will return the new job status in JSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_jobs_schedule

> models::Job create_jobs_schedule(job_id, create_jobs_schedule_request)
Modify job schedule

Re-registers the job with the schedule specified. Will return the job in JSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**create_jobs_schedule_request** | [**CreateJobsScheduleRequest**](CreateJobsScheduleRequest.md) |  | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_main

> models::MainSettings create_main(main_settings)
Update main settings

Updates main settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**main_settings** | [**MainSettings**](MainSettings.md) |  | [required] |

### Return type

[**models::MainSettings**](MainSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_main_regenerate

> models::MainSettings create_main_regenerate()
Get main settings with newly-generated API key

Returns main settings in a JSON object, using the new API key.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MainSettings**](MainSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_discord

> models::DiscordSettings create_notifications_discord(discord_settings)
Update Discord notification settings

Updates Discord notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discord_settings** | [**DiscordSettings**](DiscordSettings.md) |  | [required] |

### Return type

[**models::DiscordSettings**](DiscordSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_email

> models::NotificationEmailSettings create_notifications_email(notification_email_settings)
Update email notification settings

Updates email notification settings with provided values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_email_settings** | [**NotificationEmailSettings**](NotificationEmailSettings.md) |  | [required] |

### Return type

[**models::NotificationEmailSettings**](NotificationEmailSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_gotify

> models::GotifySettings create_notifications_gotify(gotify_settings)
Update Gotify notification settings

Update Gotify notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gotify_settings** | [**GotifySettings**](GotifySettings.md) |  | [required] |

### Return type

[**models::GotifySettings**](GotifySettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_lunasea

> models::LunaSeaSettings create_notifications_lunasea(luna_sea_settings)
Update LunaSea notification settings

Updates LunaSea notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**luna_sea_settings** | [**LunaSeaSettings**](LunaSeaSettings.md) |  | [required] |

### Return type

[**models::LunaSeaSettings**](LunaSeaSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_pushbullet

> models::PushbulletSettings create_notifications_pushbullet(pushbullet_settings)
Update Pushbullet notification settings

Update Pushbullet notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pushbullet_settings** | [**PushbulletSettings**](PushbulletSettings.md) |  | [required] |

### Return type

[**models::PushbulletSettings**](PushbulletSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_pushover

> models::PushoverSettings create_notifications_pushover(pushover_settings)
Update Pushover notification settings

Update Pushover notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pushover_settings** | [**PushoverSettings**](PushoverSettings.md) |  | [required] |

### Return type

[**models::PushoverSettings**](PushoverSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_slack

> models::SlackSettings create_notifications_slack(slack_settings)
Update Slack notification settings

Updates Slack notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slack_settings** | [**SlackSettings**](SlackSettings.md) |  | [required] |

### Return type

[**models::SlackSettings**](SlackSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_telegram

> models::TelegramSettings create_notifications_telegram(telegram_settings)
Update Telegram notification settings

Update Telegram notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**telegram_settings** | [**TelegramSettings**](TelegramSettings.md) |  | [required] |

### Return type

[**models::TelegramSettings**](TelegramSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_webhook

> models::WebhookSettings create_notifications_webhook(webhook_settings)
Update webhook notification settings

Updates webhook notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_settings** | [**WebhookSettings**](WebhookSettings.md) |  | [required] |

### Return type

[**models::WebhookSettings**](WebhookSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notifications_webpush

> models::WebPushSettings create_notifications_webpush(web_push_settings)
Update Web Push notification settings

Updates Web Push notification settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_push_settings** | [**WebPushSettings**](WebPushSettings.md) |  | [required] |

### Return type

[**models::WebPushSettings**](WebPushSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_plex

> models::PlexSettings create_plex(plex_settings)
Update Plex settings

Updates Plex settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plex_settings** | [**PlexSettings**](PlexSettings.md) |  | [required] |

### Return type

[**models::PlexSettings**](PlexSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_plex_sync

> models::GetPlexSync2XxResponse create_plex_sync(create_plex_sync_request)
Start full Plex library scan

Runs a full Plex library scan and returns the progress in a JSON array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_plex_sync_request** | Option<[**CreatePlexSyncRequest**](CreatePlexSyncRequest.md)> |  |  |

### Return type

[**models::GetPlexSync2XxResponse**](GetPlexSync_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_radarr

> models::RadarrSettings create_radarr(radarr_settings)
Create Radarr instance

Creates a new Radarr instance from the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**radarr_settings** | [**RadarrSettings**](RadarrSettings.md) |  | [required] |

### Return type

[**models::RadarrSettings**](RadarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sonarr

> models::SonarrSettings create_sonarr(sonarr_settings)
Create Sonarr instance

Creates a new Sonarr instance from the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sonarr_settings** | [**SonarrSettings**](SonarrSettings.md) |  | [required] |

### Return type

[**models::SonarrSettings**](SonarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tautulli

> models::TautulliSettings create_tautulli(tautulli_settings)
Update Tautulli settings

Updates Tautulli settings with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tautulli_settings** | [**TautulliSettings**](TautulliSettings.md) |  | [required] |

### Return type

[**models::TautulliSettings**](TautulliSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_discover

> models::DiscoverSlider delete_discover(slider_id)
Delete slider by ID

Deletes the slider with the provided sliderId. Requires the `ADMIN` permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slider_id** | **f64** |  | [required] |

### Return type

[**models::DiscoverSlider**](DiscoverSlider.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_radarr

> models::RadarrSettings delete_radarr(radarr_id)
Delete Radarr instance

Deletes an existing Radarr instance based on the radarrId parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**radarr_id** | **i32** | Radarr instance ID | [required] |

### Return type

[**models::RadarrSettings**](RadarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sonarr

> models::SonarrSettings delete_sonarr(sonarr_id)
Delete Sonarr instance

Deletes an existing Sonarr instance based on the sonarrId parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sonarr_id** | **i32** | Sonarr instance ID | [required] |

### Return type

[**models::SonarrSettings**](SonarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_about

> models::GetAbout2XxResponse get_about()
Get server stats

Returns current server stats in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAbout2XxResponse**](GetAbout_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cache

> models::GetCache2XxResponse get_cache()
Get a list of active caches

Retrieves a list of all active caches and their current stats.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetCache2XxResponse**](GetCache_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discover_reset

> get_discover_reset()
Reset all discover sliders

Resets all discovery sliders to the default values. Requires the `ADMIN` permission.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_main

> models::MainSettings get_main()
Get main settings

Retrieves all main settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MainSettings**](MainSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_discord

> models::DiscordSettings get_notifications_discord()
Get Discord notification settings

Returns current Discord notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DiscordSettings**](DiscordSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_email

> models::NotificationEmailSettings get_notifications_email()
Get email notification settings

Returns current email notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NotificationEmailSettings**](NotificationEmailSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_gotify

> models::GotifySettings get_notifications_gotify()
Get Gotify notification settings

Returns current Gotify notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GotifySettings**](GotifySettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_lunasea

> models::LunaSeaSettings get_notifications_lunasea()
Get LunaSea notification settings

Returns current LunaSea notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LunaSeaSettings**](LunaSeaSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_pushbullet

> models::PushbulletSettings get_notifications_pushbullet()
Get Pushbullet notification settings

Returns current Pushbullet notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PushbulletSettings**](PushbulletSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_pushover

> models::PushoverSettings get_notifications_pushover()
Get Pushover notification settings

Returns current Pushover notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PushoverSettings**](PushoverSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_slack

> models::SlackSettings get_notifications_slack()
Get Slack notification settings

Returns current Slack notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SlackSettings**](SlackSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_telegram

> models::TelegramSettings get_notifications_telegram()
Get Telegram notification settings

Returns current Telegram notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TelegramSettings**](TelegramSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_webhook

> models::WebhookSettings get_notifications_webhook()
Get webhook notification settings

Returns current webhook notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhookSettings**](WebhookSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_webpush

> models::WebPushSettings get_notifications_webpush()
Get Web Push notification settings

Returns current Web Push notification settings in a JSON object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebPushSettings**](WebPushSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plex

> models::PlexSettings get_plex()
Get Plex settings

Retrieves current Plex settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PlexSettings**](PlexSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plex_sync

> models::GetPlexSync2XxResponse get_plex_sync()
Get status of full Plex library scan

Returns scan progress in a JSON array.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPlexSync2XxResponse**](GetPlexSync_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public

> models::PublicSettings get_public()
Get public settings

Returns settings that are not protected or sensitive. Mainly used to determine if the application has been configured for the first time.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicSettings**](PublicSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tautulli

> models::TautulliSettings get_tautulli()
Get Tautulli settings

Retrieves current Tautulli settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TautulliSettings**](TautulliSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_discover

> Vec<models::DiscoverSlider> list_discover()
Get all discover sliders

Returns all discovery sliders. Built-in and custom made.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DiscoverSlider>**](DiscoverSlider.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jobs

> Vec<models::Job> list_jobs()
Get scheduled jobs

Returns list of all scheduled jobs and details about their next execution time in a JSON array.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Job>**](Job.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_logs

> Vec<models::ListLogs2XxResponseInner> list_logs(take, skip, filter, search)
Returns logs

Returns list of all log items and details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**take** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**filter** | Option<**String**> |  |  |[default to debug]
**search** | Option<**String**> |  |  |

### Return type

[**Vec<models::ListLogs2XxResponseInner>**](ListLogs_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_notifications_pushover_sounds

> Vec<models::ListNotificationsPushoverSounds2XxResponseInner> list_notifications_pushover_sounds(token)
Get Pushover sounds

Returns valid Pushover sound options in a JSON array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**Vec<models::ListNotificationsPushoverSounds2XxResponseInner>**](ListNotificationsPushoverSounds_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_plex_devices_servers

> Vec<models::PlexDevice> list_plex_devices_servers()
Gets the user's available Plex servers

Returns a list of available Plex servers and their connectivity state

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PlexDevice>**](PlexDevice.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_plex_library

> Vec<models::PlexLibrary> list_plex_library(sync, enable)
Get Plex libraries

Returns a list of Plex libraries in a JSON array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync** | Option<**String**> | Syncs the current libraries with the current Plex server |  |
**enable** | Option<**String**> | Comma separated list of libraries to enable. Any libraries not passed will be disabled! |  |

### Return type

[**Vec<models::PlexLibrary>**](PlexLibrary.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_plex_users

> Vec<models::ListPlexUsers2XxResponseInner> list_plex_users()
Get Plex users

Returns a list of Plex users in a JSON array.  Requires the `MANAGE_USERS` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ListPlexUsers2XxResponseInner>**](ListPlexUsers_2XX_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_radarr

> Vec<models::RadarrSettings> list_radarr()
Get Radarr settings

Returns all Radarr settings in a JSON array.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RadarrSettings>**](RadarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_radarr_profiles

> Vec<models::ServiceProfile> list_radarr_profiles(radarr_id)
Get available Radarr profiles

Returns a list of profiles available on the Radarr server instance in a JSON array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**radarr_id** | **i32** | Radarr instance ID | [required] |

### Return type

[**Vec<models::ServiceProfile>**](ServiceProfile.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sonarr

> Vec<models::SonarrSettings> list_sonarr()
Get Sonarr settings

Returns all Sonarr settings in a JSON array.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SonarrSettings>**](SonarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_discord

> test_notifications_discord(discord_settings)
Test Discord settings

Sends a test notification to the Discord agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discord_settings** | [**DiscordSettings**](DiscordSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_email

> test_notifications_email(notification_email_settings)
Test email settings

Sends a test notification to the email agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_email_settings** | [**NotificationEmailSettings**](NotificationEmailSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_gotify

> test_notifications_gotify(gotify_settings)
Test Gotify settings

Sends a test notification to the Gotify agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gotify_settings** | [**GotifySettings**](GotifySettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_lunasea

> test_notifications_lunasea(luna_sea_settings)
Test LunaSea settings

Sends a test notification to the LunaSea agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**luna_sea_settings** | [**LunaSeaSettings**](LunaSeaSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_pushbullet

> test_notifications_pushbullet(pushbullet_settings)
Test Pushbullet settings

Sends a test notification to the Pushbullet agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pushbullet_settings** | [**PushbulletSettings**](PushbulletSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_pushover

> test_notifications_pushover(pushover_settings)
Test Pushover settings

Sends a test notification to the Pushover agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pushover_settings** | [**PushoverSettings**](PushoverSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_slack

> test_notifications_slack(slack_settings)
Test Slack settings

Sends a test notification to the Slack agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slack_settings** | [**SlackSettings**](SlackSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_telegram

> test_notifications_telegram(telegram_settings)
Test Telegram settings

Sends a test notification to the Telegram agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**telegram_settings** | [**TelegramSettings**](TelegramSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_webhook

> test_notifications_webhook(webhook_settings)
Test webhook settings

Sends a test notification to the webhook agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_settings** | [**WebhookSettings**](WebhookSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notifications_webpush

> test_notifications_webpush(web_push_settings)
Test Web Push settings

Sends a test notification to the Web Push agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_push_settings** | [**WebPushSettings**](WebPushSettings.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_radarr

> models::TestRadarr2XxResponse test_radarr(test_radarr_request)
Test Radarr configuration

Tests if the Radarr configuration is valid. Returns profiles and root folders on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_radarr_request** | [**TestRadarrRequest**](TestRadarrRequest.md) |  | [required] |

### Return type

[**models::TestRadarr2XxResponse**](TestRadarr_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_sonarr

> models::TestRadarr2XxResponse test_sonarr(test_sonarr_request)
Test Sonarr configuration

Tests if the Sonarr configuration is valid. Returns profiles and root folders on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_sonarr_request** | [**TestSonarrRequest**](TestSonarrRequest.md) |  | [required] |

### Return type

[**models::TestRadarr2XxResponse**](TestRadarr_2XX_response.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_discover

> models::DiscoverSlider update_discover(slider_id, update_discover_request)
Update a single slider

Updates a single slider and return the newly updated slider. Requires the `ADMIN` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slider_id** | **f64** |  | [required] |
**update_discover_request** | [**UpdateDiscoverRequest**](UpdateDiscoverRequest.md) |  | [required] |

### Return type

[**models::DiscoverSlider**](DiscoverSlider.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_radarr

> models::RadarrSettings update_radarr(radarr_id, radarr_settings)
Update Radarr instance

Updates an existing Radarr instance with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**radarr_id** | **i32** | Radarr instance ID | [required] |
**radarr_settings** | [**RadarrSettings**](RadarrSettings.md) |  | [required] |

### Return type

[**models::RadarrSettings**](RadarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sonarr

> models::SonarrSettings update_sonarr(sonarr_id, sonarr_settings)
Update Sonarr instance

Updates an existing Sonarr instance with the provided values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sonarr_id** | **i32** | Sonarr instance ID | [required] |
**sonarr_settings** | [**SonarrSettings**](SonarrSettings.md) |  | [required] |

### Return type

[**models::SonarrSettings**](SonarrSettings.md)

### Authorization

[apiKey](../README.md#apiKey), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

