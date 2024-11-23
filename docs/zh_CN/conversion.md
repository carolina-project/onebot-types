# OneBot11-12 转换规则

为了统一数据格式的目的，本库提供了大部分的数据转换规则以兼容OneBot11协议到OneBot12。

## 1. 消息段

所有存在于OneBot12协议的OneBot11消息段都可以正常转换，不存在于OneBot12的消息段会被转换成[拓展消息段](https://12.onebot.dev/interface/rules/#_8)，前缀为`ob11.`。

### 1.1 转换规则(ob11 - ob12)

-   未提及的相同名称消息段字段保持不变。
-   如果某个OneBot11消息段的字段对应的OneBot12消息段以下内容未提及如何转换或是不存在，则会被加上`ob11.`前缀作为拓展字段。
-   -> / <- / <-> 代表转换方向，`->`表示OneBot11转OneBot12，`<-`表示OneBot12转OneBot11，`<->`表示双向转换，OneBot11 消息段名称在前，OneBot12 消息段名称在后。

#### `text` <-> `text`

字段均保持不变。

#### `at` <-> `mention`

| OneBot11 字段 | OneBot12 字段 | 备注              |
| ------------- | ------------- | ----------------- |
| `qq`          | `user_id`     | `qq`的值不为`all` |

#### `at` <-> `mention_all`

| OneBot11 字段 | OneBot12 字段 | 备注            |
| ------------- | ------------- | --------------- |
| `qq`          | `user_id`     | `qq`的值为`all` |

#### `image` <-> `image`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `file`        | `file_id`     | 无   |

#### `record` <-> `voice`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `file`        | `file_id`     | 无   |

### 无 <- `audio`

将OneBot12音频消息段视为文件，并调用相关API进行上传发送。

#### `video` <-> `video`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `file`        | `file_id`     | 无   |

### 无 <- `file`

OneBot11协议未提供文件消息段，需调用文件API进行上传来发送。

#### `location` <-> `location`

| OneBot11 字段 | OneBot12 字段 | 备注                                  |
| ------------- | ------------- | ------------------------------------- |
| `lat`         | `latitude`    | 无                                    |
| `lon`         | `longitude`   | 无                                    |
| `title`       | `title`       | OneBot12侧默认值为`OneBot 11 Title`   |
| `content`     | `content`     | OneBot12侧默认值为`OneBot 11 Content` |

#### `reply` <-> `reply`

| OneBot11 字段 | OneBot12 字段 | 备注               |
| ------------- | ------------- | ------------------ |
| `id`          | `message_id`  | 无                 |
| 无            | `user_id`     | OneBot实现自行决定 |

建议OneBot实现获取回复目标的发送者ID来作为`user_id`字段内容。
## 2. 事件


### 2.1 转换规则(ob11 - ob12)

-   未提及的相同名称事件字段保持不变。
-   如果某个OneBot11事件的字段对应的OneBot12消息段以下内容未提及如何转换或是不存在，则会被加上`ob11.`前缀作为拓展字段。
-   -> / <- / <-> 代表转换方向，`->`表示OneBot11转OneBot12，`<-`表示OneBot12转OneBot11，`<->`表示双向转换，OneBot11 消息段名称在前，OneBot12 事件名称在后。


