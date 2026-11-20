# PR #2217 首行高亮对照

- 修复前：桌面 `7ea7eb7f4` 的工作台实现。
- 修复后：桌面 `3ff530de8`。两侧均使用 Core `3c0fcff`。
- 每组覆盖 Git Diff 的可编辑侧及全局搜索的只读预览。查找第二行的 `sample`。截图使用中英文及明暗主题，窗口 1280×800，输出 2560×1600。
- 采集命令：`LITHE_VISUAL_OUTPUT_DIR=<目录> scripts/desktop/desktop-go.sh test -tags 'vivahead lithevisual' ./internal/workbench -run '^TestVisualSearchFirstLineLanguage$' -count=1`。修复前以 Go overlay 使用上述基线的工作台实现。
