## show_text

### 介绍

用于实现 KPlayer 推流过程中在指定位置显示文本文字的插件

### 参数

### 参数列表

| 参数      | 默认值            | 取值范围 | 说明                                                                                 |
| --------- | ----------------- | -------- | ------------------------------------------------------------------------------------ |
| text      | none              | 无       | 定义显示文字，支持转义字符`\n`进行换行显示。支持的语言由字体决定，默认字体支持中英文 |
| font_size | 17                | 整型     | 定义字体大小                                                                         |
| fontcolor | white             | 无       | 支持语义化颜色值，也支持十六机制颜色值                                               |
| fontfile  | resource/font.ttf | 无       | 字体文件路径                                                                         |
| x         | 0                 | 整型     | x 轴坐标值值。以屏幕左上角为原点，最大可显示范围为当前设置的分辨率                   |
| y         | 0                 | 整型     | y 轴坐标值值。以屏幕左上角为原点，最大可显示范围为当前设置的分辨率                   |
