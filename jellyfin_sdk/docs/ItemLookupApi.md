# \ItemLookupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_search_criteria**](ItemLookupApi.md#apply_search_criteria) | **POST** /Items/RemoteSearch/Apply/{itemId} | Applies search criteria to an item and refreshes metadata.
[**get_book_remote_search_results**](ItemLookupApi.md#get_book_remote_search_results) | **POST** /Items/RemoteSearch/Book | Get book remote search.
[**get_box_set_remote_search_results**](ItemLookupApi.md#get_box_set_remote_search_results) | **POST** /Items/RemoteSearch/BoxSet | Get box set remote search.
[**get_external_id_infos**](ItemLookupApi.md#get_external_id_infos) | **GET** /Items/{itemId}/ExternalIdInfos | Get the item's external id info.
[**get_movie_remote_search_results**](ItemLookupApi.md#get_movie_remote_search_results) | **POST** /Items/RemoteSearch/Movie | Get movie remote search.
[**get_music_album_remote_search_results**](ItemLookupApi.md#get_music_album_remote_search_results) | **POST** /Items/RemoteSearch/MusicAlbum | Get music album remote search.
[**get_music_artist_remote_search_results**](ItemLookupApi.md#get_music_artist_remote_search_results) | **POST** /Items/RemoteSearch/MusicArtist | Get music artist remote search.
[**get_music_video_remote_search_results**](ItemLookupApi.md#get_music_video_remote_search_results) | **POST** /Items/RemoteSearch/MusicVideo | Get music video remote search.
[**get_person_remote_search_results**](ItemLookupApi.md#get_person_remote_search_results) | **POST** /Items/RemoteSearch/Person | Get person remote search.
[**get_series_remote_search_results**](ItemLookupApi.md#get_series_remote_search_results) | **POST** /Items/RemoteSearch/Series | Get series remote search.
[**get_trailer_remote_search_results**](ItemLookupApi.md#get_trailer_remote_search_results) | **POST** /Items/RemoteSearch/Trailer | Get trailer remote search.



## apply_search_criteria

> apply_search_criteria(item_id, unknown_base_type, replace_all_images)
Applies search criteria to an item and refreshes metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | The remote search result. | [required] |
**replace_all_images** | Option<**bool**> | Optional. Whether or not to replace all images. Default: True. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_book_remote_search_results(unknown_base_type)
Get book remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_set_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_box_set_remote_search_results(unknown_base_type)
Get box set remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_id_infos

> Vec<crate::models::ExternalIdInfo> get_external_id_infos(item_id)
Get the item's external id info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |

### Return type

[**Vec<crate::models::ExternalIdInfo>**](ExternalIdInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_movie_remote_search_results(unknown_base_type)
Get movie remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_album_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_music_album_remote_search_results(unknown_base_type)
Get music album remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_artist_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_music_artist_remote_search_results(unknown_base_type)
Get music artist remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_video_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_music_video_remote_search_results(unknown_base_type)
Get music video remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_person_remote_search_results(unknown_base_type)
Get person remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_series_remote_search_results(unknown_base_type)
Get series remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trailer_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_trailer_remote_search_results(unknown_base_type)
Get trailer remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unknown_base_type** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

