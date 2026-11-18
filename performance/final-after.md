# Editor performance report

- Machine: darwin/arm64, Apple M5
- Reference machine: m4-mbp (physical), Apple M4
- Time budgets: not judged: CPU differs from the reference machine
- Large size: 10MiB
- Base machine: darwin/arm64, Apple M5

Summary: 125 results, 82 within budget, 0 over budget, 0 without budget, 43 not judged

| Benchmark | ns/op | B/op | allocs/op | extra metrics | vs base | Budget |
| --- | ---: | ---: | ---: | --- | --- | --- |
| BenchmarkEditorRopeBuild/go/100KiB | 8.42 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 12170 | -4.3% ns/op, allocs = | — not judged |
| BenchmarkEditorRopeBuild/go/10MiB | 596 µs (not judged: CPU differs from the reference machine) | 10.5 MiB | 5896 | MB/s: 17600 | -6.8% ns/op, allocs = | — not judged |
| BenchmarkEditorRopeTypePair/go/100KiB | 197 ns (not judged: CPU differs from the reference machine) | 2.44 KiB | 7 |  | -6.1% ns/op, allocs = | ✅ within |
| BenchmarkEditorRopeTypePair/go/10MiB | 291 ns (not judged: CPU differs from the reference machine) | 3.02 KiB | 13 |  | -10.2% ns/op, allocs = | ✅ within (scale 1.47x vs 100KiB <= 2x) |
| BenchmarkEditorRopeLineStart/go/100KiB | 259 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -5.1% ns/op, allocs = | ✅ within |
| BenchmarkEditorRopeLineStart/go/10MiB | 315 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -2.1% ns/op, allocs = | ✅ within (scale 1.22x vs 100KiB <= 2x) |
| BenchmarkEditorRopeLineOf/go/100KiB | 27.5 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -1.1% ns/op, allocs = | ✅ within |
| BenchmarkEditorRopeLineOf/go/10MiB | 38.8 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -9.7% ns/op, allocs = | ✅ within (scale 1.41x vs 100KiB <= 2x) |
| BenchmarkEditorRopeViewportLines/go/100KiB | 25.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -1.4% ns/op, allocs = | ✅ within |
| BenchmarkEditorRopeViewportLines/go/10MiB | 38.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -2.4% ns/op, allocs = | ✅ within (scale 1.51x vs 100KiB <= 2x) |
| BenchmarkEditorRopeBytes/go/100KiB | 3.84 µs (not judged: CPU differs from the reference machine) | 104 KiB | 1 | MB/s: 26680 | -2.5% ns/op, allocs = | — not judged |
| BenchmarkEditorRopeBytes/go/10MiB | 343 µs (not judged: CPU differs from the reference machine) | 10 MiB | 1 | MB/s: 30570 | -16.5% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxViewportCold/go/100KiB | 859 µs (not judged: CPU differs from the reference machine) | 141 KiB | 629 |  | -7.6% ns/op, allocs -1 | — not judged |
| BenchmarkEditorSyntaxViewportCold/go/10MiB | 913 µs (not judged: CPU differs from the reference machine) | 259 KiB | 756 |  | -6.7% ns/op, allocs -1 | ✅ within (scale 1.06x vs 100KiB <= 2x) |
| BenchmarkEditorSyntaxViewportCold/java/100KiB | 671 µs (not judged: CPU differs from the reference machine) | 455 KiB | 544 |  | -8.2% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxViewportCold/java/10MiB | 680 µs (not judged: CPU differs from the reference machine) | 447 KiB | 447 |  | -5.9% ns/op, allocs -1 | ✅ within (scale 1.01x vs 100KiB <= 2x) |
| BenchmarkEditorSyntaxViewportCold/markdown/100KiB | 759 µs (not judged: CPU differs from the reference machine) | 929 KiB | 598.5 |  | -4.6% ns/op, allocs -6 | — not judged |
| BenchmarkEditorSyntaxViewportCold/markdown/10MiB | 769 µs (not judged: CPU differs from the reference machine) | 929 KiB | 644 |  | -4.2% ns/op, allocs -4 | ✅ within (scale 1.01x vs 100KiB <= 2x) |
| BenchmarkEditorSyntaxViewportWarm/go/100KiB | 1.17 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -0.9% ns/op, allocs = | ✅ within |
| BenchmarkEditorSyntaxViewportWarm/go/10MiB | 1.26 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | +0.2% ns/op, allocs = | ✅ within (scale 1.07x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxViewportWarm/java/100KiB | 1.21 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | +1.2% ns/op, allocs = | ✅ within |
| BenchmarkEditorSyntaxViewportWarm/java/10MiB | 1.15 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | +2.8% ns/op, allocs = | ✅ within (scale 0.95x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxViewportWarm/markdown/100KiB | 1.11 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -2.5% ns/op, allocs = | ✅ within |
| BenchmarkEditorSyntaxViewportWarm/markdown/10MiB | 1.1 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -3.2% ns/op, allocs = | ✅ within (scale 0.99x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeInLine/go/100KiB | 7.61 µs (not judged: CPU differs from the reference machine) | 2.6 KiB | 9 |  | -1.8% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxTypeInLine/go/10MiB | 7.76 µs (not judged: CPU differs from the reference machine) | 3.14 KiB | 15 |  | -2.9% ns/op, allocs = | ✅ within (scale 1.02x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeInLine/java/100KiB | 24.7 µs (not judged: CPU differs from the reference machine) | 3.17 KiB | 15 |  | -1.2% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxTypeInLine/java/10MiB | 24.5 µs (not judged: CPU differs from the reference machine) | 3.72 KiB | 21 |  | -7.7% ns/op, allocs = | ✅ within (scale 0.99x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeInLine/markdown/100KiB | 15.5 µs (not judged: CPU differs from the reference machine) | 2.76 KiB | 10 |  | -1.0% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxTypeInLine/markdown/10MiB | 15.8 µs (not judged: CPU differs from the reference machine) | 3.3 KiB | 16 |  | -0.8% ns/op, allocs = | ✅ within (scale 1.02x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeCatchUp/go/100KiB | 735 µs (not judged: CPU differs from the reference machine) | 33.1 KiB | 372 |  | -7.6% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxTypeCatchUp/go/10MiB | 664 µs (not judged: CPU differs from the reference machine) | 34.1 KiB | 374 |  | -9.1% ns/op, allocs = | ✅ within (scale 0.90x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeCatchUp/java/100KiB | 592 µs (not judged: CPU differs from the reference machine) | 30.3 KiB | 319 |  | -9.3% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxTypeCatchUp/java/10MiB | 467 µs (not judged: CPU differs from the reference machine) | 13.2 KiB | 127 |  | -13.4% ns/op, allocs = | ✅ within (scale 0.79x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeCatchUp/markdown/100KiB | 804 µs (not judged: CPU differs from the reference machine) | 23.8 KiB | 344 |  | -12.8% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxTypeCatchUp/markdown/10MiB | 754 µs (not judged: CPU differs from the reference machine) | 25 KiB | 407 |  | -14.9% ns/op, allocs = | ✅ within (scale 0.94x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxFullPass/go/100KiB | 46.1 ms (not judged: CPU differs from the reference machine) | 6.02 MiB | 28993 | MB/s: 2.22 | -19.7% ns/op, allocs -362 | — not judged |
| BenchmarkEditorSyntaxFullPass/java/100KiB | 35.8 ms (not judged: CPU differs from the reference machine) | 4.99 MiB | 15389 | MB/s: 2.865 | -21.0% ns/op, allocs -27 | — not judged |
| BenchmarkEditorSyntaxFullPass/markdown/100KiB | 21.1 ms (not judged: CPU differs from the reference machine) | 4.78 MiB | 16105 | MB/s: 4.865 | -16.4% ns/op, allocs -712 | — not judged |
| BenchmarkEditorSyntaxFirstUse/go | 2.24 ms (not judged: CPU differs from the reference machine) | 15.8 MiB | 34770 |  | -24.2% ns/op, allocs -9 | — not judged |
| BenchmarkEditorSyntaxFirstUse/java | 2.5 ms (not judged: CPU differs from the reference machine) | 30.6 MiB | 23088 |  | -20.2% ns/op, allocs +1 | — not judged |
| BenchmarkEditorSyntaxFirstUse/markdown | 5.06 ms (not judged: CPU differs from the reference machine) | 35.3 MiB | 78387 |  | -18.7% ns/op, allocs -1 | — not judged |
| BenchmarkEditorSyntaxSemanticBuild/go/100KiB | 34.4 µs (not judged: CPU differs from the reference machine) | 35.4 KiB | 157 |  | -16.4% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxSemanticBuild/go/10MiB | 4.83 ms (not judged: CPU differs from the reference machine) | 3.38 MiB | 13853 |  | -14.9% ns/op, allocs = | — not judged |
| BenchmarkEditorSyntaxSemanticBuild/longline/100KiB | 74.3 µs (not judged: CPU differs from the reference machine) | 608 KiB | 16 |  | -15.2% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPaint/plain/100KiB | 40.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -9.9% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/plain/10MiB | 37.7 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -9.6% ns/op, allocs = | ✅ within (scale 0.93x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/go/100KiB | 42.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -9.8% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/go/10MiB | 39.2 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -15.0% ns/op, allocs = | ✅ within (scale 0.92x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/java/100KiB | 31.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -10.8% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/java/10MiB | 30.3 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -19.3% ns/op, allocs = | ✅ within (scale 0.96x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/wrap/100KiB | 41.3 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -11.8% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/wrap/10MiB | 38.2 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -10.5% ns/op, allocs = | ✅ within (scale 0.92x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/minimap/100KiB | 58.7 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -13.4% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/minimap/10MiB | 55.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -5.2% ns/op, allocs = | ✅ within (scale 0.95x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/longline/100KiB | 1.32 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -16.3% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/longline/10MiB | 1.35 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -14.2% ns/op, allocs = | ✅ within (scale 1.02x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/cjk/100KiB | 25.2 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -12.9% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/cjk/10MiB | 23.9 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -9.5% ns/op, allocs = | ✅ within (scale 0.95x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/mixed/100KiB | 27.6 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -14.4% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/mixed/10MiB | 28.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -12.1% ns/op, allocs = | ✅ within (scale 1.03x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/selection/100KiB | 43.1 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -8.3% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/selection/10MiB | 39.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -6.8% ns/op, allocs = | ✅ within (scale 0.91x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/markers/100KiB | 51.7 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -5.3% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/markers/10MiB | 47 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -8.6% ns/op, allocs = | ✅ within (scale 0.91x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/gutter/100KiB | 44.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -5.2% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPaint/gutter/10MiB | 41.2 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -11.3% ns/op, allocs = | ✅ within (scale 0.93x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewType/go/100KiB | 48.3 µs (not judged: CPU differs from the reference machine) | 2.72 KiB | 7 |  | -5.6% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewType/go/10MiB | 44.9 µs (not judged: CPU differs from the reference machine) | 4.66 KiB | 31 |  | -10.3% ns/op, allocs = | ✅ within (scale 0.93x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/longline/100KiB | 2.55 µs (not judged: CPU differs from the reference machine) | 1.74 KiB | 7 |  | -16.4% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewType/longline/10MiB | 2.97 µs (not judged: CPU differs from the reference machine) | 2.32 KiB | 13 |  | -17.9% ns/op, allocs = | ✅ within (scale 1.17x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/longline-wrap/100KiB | 36.7 µs (not judged: CPU differs from the reference machine) | 2.07 KiB | 9 |  | -15.3% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewType/longline-wrap/10MiB | 73.2 µs (not judged: CPU differs from the reference machine) | 2.81 KiB | 16 |  | -14.7% ns/op, allocs = | ✅ within (scale 1.99x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/minimap/100KiB | 362 µs (not judged: CPU differs from the reference machine) | 2.78 KiB | 7 |  | -10.8% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewType/minimap/10MiB | 344 µs (not judged: CPU differs from the reference machine) | 5.07 KiB | 31 |  | -6.8% ns/op, allocs = | ✅ within (scale 0.95x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/longline-wrap-minimap/100KiB | 235 µs (not judged: CPU differs from the reference machine) | 3.15 KiB | 21 |  | -16.8% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewType/longline-wrap-minimap/10MiB | 283 µs (not judged: CPU differs from the reference machine) | 4.14 KiB | 34 |  | -12.7% ns/op, allocs = | ✅ within (scale 1.20x vs 100KiB <= 2x) |
| BenchmarkEditorViewEnter/go/100KiB | 39.5 µs (not judged: CPU differs from the reference machine) | 2.67 KiB | 7 |  | -12.0% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewEnter/go/10MiB | 34.7 µs (not judged: CPU differs from the reference machine) | 3.24 KiB | 13 |  | -11.4% ns/op, allocs = | ✅ within (scale 0.88x vs 100KiB <= 2x) |
| BenchmarkEditorViewUndoRedo/go/100KiB | 48.3 µs (not judged: CPU differs from the reference machine) | 86 B | 0 |  | -13.6% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewUndoRedo/go/10MiB | 44.5 µs (not judged: CPU differs from the reference machine) | 1.44 KiB | 18 |  | -12.2% ns/op, allocs = | ✅ within (scale 0.92x vs 100KiB <= 2x) |
| BenchmarkEditorViewCaretMove/go/100KiB | 44 µs (not judged: CPU differs from the reference machine) | 32 B | 0 |  | -13.0% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewCaretMove/go/10MiB | 32.7 µs (not judged: CPU differs from the reference machine) | 32 B | 0 |  | -9.1% ns/op, allocs = | ✅ within (scale 0.74x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPageMove/go/100KiB | 37 µs (not judged: CPU differs from the reference machine) | 40 B | 0 |  | -9.9% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewPageMove/go/10MiB | 34.6 µs (not judged: CPU differs from the reference machine) | 45 B | 0 |  | -13.0% ns/op, allocs = | ✅ within (scale 0.94x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewScroll/go/100KiB | 43.9 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -9.6% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewScroll/go/10MiB | 39.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | -10.4% ns/op, allocs = | ✅ within (scale 0.90x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaste/go/100KiB | 3.12 ms (not judged: CPU differs from the reference machine) | 1.17 MiB | 1968 |  | -15.2% ns/op, allocs -3 | — not judged |
| BenchmarkEditorViewPaste/go/10MiB | 3.2 ms (not judged: CPU differs from the reference machine) | 1.18 MiB | 2083 |  | -15.9% ns/op, allocs -2 | ✅ within (scale 1.03x vs 100KiB <= 2x) |
| BenchmarkEditorViewOpen/go/100KiB | 932 µs (not judged: CPU differs from the reference machine) | 171 KiB | 1186 |  | -22.3% ns/op, allocs = | — not judged |
| BenchmarkEditorViewOpen/go/10MiB | 936 µs (not judged: CPU differs from the reference machine) | 171 KiB | 1186 |  | -16.2% ns/op, allocs = | ✅ within (scale 1.01x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/java/100KiB | 1.16 ms (not judged: CPU differs from the reference machine) | 195 KiB | 1133 |  | -18.8% ns/op, allocs = | — not judged |
| BenchmarkEditorViewOpen/java/10MiB | 1.17 ms (not judged: CPU differs from the reference machine) | 195 KiB | 1133 |  | -15.9% ns/op, allocs = | ✅ within (scale 1.01x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/json/100KiB | 71 µs (not judged: CPU differs from the reference machine) | 122 KiB | 517 |  | -14.2% ns/op, allocs = | — not judged |
| BenchmarkEditorViewOpen/json/10MiB | 71.8 µs (not judged: CPU differs from the reference machine) | 122 KiB | 517 |  | -11.5% ns/op, allocs = | ✅ within (scale 1.01x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/markdown/100KiB | 1.31 ms (not judged: CPU differs from the reference machine) | 235 KiB | 1195 |  | -16.3% ns/op, allocs = | — not judged |
| BenchmarkEditorViewOpen/markdown/10MiB | 1.3 ms (not judged: CPU differs from the reference machine) | 235 KiB | 1195 |  | -19.6% ns/op, allocs = | ✅ within (scale 1.00x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/log/100KiB | 99.1 µs (not judged: CPU differs from the reference machine) | 89 KiB | 501 |  | -18.1% ns/op, allocs = | — not judged |
| BenchmarkEditorViewOpen/log/10MiB | 98.6 µs (not judged: CPU differs from the reference machine) | 89 KiB | 501 |  | -20.8% ns/op, allocs = | ✅ within (scale 1.00x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/mixed/100KiB | 87.4 µs (not judged: CPU differs from the reference machine) | 91.2 KiB | 506 |  | -17.7% ns/op, allocs = | — not judged |
| BenchmarkEditorViewOpen/mixed/10MiB | 87.7 µs (not judged: CPU differs from the reference machine) | 91.2 KiB | 506 |  | -20.9% ns/op, allocs = | ✅ within (scale 1.00x vs 100KiB <= 4x) |
| BenchmarkEditorViewPrepareReload/go/100KiB | 107 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 958.5 | -13.1% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/go/10MiB | 8.61 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5896 | MB/s: 1217 | -11.5% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/java/100KiB | 97.5 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 1050 | -14.0% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/java/10MiB | 8 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5893 | MB/s: 1311 | -11.4% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/json/100KiB | 117 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 873.8 | -18.4% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/json/10MiB | 12.1 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5829 | MB/s: 864.4 | -19.7% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/markdown/100KiB | 72.8 µs (not judged: CPU differs from the reference machine) | 109 KiB | 69 | MB/s: 1408 | -15.8% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/markdown/10MiB | 5.46 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5982 | MB/s: 1921 | -16.9% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/log/100KiB | 36.6 µs (not judged: CPU differs from the reference machine) | 109 KiB | 69 | MB/s: 2802 | -12.9% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/log/10MiB | 3.32 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5996 | MB/s: 3162 | -17.2% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/mixed/100KiB | 61.2 µs (not judged: CPU differs from the reference machine) | 109 KiB | 69 | MB/s: 1673 | -13.1% ns/op, allocs = | — not judged |
| BenchmarkEditorViewPrepareReload/mixed/10MiB | 5.45 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5994 | MB/s: 1925 | -12.6% ns/op, allocs = | — not judged |
| BenchmarkEditorViewRewrap/go/100KiB | 85.3 µs (not judged: CPU differs from the reference machine) | 256 B | 15 |  | -13.4% ns/op, allocs = | — not judged |
| BenchmarkEditorViewRewrap/go/10MiB | 6.05 ms (not judged: CPU differs from the reference machine) | 152 B | 9 |  | -11.2% ns/op, allocs = | — not judged |
| BenchmarkEditorViewMemory/go/100KiB | 3.15 ms | 286 KiB | 1293 | retained-B: 65.4 KiB, retained/text-byte: 0.6541 | -17.9% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewMemory/go/10MiB | 12.1 ms | 10.7 MiB | 7121 | retained-B: 450 KiB, retained/text-byte: 0.04398 | -8.3% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewMemory/json/100KiB | 2.22 ms | 238 KiB | 624 | retained-B: 63.5 KiB, retained/text-byte: 0.6351 | -14.1% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewMemory/json/10MiB | 15.1 ms | 10.7 MiB | 6385 | retained-B: 444 KiB, retained/text-byte: 0.04337 | -12.5% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewMemory/markdown/100KiB | 3.46 ms | 388 KiB | 1310 | retained-B: 106 KiB, retained/text-byte: 1.057 | -11.5% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewMemory/markdown/10MiB | 9.15 ms | 10.8 MiB | 7223 | retained-B: 496 KiB, retained/text-byte: 0.04847 | -12.9% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewMemory/mixed/100KiB | 2.31 ms | 207 KiB | 614 | retained-B: 42.9 KiB, retained/text-byte: 0.4287 | -13.0% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewMemory/mixed/10MiB | 7.87 ms | 10.6 MiB | 6539 | retained-B: 434 KiB, retained/text-byte: 0.04241 | -12.8% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewReload/go/100KiB | 33 ns (not judged: CPU differs from the reference machine) | 183 B | 0 |  | -9.6% ns/op, allocs = | ✅ within |
| BenchmarkEditorViewReload/go/10MiB | 60.8 ns (not judged: CPU differs from the reference machine) | 183 B | 0 |  | -10.3% ns/op, allocs = | ✅ within (scale 1.84x vs 100KiB <= 2x) |

This report never fails the build; overruns are findings to fix or to justify in the PR.

