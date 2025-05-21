# Dota2 Timer

![Version](https://img.shields.io/badge/version-0.9.2-green.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)

# Overview
- 基于Rust的Dota2计时提醒工具。

![example](example/example.png)

# Quick Start
## 1. 程序目录结构
    ```
    - dota2-timer
        - dota2-timer.exe
        - config
            - default.json
            - [自定义的配置].json
    ```
## 2. 配置文件

### 1. 示例
```json
[
    {
        "period": -1,
        "text": "出兵",
        "start_time": 0,
        "end_time": 0,
        "repeat_count": 1,
        "early_notice_time": 10
    },
    {
        "period": 120,
        "text": "功能神符",
        "start_time": 360,
        "end_time": 1800,
        "repeat_count": -1,
        "early_notice_time": 15
    }
]
```

### 2.配置说明
| 字段 | 类型 | 说明 | 备注 |
|-----|-----|-----|-----|
| period | 数字（秒） | 间隔多少时间触发 | -1代表不再次触发 |
| text | 文字 | 展示文本 | 不能超过6个字 |
| start_time | 数字（秒） | 触发开始时间 | 不能小于-90 |
| end_time | 数字（秒） | 触发结束时间 | 不能小于-90，-1代表5小时 |
| repeat_count | 数字（次） | 触发最大次数 | -1代表300次 |
| early_notice_time | 数字（秒） | 提前多少时间提醒 | -1代表不提醒 |

# Todo List

1. 买活时间记录并提醒。
