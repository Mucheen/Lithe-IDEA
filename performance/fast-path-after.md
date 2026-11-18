# Editor performance report

- Machine: darwin/arm64, Apple M5
- Reference machine: m4-mbp (physical), Apple M4
- Time budgets: not judged: CPU differs from the reference machine
- Large size: 10MiB
- Base machine: darwin/arm64, Apple M5

Summary: 2 results, 2 within budget, 0 over budget, 0 without budget

| Benchmark | ns/op | B/op | allocs/op | extra metrics | vs base | Budget |
| --- | ---: | ---: | ---: | --- | --- | --- |
| BenchmarkEditorViewType/longline-wrap/100KiB | 37.6 µs (not judged: CPU differs from the reference machine) | 2.08 KiB | 9 |  | -3.6% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewType/longline-wrap/10MiB | 73.9 µs (not judged: CPU differs from the reference machine) | 2.81 KiB | 16 |  | -4.8% ns/op, allocs = | ✅ within (scale 1.97x vs 100KiB <= 2x) |

This report never fails the build; overruns are findings to fix or to justify in the PR.

