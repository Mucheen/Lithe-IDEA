# 内联提示重试截图

源码提交：`dffaec794f2c701b886365f767c1fc9143f3da90`。分辨率为 2880 × 1800，覆盖英文、简体中文以及明暗主题。

`inlay-retry-pending-*` 显示第一次请求失败、等待重试时的编辑器。
`inlay-hints-*` 显示原视口在重试成功后绘制的提示以及鼠标悬停时的说明。
`editor-before-*`、`code-actions-*` 和 `signature-help-*` 为同一次无头运行的编辑器、代码操作和签名帮助截图。
`inlay-review.png` 是失败后等待重试与恢复后的对照。

```bash
LITHE_VISUAL_OUTPUT_DIR=/absolute/path/to/screenshots \
  scripts/desktop/desktop-go.sh test -tags 'vivahead lithevisual' \
    ./internal/workbench -run '^TestVisualLSPActions$' -count=1 -timeout=1m
```
