# OneBot11-12 转换规则

为了统一数据格式的目的，本库提供了大部分的数据转换规则以兼容OneBot11协议到OneBot12。

## 1. 消息段

-   所有存在于OneBot12协议的OneBot11消息段都可以正常转换，不存在于OneBot12的消息段会被转换成[拓展消息段](https://12.onebot.dev/interface/rules/#_8)，前缀为`ob11.`。
-   未提及的**相同名称**或消息段字段保持**不变**。
-   如果某个OneBot11消息段的**字段或值**对应的OneBot12消息段**不存在**且**以下内容未提及如何转换**，则会被加上`ob11.`前缀作为拓展字段。
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

-   所有存在于OneBot12协议的OneBot11事件都可以正常转换，不存在于OneBot12的事件会被转换成[拓展事件](https://12.onebot.dev/interface/rules/#_1)，前缀为`ob11.`。
-   未提及的相同名称事件字段或相同值保持**不变**。
-   如果某个OneBot11事件的**字段或值**对应的OneBot12事件**不存在**且**以下内容未提及如何转换**，则会被加上`ob11.`前缀作为拓展字段。
-   -> / <- / <-> 代表转换方向，`->`表示OneBot11转OneBot12，`<-`表示OneBot12转OneBot11，`<->`表示双向转换，OneBot11 消息段名称在前，OneBot12 事件名称在后。

**共用字段转换规则**

| OneBot11 字段 | OneBot12 字段       | 备注                                            | 默认值       |
| ------------- | ------------------- | ----------------------------------------------- | ------------ |
| 无            | `id`                | 由OneBot实现自行决定                            | 无           |
| 无            | `self` : `platform` | 无                                              | OB12: `ob11` |
| `self_id`     | `self` : `user_id`  | 无                                              | 无           |
| `post_type`   | `type`              | `meta_event` 转换成 `meta`，其他一致            | 无           |
| 无            | `detail_type`       | 为事件的具体类型，如`meta.connect`对应`connect` | 无           |

### 2.1 元事件 转换规则(ob11 - ob12)

**共用字段转换规则**

| OneBot11 字段     | OneBot12 字段 | 备注                           | 默认值 |
| ----------------- | ------------- | ------------------------------ | ------ |
| `meta_event_type` | 无            | 以下转换规则中OneBot11事件类型 | 无     |

#### `lifecycle` -> `meta.connect`

| OneBot11 字段           | OneBot12 字段 | 备注 | 默认值                        |
| ----------------------- | ------------- | ---- | ----------------------------- |
| `sub_type == 'connect'` | 无            | 无   | 无                            |
| 无                      | `version`     | 无   | OB12: resp[`[get_version]`]() |

#### `lifecycle` -> `meta.ob11.lifecycle`

| OneBot11 字段           | OneBot12 字段 | 备注                  | 默认值 |
| ----------------------- | ------------- | --------------------- | ------ |
| `sub_type != 'connect'` | `sub_type`    | `enable` 或 `disable` | 无     |

#### `heartbeat` -> `meta.heartbeat`, `meta.status_update`

| OneBot11 字段             | OneBot12 字段 | 备注   | 默认值 |
| ------------------------- | ------------- | ------ | ------ |
| `sub_type == 'heartbeat'` | 无            | 无     | 无     |
| `status`                  | 无            | 见下方 | 无     |

如果`status`字段包含的状态信息与当前状态信息不同，OneBot实现应同时产生`meta.status_update`事件。

### 2.2 消息事件 转换规则(ob11 - ob12)

**共用字段转换规则**

| OneBot11 字段 | OneBot12 字段 | 备注 | 默认值 |
| ------------- | ------------- | ---- | ------ |
| `raw_message` | `alt_message` | 无   | 无     |

#### `message` -> `message.private`

| OneBot11 字段               | OneBot12 字段 | 备注 | 默认值 |
| --------------------------- | ------------- | ---- | ------ |
| `message_type == 'private'` | 无            | 无   | 无     |

#### `message` -> `message.group`

| OneBot11 字段             | OneBot12 字段 | 备注 | 默认值 |
| ------------------------- | ------------- | ---- | ------ |
| `message_type == 'group'` | 无            | 无   | 无     |

### 2.3 通知事件 转换规则(ob11 - ob12)

**共用字段转换规则**

| OneBot11 字段 | OneBot12 字段                  | 备注 | 默认值 |
| ------------- | ------------------------------ | ---- | ------ |
| `notice_type` | 以下转换规则中OneBot11事件类型 | 无   | 无     |

#### `group_upload` -> `message.group`

本事件较为特殊，因为OneBot12协议中文件是消息段形式的，并非事件形式，所以会被转换成消息事件

| OneBot11 字段 | OneBot12 字段 | 备注                       | 默认值             |
| ------------- | ------------- | -------------------------- | ------------------ |
| 无            | `alt_message` | 无                         | `[OneBot 11 File]` |
| `file`        | `message[0]`  | 转换成OneBot12的文件消息段 | 无                 |

文件事件`file`字段转换到文件消息段的规则

| OneBot11 字段 | OneBot12 字段 | 备注 | 默认值 |
| ------------- | ------------- | ---- | ------ |
| `id`          | `file_id`     | 无   | 无     |

#### `group_increase` -> `notice.group_member_increase`

| OneBot11 字段 | OneBot12 字段 | 备注                    | 默认值 |
| ------------- | ------------- | ----------------------- | ------ |
| `sub_type`    | `sub_type`    | `approve`会转换成`join` | 无     |

#### `group_decrease` -> `notice.group_member_decrease`

字段均保持不变。

#### `friend_add` -> `notice.friend_increase`

字段均保持不变。

#### `group_recall` -> `notice.group_message_delete`

| OneBot11 字段 | OneBot12 字段 | 备注                             | 默认值 |
| ------------- | ------------- | -------------------------------- | ------ |
| 无            | `sub_type`    | 根据`operator_id`和`user_id`决定 | 无     |

#### `friend_recall` -> `notice.private_message_delete`

字段均保持不变。
