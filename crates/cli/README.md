```
 _______ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|
```

```bash
❯ .\seam.exe -l douyu -i 88080
[  {
    "rate": "超清1",
    "url": "http://url1"
  },
  {
    "rate": "超清2",
    "url": "http://url2"
  }
]
```

-   `-l` 代表平台, 目前支持的平台见下表
-   `-i` 代表直播间号, 也就是直播间链接中的 `rid`

> 因为数据具有时效性, 所以具体链接使用 `url` 进行替换

**注意事项: 目前抖音和快手因为 cookie 模块的加入进行了较大修改, 所以目前不支持获取这两个平台的直播源**
