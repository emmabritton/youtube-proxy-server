# Youtube Proxy Server

Server created to proxy some requests to YouTube using rotating keys. It is designed for personal use only and will not scale past a few users.

## Environment Variables

| Name | Type | Comment | Default |
| --- | --- | --- | --- |
| HOSTNAME | String | Address of server, used by dokku to forward connection | `localhost` |
| PORT | String | Port of server, used by dokku to forward connection | `3001` |
| YOUTUBE_KEYS | String | Comma separated list of YouTube API keys, these are rotated on each use | N/A |
| API_KEY | String | The API key needed to use this server | N/A |

## Endpoints

### GET /v1/admin/status

Get the key quota status

#### Response

Object of key number to remaining quota

#### Example 

```json
{
    "0": 5400,
    "1": 10000
}
```

### GET /search/:type

| Param | Type | Comment |
| --- | --- | --- |
| type | String | Must be 'channel', 'playlist' or 'video' |

| Query | Type | Comment |
| --- | --- | --- |
| q | String | Search term |

#### Response

| Field | Type | Comment |
| --- | --- | --- |
| list | Array<Video, Channel or Playlist> | List of search results |

##### Channel

| Field | Type | Comment |
| --- | --- | --- |
| id | String | YouTube ID of channel |
| title | String | Channel Title |
| thumbnail | String | URL of YouTube channel thumbnail |

##### Video

| Field | Type | Comment |
| --- | --- | --- |
| id | String | YouTube ID of video |
| channelId | String | Channel ID |
| channelTitle | String | Channel Title |
| title | String | Video name |
| thumbnail | String | URL of YouTube thumbnail |
| date | String | Video publish date |

##### Playlist

| Field | Type | Comment |
| --- | --- | --- |
| id | String | YouTube ID of playlist |
| name | String | Playlist name |
| channelId | String | Channel ID |
| channelTitle | String | Channel Title |
| thumbnail | String | URL of YouTube thumbnail |

#### Example 

```json
[
    {
        "title": "Example Video",
        "id": "grjytegdg",
        "thumbnail": "https://youtube.com/media/5hjhrtjhe",
        "channelId": "j75erhethr",
        "channelTitle": "Example Channel"
    }
]
```
###  License

```
Copyright 2020 Ray Britton

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```