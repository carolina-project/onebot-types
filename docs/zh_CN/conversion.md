# OneBot11-12 转换规则

为了统一数据格式的目的，本库提供了大部分的数据转换规则以兼容OneBot11协议到OneBot12。

下文中的实现均指代实现协议转换的应用。

## 1. 消息段

-   不存在于OneBot12的消息段会被转换成[拓展消息段](https://12.onebot.dev/interface/rules/#_8)，前缀为`ob11.`。
-   未提及的**相同名称**消息段/消息段字段保持**不变**。
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

| OneBot11 字段 | OneBot12 字段 | 备注                        |
| ------------- | ------------- | --------------------------- |
| `file`        | 无            | 实现根据该字段决定`file_id` |
| 无            | `file_id`     | 无                          |

OB11消息段的文件`url`字段不存在于OB12消息段中，也不会成为拓展字段，因为OB12协议存在获取文件信息和上传文件的动作，文件的url等信息在实现内部保存，需要实现自行缓存文件信息来提供转换操作

而文件类(`image`, `record`等)消息段的特有字段(比如`image`的`type`，`record`的`image`等)会根据情况来进行转换，如果OB12消息段的拓展字段存在对应的特有字段，那么就用OB12消息段的对应字段值；反之就用实现所缓存的字段值或默认值

#### `record` <-> `voice`

| OneBot11 字段 | OneBot12 字段 | 备注                        |
| ------------- | ------------- | --------------------------- |
| `file`        | 无            | 实现根据该字段决定`file_id` |
| 无            | `file_id`     | 无                          |

与`image`的转换规则相同。

### `record` <- `audio`

| OneBot11 字段 | OneBot12 字段 | 备注                        |
| ------------- | ------------- | --------------------------- |
| `file`        | 无            | 实现根据该字段决定`file_id` |
| 无            | `file_id`     | 无                          |

与`image`的转换规则相同。

#### `video` <-> `video`

| OneBot11 字段 | OneBot12 字段 | 备注                        |
| ------------- | ------------- | --------------------------- |
| `file`        | 无            | 实现根据该字段决定`file_id` |
| 无            | `file_id`     | 无                          |

与`image`的转换规则相同。

#### `location` <-> `location`

| OneBot11 字段 | OneBot12 字段 | 备注 | 默认值                    |
| ------------- | ------------- | ---- | ------------------------- |
| `lat`         | `latitude`    | 无   | 无                        |
| `lon`         | `longitude`   | 无   | 无                        |
| `title`       | `title`       | 无   | OB12: `OneBot 11 Title`   |
| `content`     | `content`     | 无   | OB12: `OneBot 11 Content` |

#### `reply` <-> `reply`

| OneBot11 字段 | OneBot12 字段 | 备注         |
| ------------- | ------------- | ------------ |
| `id`          | `message_id`  | 无           |
| 无            | `user_id`     | 实现自行决定 |

建议实现获取回复目标的发送者ID来作为`user_id`字段内容。

## 2. 事件

-   不存在于OneBot12的事件会被转换成[拓展事件](https://12.onebot.dev/interface/rules/#_1)，前缀为`ob11.`。
-   未提及的相同名称事件/事件字段/字段值保持**不变**。
-   如果某个OneBot11事件的**字段或值**对应的OneBot12事件**不存在**且**以下内容未提及如何转换**，则会被加上`ob11.`前缀作为拓展字段。
-   -> / <- / <-> 代表转换方向，`->`表示OneBot11转OneBot12，`<-`表示OneBot12转OneBot11，`<->`表示双向转换，OneBot11 消息段名称在前，OneBot12 事件名称在后。

**共用字段转换规则**

| OneBot11 字段 | OneBot12 字段       | 备注                                            | 默认值       |
| ------------- | ------------------- | ----------------------------------------------- | ------------ |
| 无            | `id`                | 由实现自行决定                                  | 无           |
| 无            | `self` : `platform` | 无                                              | OB12: `ob11` |
| `self_id`     | `self` : `user_id`  | 无                                              | 无           |
| `post_type`   | `type`              | `meta_event` 转换成 `meta`，其他一致            | 无           |
| 无            | `detail_type`       | 为事件的具体类型，如`meta.connect`对应`connect` | 无           |

### 2.1 元事件 转换规则(ob11 - ob12)

**共用字段转换规则**

| OneBot11 字段     | OneBot12 字段 | 备注                             | 默认值 |
| ----------------- | ------------- | -------------------------------- | ------ |
| `meta_event_type` | 无            | 以下转换规则中的OneBot11事件类型 | 无     |

#### `lifecycle` -> `meta.connect`

| OneBot11 字段           | OneBot12 字段 | 备注 | 默认值                                                  |
| ----------------------- | ------------- | ---- | ------------------------------------------------------- |
| `sub_type == 'connect'` | 无            | 无   | 无                                                      |
| 无                      | `version`     | 无   | OB12: resp[`[get_version]`](#cleancache-ob11cleancache) |

#### `lifecycle` -> `meta.ob11.lifecycle`

| OneBot11 字段           | OneBot12 字段 | 备注                  | 默认值 |
| ----------------------- | ------------- | --------------------- | ------ |
| `sub_type != 'connect'` | `sub_type`    | `enable` 或 `disable` | 无     |

#### `heartbeat` -> `meta.heartbeat`, `meta.status_update`

| OneBot11 字段             | OneBot12 字段 | 备注   | 默认值 |
| ------------------------- | ------------- | ------ | ------ |
| `sub_type == 'heartbeat'` | 无            | 无     | 无     |
| `status`                  | 无            | 见下方 | 无     |

如果`status`字段包含的状态信息与当前状态信息不同，实现应同时产生`meta.status_update`事件。

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

| OneBot11 字段 | OneBot12 字段 | 备注                             | 默认值 |
| ------------- | ------------- | -------------------------------- | ------ |
| `notice_type` | 无            | 以下转换规则中的OneBot11事件类型 | 无     |

#### `group_upload` -> `message.group`

本事件较为特殊，因为OneBot12协议中文件是消息段形式的，并非事件形式，所以会被转换成消息事件

| OneBot11 字段 | OneBot12 字段 | 备注                       | 默认值             |
| ------------- | ------------- | -------------------------- | ------------------ |
| 无            | `alt_message` | 无                         | `[OneBot 11 File]` |
| `file`        | `message[0]`  | 转换成OneBot12的文件消息段 | 无                 |
| 无            | `message_id`  | 由实现自行决定             | 无                 |

文件事件`file`字段转换到文件消息段的规则

| OneBot11 字段 | OneBot12 字段 | 备注 | 默认值 |
| ------------- | ------------- | ---- | ------ |
| `id`          | `file_id`     | 无   | 无     |

#### `group_increase` -> `notice.group_member_increase`

| OneBot11 字段 | OneBot12 字段 | 备注                    | 默认值 |
| ------------- | ------------- | ----------------------- | ------ |
| `sub_type`    | `sub_type`    | `approve`会转换成`join` | 无     |

#### `group_decrease` -> `notice.group_member_decrease`

| OneBot11 字段 | OneBot12 字段 | 备注                    | 默认值 |
| ------------- | ------------- | ----------------------- | ------ |
| `sub_type`    | `sub_type`    | `kick_me`会转换成`kick` | 无     |

#### `friend_add` -> `notice.friend_increase`

字段均保持不变。

#### `group_recall` -> `notice.group_message_delete`

| OneBot11 字段 | OneBot12 字段 | 备注                             | 默认值 |
| ------------- | ------------- | -------------------------------- | ------ |
| 无            | `sub_type`    | 根据`operator_id`和`user_id`决定 | 无     |

#### `friend_recall` -> `notice.private_message_delete`

字段均保持不变。

### 2.4 请求事件 转换规则(ob11 - ob12)

**共用字段转换规则**

| OneBot11 字段  | OneBot12 字段 | 备注                             | 默认值 |
| -------------- | ------------- | -------------------------------- | ------ |
| `request_type` | 无            | 以下转换规则中的OneBot11事件类型 | 无     |

OneBot11中的请求事件全部转换为拓展事件。

## 3. 动作

-   `echo`字段OneBot11和12均存在，不变
-   OneBot 12的`self`字段的`user_id`用于根据ID确定使用的OneBot11实现端（当`self`的`platform`字段为`ob11`）
-   可转换的OneBot11 API都会在下方列出，部分OneBot11 API因为重复会被丢弃或是合并
-   未提及的相同名称动作/响应/API字段或相同值保持**不变**。
-   如果某个OneBot11 API的**字段或值**对应的OneBot12动作**不存在**且**以下内容未提及如何转换**，则会被加上`ob11.`前缀作为拓展字段/值。
-   -> / <- / <-> 代表转换方向，`->`表示OneBot11转OneBot12，`<-`表示OneBot12转OneBot11，`<->`表示双向转换，OneBot11 消息段名称在前，OneBot12 事件名称在后。

以下的转换规则内容为参数的转换规则。

### 3.1 转换规则

以下的响应转换规则转换方向与动作转换规则相反。

#### `send_msg` <- `send_message`

| OneBot11 字段  | OneBot12 字段 | 备注 | 默认值 |
| -------------- | ------------- | ---- | ------ |
| `message_type` | `detail_type` | 无   | 无     |

**响应**

字段均不变。

#### `delete_msg` <- `delete_message`

字段均不变。

**响应**

字段均不变。

#### `get_msg` <- `ob11.get_msg`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `get_forward_msg` <- `ob11.get_forward_msg`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `send_like` <- `ob11.send_like`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_kick` <- `ob11.set_group_kick`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_ban` <- `ob11.set_group_ban`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_anonymous_ban` <- `ob11.set_group_anonymous_ban`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_whole_ban` <- `ob11.set_group_whole_ban`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_admin` <- `ob11.set_group_admin`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_anonymous` <- `ob11.set_group_anonymous`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_card` <- `ob11.set_group_card`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_name` <- `set_group_name`

字段均不变。

**响应**

字段均不变。

#### `set_group_leave` <- `leave_group`

| OneBot11 字段 | OneBot12 字段     | 备注 | 默认值  |
| ------------- | ----------------- | ---- | ------- |
| `group_id`    | `group_id`        | 无   | 无      |
| `is_dismiss`  | `ob11.is_dismiss` | 无   | `false` |

**响应**

字段均不变。

#### `set_group_special_title` <- `ob11.set_group_special_title`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_friend_add_request` <- `ob11.set_friend_add_request`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `set_group_add_request` <- `ob11.set_group_add_request`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `get_login_info` <- `get_self_info`

字段均不变。

**响应**

| OneBot11 字段 | OneBot12 字段      | 备注 | 默认值 |
| ------------- | ------------------ | ---- | ------ |
| `user_id`     | `user_id`          | 无   | 无     |
| `user_id`     | `user_name`        | 无   | 无     |
| `nickname`    | `user_displayname` | 无   | 无     |

#### `get_stranger_info` <- `get_user_info`

| OneBot11 字段 | OneBot12 字段   | 备注 | 默认值  |
| ------------- | --------------- | ---- | ------- |
| `no_cache`    | `ob11.no_cache` | 无   | `false` |

**响应**

| OneBot11 字段 | OneBot12 字段      | 备注 | 默认值   |
| ------------- | ------------------ | ---- | -------- |
| `user_id`     | `user_id`          | 无   | 无       |
| `user_id`     | `user_name`        | 无   | 无       |
| `nickname`    | `user_displayname` | 无   | 无       |
| 无            | `user_remark`      | 无   | 空字符串 |
| `sex`         | `ob11.sex`         | 无   | 无       |
| `age`         | `ob11.age`         | 无   | 无       |

#### `get_friend_list` <- `get_friend_list`

字段均不变。

**响应**

由以下元素组成的数组

| OneBot11 字段 | OneBot12 字段      | 备注 | 默认值 |
| ------------- | ------------------ | ---- | ------ |
| `user_id`     | `user_id`          | 无   | 无     |
| `user_id`     | `user_name`        | 无   | 无     |
| `nickname`    | `user_displayname` | 无   | 无     |
| `remark`      | `user_remark`      | 无   | 无     |

#### `get_group_info` <- `get_group_info`

| OneBot11 字段 | OneBot12 字段   | 备注 | 默认值  |
| ------------- | --------------- | ---- | ------- |
| `no_cache`    | `ob11.no_cache` | 无   | `false` |

**响应**

| OneBot11 字段      | OneBot12 字段           | 备注 | 默认值 |
| ------------------ | ----------------------- | ---- | ------ |
| `member_count`     | `ob11.member_count`     | 无   | 无     |
| `max_member_count` | `ob11.max_member_count` | 无   | 无     |

#### `get_group_list` <- `get_group_list`

字段均不变。

**响应**

由[`get_group_info`](#getgroupinfo-getgroupinfo)动作响应作为元素组成的列表

#### `get_group_member_info` <- `get_group_member_info`

| OneBot11 字段 | OneBot12 字段   | 备注 | 默认值  |
| ------------- | --------------- | ---- | ------- |
| `no_cache`    | `ob11.no_cache` | 无   | `false` |

**响应**

| OneBot11 字段       | OneBot12 字段            | 备注 | 默认值   |
| ------------------- | ------------------------ | ---- | -------- |
| `nickname`          | `user_name`              | 无   | 空字符串 |
| `card`              | `user_displayname`       | 无   | 空字符串 |
| `sex`               | `ob11.sex`               | 无   | 无       |
| `age`               | `ob11.age`               | 无   | 无       |
| `area`              | `ob11.area`              | 无   | 无       |
| `join_time`         | `ob11.join_time`         | 无   | 无       |
| `last_sent_time`    | `ob11.last_sent_time`    | 无   | 无       |
| `level`             | `ob11.level`             | 无   | 无       |
| `role`              | `ob11.role`              | 无   | 无       |
| `unfriendly`        | `ob11.unfriendly`        | 无   | 无       |
| `title`             | `ob11.title`             | 无   | 无       |
| `title_expire_time` | `ob11.title_expire_time` | 无   | 无       |
| `card_changeable`   | `ob11.card_changeable`   | 无   | 无       |

#### `get_group_member_list` <- `get_group_member_list`

**响应**

由[`get_group_member_info`](#getgroupmemberinfo-getgroupmemberinfo)动作响应作为元素构成的列表

#### `get_group_honor_info` <- `ob11.get_group_honor_info`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `get_cookies` <- `ob11.get_cookies`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `get_csrf_token` <- `ob11.get_csrf_token`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `get_credentials` <- `ob11.get_credentials`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `get_record`, `get_image` <- `get_file`

因为OneBot11协议中获取图片和语音的动作名称不同，所以要根据`file_id`来决定

| OneBot11 字段 | OneBot12 字段     | 备注                         | 默认值 |
| ------------- | ----------------- | ---------------------------- | ------ |
| `file`        | 无                | 由实现根据`file_id`决定      | 无     |
| `out_format`  | `ob11.out_format` | 如果是语音文件，本字段会存在 | `mp3`  |
| 无            | `file_id`         | 无                           | 无     |
| 无            | `type`            | 只能为`path`                 | 无     |

**响应**

| OneBot11 字段 | OneBot12 字段 | 备注   | 默认值 |
| ------------- | ------------- | ------ | ------ |
| 无            | `name`        | 文件名 | 无     |
| `file`        | `path`        | 无     | 无     |

#### `can_send_image` <- `ob11.can_send_image`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `can_send_record` <- `ob11.can_send_record`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `get_status` <- `get_status`

由于OneBot11并不适用于多个Bot的环境，所以转换规则会将部分字段转换到`bots`字段的元素中

**响应**

| OneBot11 字段 | OneBot12 字段        | 备注           | 默认值       |
| ------------- | -------------------- | -------------- | ------------ |
| `good`        | `good`               | 由实现自行判断 | 无           |
| `online`      | `bots[_]` : `online` | 无             | 无           |
| 无            | `self`: `user_id`    | Bot用户ID      | 无           |
| 无            | `self`: `platform`   | 无             | OB12: `ob11` |

OneBot 实现自行添加的其它内容字段名称都会加上`ob11.extra.`前缀

#### `get_version_info` <- `get_version`

拓展动作，字段均不变。

**响应**

| OneBot11 字段 | OneBot12 字段    | 备注 | 默认值 |
| ------------- | ---------------- | ---- | ------ |
| 无            | `impl`           | 无   | `ob11` |
| `app_name`    | `ob11.app_name`  | 无   | 无     |
| `app_version` | `version`        | 无   | 无     |
| 无            | `onebot_version` | 无   | `12`   |

OneBot 实现自行添加的其它内容字段名称都会加上`ob11.extra.`前缀

#### `set_restart` <- `ob11.set_restart`

拓展动作，字段均不变。

**响应**

字段均不变。

#### `clean_cache` <- `ob11.clean_cache`

拓展动作，字段均不变。

**响应**

字段均不变。
