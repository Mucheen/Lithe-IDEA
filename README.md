# Desktop LSP editor verification

- Baseline: `4d67a97d1`. Candidate: `dc510f1fd`.
- `screenshots/` contains the same synthetic workbench scene without a reply, with code actions, with signature help, and with inlay hints. Each scene covers English and Simplified Chinese in light and dark themes. Dimensions: 2880 × 1800.
- `settings/` shows the inlay setting in dark Simplified Chinese and light English. Dimensions: 1800 × 1400.
- `performance/` contains the Apple M5 comparison. The first long-line wrap ratio exceeded its budget. The final six-sample comparison after the fast-path fix is within the 2× budget. The independent syntax viewport still shows three additional median allocations in a single Markdown case. No syntax-package source changed.

The workbench capture uses `desktop-go.sh test -tags 'vivahead lithevisual' ./internal/workbench -run '^TestVisualLSPActions$'`.
