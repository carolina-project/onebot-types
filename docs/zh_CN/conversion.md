# OneBot11-12 转换规则

为了统一数据格式的目的，本库提供了大部分的数据转换规则以兼容OneBot11协议到OneBot12。

## 1. 消息段

所有存在于OneBot12协议的OneBot11消息段都可以正常转换，不存在于OneBot12的消息段会被转换成[拓展消息段](https://12.onebot.dev/interface/rules/#_8)，前缀为`ob11.`。

-   未提及的相同名称消息段字段保持不变。
-   如果某个OneBot11消息段的字段对应的OneBot12消息段以下内容未提及如何转换或是不存在，则会被加上`ob11.`前缀作为拓展字段。
-   -> / <- / <-> 代表转换方向，`->`表示OneBot11转OneBot12，`<-`表示OneBot12转OneBot11，`<->`表示双向转换，OneBot11 消息段名称在前，OneBot12 消息段名称在后。

### 1.1 转换规则(ob11 - ob12)

#### `text` <-> `text`

字段均保持不变。

#### `at` <-> `mention`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `qq != 'all'` | `user_id`     | 无   |

#### `at` <-> `mention_all`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `qq == 'all'` | `user_id`     | 无   |

#### `image` <-> `image`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `file`        | `file_id`     | 无   |

#### `record` <-> `voice`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `file`        | `file_id`     | 无   |

### 无 <- `audio`

OneBot11协议未提供音频消息段，将OneBot12音频消息段视为文件，并调用相关API进行上传发送。

#### `video` <-> `video`

| OneBot11 字段 | OneBot12 字段 | 备注 |
| ------------- | ------------- | ---- |
| `file`        | `file_id`     | 无   |

### 无 <- `file`

OneBot11协议未提供文件消息段，需调用文件API进行上传来发送。

#### `location` <-> `location`

| OneBot11 字段 | OneBot12 字段 | 备注 | 默认值                    |
| ------------- | ------------- | ---- | ------------------------- |
| `lat`         | `latitude`    | 无   | 无                        |
| `lon`         | `longitude`   | 无   | 无                        |
| `title`       | `title`       | 无   | OB12: `OneBot 11 Title`   |
| `content`     | `content`     | 无   | OB12: `OneBot 11 Content` |

#### `reply` <-> `reply`

| OneBot11 字段 | OneBot12 字段 | 备注               |
| ------------- | ------------- | ------------------ |
| `id`          | `message_id`  | 无                 |
| 无            | `user_id`     | OneBot实现自行决定 |

建议OneBot实现获取回复目标的发送者ID来作为`user_id`字段内容。

## 2. 事件

-   未提及的相同名称事件字段保持不变。
-   如果某个OneBot11事件的字段对应的OneBot12事件以下内容未提及如何转换或是不存在，则会被加上`ob11.`前缀作为拓展字段。
-   -> / <- / <-> 代表转换方向，`->`表示OneBot11转OneBot12，`<-`表示OneBot12转OneBot11，`<->`表示双向转换，OneBot11 消息段名称在前，OneBot12 事件名称在后。

**共用字段转换规则**

| OneBot11 字段 | OneBot12 字段       | 备注                                 | 默认值       |
| ------------- | ------------------- | ------------------------------------ | ------------ |
| 无            | `id`                | 由OneBot实现自行决定                 | 无           |
| 无            | `self` : `platform` | 无                                   | OB12: `ob11` |
| `self_id`     | `self` : `user_id`  | 无                                   | 无           |
| `post_type`   | `type`              | `meta_event` 转换成 `meta`，其他一致 | 无           |

### 2.1 元事件 转换规则(ob11 - ob12)

**共用字段转换规则**

| OneBot11 字段     | OneBot12 字段 | 备注 | 默认值 |
| ----------------- | ------------- | ---- | ------ |
| `meta_event_type` | 无            | 无   | 无     |

#### `lifecycle` -> `meta.connect`

| OneBot11 字段           | OneBot12 字段 | 备注 | 默认值                        |
| ----------------------- | ------------- | ---- | ----------------------------- |
| `sub_type == 'connect'` | 无            | 无   | 无                            |
| 无                      | `detail_type` | 无   | OB12: `connect`               |
| 无                      | `version`     | 无   | OB12: resp[`[get_version]`]() |

#### `lifecycle` -> `meta.ob11.lifecycle`

| OneBot11 字段           | OneBot12 字段 | 备注                  | 默认值                 |
| ----------------------- | ------------- | --------------------- | ---------------------- |
| `sub_type != 'connect'` | `sub_type`    | `enable` 或 `disable` | 无                     |
| 无                      | `detail_type` | 无                    | OB12: `ob11.lifecycle` |

#### `heartbeat` -> `meta.heartbeat`, `meta.status_update`

| OneBot11 字段             | OneBot12 字段 | 备注   | 默认值 |
| ------------------------- | ------------- | ------ | ------ |
| `sub_type == 'heartbeat'` | 无            | 无     | 无     |
| `status`                  | 无            | 见下方 | 无     |

如果`status`字段包含的状态信息与当前状态信息不同，OneBot实现应同时产生`meta.status_update`事件。
