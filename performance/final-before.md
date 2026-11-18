# Editor performance report

- Machine: darwin/arm64, Apple M5
- Reference machine: m4-mbp (physical), Apple M4
- Time budgets: not judged: CPU differs from the reference machine
- Large size: 10MiB

Summary: 125 results, 82 within budget, 0 over budget, 0 without budget, 43 not judged

| Benchmark | ns/op | B/op | allocs/op | extra metrics | vs base | Budget |
| --- | ---: | ---: | ---: | --- | --- | --- |
| BenchmarkEditorRopeBuild/go/100KiB | 8.79 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 11650 | — | — not judged |
| BenchmarkEditorRopeBuild/go/10MiB | 639 µs (not judged: CPU differs from the reference machine) | 10.5 MiB | 5896 | MB/s: 16410 | — | — not judged |
| BenchmarkEditorRopeTypePair/go/100KiB | 210 ns (not judged: CPU differs from the reference machine) | 2.44 KiB | 7 |  | — | ✅ within |
| BenchmarkEditorRopeTypePair/go/10MiB | 324 ns (not judged: CPU differs from the reference machine) | 3.02 KiB | 13 |  | — | ✅ within (scale 1.54x vs 100KiB <= 2x) |
| BenchmarkEditorRopeLineStart/go/100KiB | 273 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorRopeLineStart/go/10MiB | 322 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.18x vs 100KiB <= 2x) |
| BenchmarkEditorRopeLineOf/go/100KiB | 27.8 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorRopeLineOf/go/10MiB | 42.9 ns (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.54x vs 100KiB <= 2x) |
| BenchmarkEditorRopeViewportLines/go/100KiB | 25.7 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorRopeViewportLines/go/10MiB | 39.3 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.53x vs 100KiB <= 2x) |
| BenchmarkEditorRopeBytes/go/100KiB | 3.94 µs (not judged: CPU differs from the reference machine) | 104 KiB | 1 | MB/s: 26000 | — | — not judged |
| BenchmarkEditorRopeBytes/go/10MiB | 411 µs (not judged: CPU differs from the reference machine) | 10 MiB | 1 | MB/s: 25520 | — | — not judged |
| BenchmarkEditorSyntaxViewportCold/go/100KiB | 930 µs (not judged: CPU differs from the reference machine) | 141 KiB | 630 |  | — | — not judged |
| BenchmarkEditorSyntaxViewportCold/go/10MiB | 979 µs (not judged: CPU differs from the reference machine) | 259 KiB | 757 |  | — | ✅ within (scale 1.05x vs 100KiB <= 2x) |
| BenchmarkEditorSyntaxViewportCold/java/100KiB | 730 µs (not judged: CPU differs from the reference machine) | 455 KiB | 544 |  | — | — not judged |
| BenchmarkEditorSyntaxViewportCold/java/10MiB | 723 µs (not judged: CPU differs from the reference machine) | 447 KiB | 448 |  | — | ✅ within (scale 0.99x vs 100KiB <= 2x) |
| BenchmarkEditorSyntaxViewportCold/markdown/100KiB | 796 µs (not judged: CPU differs from the reference machine) | 929 KiB | 604.5 |  | — | — not judged |
| BenchmarkEditorSyntaxViewportCold/markdown/10MiB | 803 µs (not judged: CPU differs from the reference machine) | 929 KiB | 648 |  | — | ✅ within (scale 1.01x vs 100KiB <= 2x) |
| BenchmarkEditorSyntaxViewportWarm/go/100KiB | 1.18 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorSyntaxViewportWarm/go/10MiB | 1.26 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.06x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxViewportWarm/java/100KiB | 1.2 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorSyntaxViewportWarm/java/10MiB | 1.12 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.93x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxViewportWarm/markdown/100KiB | 1.14 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorSyntaxViewportWarm/markdown/10MiB | 1.13 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.00x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeInLine/go/100KiB | 7.75 µs (not judged: CPU differs from the reference machine) | 2.6 KiB | 9 |  | — | — not judged |
| BenchmarkEditorSyntaxTypeInLine/go/10MiB | 8 µs (not judged: CPU differs from the reference machine) | 3.14 KiB | 15 |  | — | ✅ within (scale 1.03x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeInLine/java/100KiB | 25 µs (not judged: CPU differs from the reference machine) | 3.17 KiB | 15 |  | — | — not judged |
| BenchmarkEditorSyntaxTypeInLine/java/10MiB | 26.5 µs (not judged: CPU differs from the reference machine) | 3.72 KiB | 21 |  | — | ✅ within (scale 1.06x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeInLine/markdown/100KiB | 15.7 µs (not judged: CPU differs from the reference machine) | 2.76 KiB | 10 |  | — | — not judged |
| BenchmarkEditorSyntaxTypeInLine/markdown/10MiB | 15.9 µs (not judged: CPU differs from the reference machine) | 3.3 KiB | 16 |  | — | ✅ within (scale 1.02x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeCatchUp/go/100KiB | 795 µs (not judged: CPU differs from the reference machine) | 33 KiB | 372 |  | — | — not judged |
| BenchmarkEditorSyntaxTypeCatchUp/go/10MiB | 731 µs (not judged: CPU differs from the reference machine) | 34.1 KiB | 374 |  | — | ✅ within (scale 0.92x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeCatchUp/java/100KiB | 653 µs (not judged: CPU differs from the reference machine) | 30.3 KiB | 319 |  | — | — not judged |
| BenchmarkEditorSyntaxTypeCatchUp/java/10MiB | 540 µs (not judged: CPU differs from the reference machine) | 13.3 KiB | 127 |  | — | ✅ within (scale 0.83x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxTypeCatchUp/markdown/100KiB | 923 µs (not judged: CPU differs from the reference machine) | 23.8 KiB | 344 |  | — | — not judged |
| BenchmarkEditorSyntaxTypeCatchUp/markdown/10MiB | 885 µs (not judged: CPU differs from the reference machine) | 25 KiB | 407 |  | — | ✅ within (scale 0.96x vs 100KiB <= 1.5x) |
| BenchmarkEditorSyntaxFullPass/go/100KiB | 57.4 ms (not judged: CPU differs from the reference machine) | 6.05 MiB | 29355 | MB/s: 1.785 | — | — not judged |
| BenchmarkEditorSyntaxFullPass/java/100KiB | 45.3 ms (not judged: CPU differs from the reference machine) | 4.99 MiB | 15416 | MB/s: 2.26 | — | — not judged |
| BenchmarkEditorSyntaxFullPass/markdown/100KiB | 25.2 ms (not judged: CPU differs from the reference machine) | 4.84 MiB | 16817 | MB/s: 4.07 | — | — not judged |
| BenchmarkEditorSyntaxFirstUse/go | 2.96 ms (not judged: CPU differs from the reference machine) | 15.2 MiB | 34780 |  | — | — not judged |
| BenchmarkEditorSyntaxFirstUse/java | 3.13 ms (not judged: CPU differs from the reference machine) | 30.6 MiB | 23090 |  | — | — not judged |
| BenchmarkEditorSyntaxFirstUse/markdown | 6.22 ms (not judged: CPU differs from the reference machine) | 35.3 MiB | 78390 |  | — | — not judged |
| BenchmarkEditorSyntaxSemanticBuild/go/100KiB | 41.2 µs (not judged: CPU differs from the reference machine) | 35.4 KiB | 157 |  | — | — not judged |
| BenchmarkEditorSyntaxSemanticBuild/go/10MiB | 5.67 ms (not judged: CPU differs from the reference machine) | 3.38 MiB | 13853 |  | — | — not judged |
| BenchmarkEditorSyntaxSemanticBuild/longline/100KiB | 87.6 µs (not judged: CPU differs from the reference machine) | 608 KiB | 16 |  | — | — not judged |
| BenchmarkEditorViewPaint/plain/100KiB | 45 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/plain/10MiB | 41.7 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.93x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/go/100KiB | 47.2 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/go/10MiB | 46.1 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.98x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/java/100KiB | 35.3 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/java/10MiB | 37.6 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.06x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/wrap/100KiB | 46.8 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/wrap/10MiB | 42.6 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.91x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/minimap/100KiB | 67.8 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/minimap/10MiB | 58.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.86x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/longline/100KiB | 1.58 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/longline/10MiB | 1.57 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.00x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/cjk/100KiB | 28.9 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/cjk/10MiB | 26.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.91x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/mixed/100KiB | 32.3 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/mixed/10MiB | 32.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 1.00x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/selection/100KiB | 47 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/selection/10MiB | 42.3 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.90x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/markers/100KiB | 54.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/markers/10MiB | 51.4 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.94x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaint/gutter/100KiB | 46.9 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPaint/gutter/10MiB | 46.5 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.99x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewType/go/100KiB | 51.1 µs (not judged: CPU differs from the reference machine) | 2.71 KiB | 7 |  | — | ✅ within |
| BenchmarkEditorViewType/go/10MiB | 50 µs (not judged: CPU differs from the reference machine) | 4.67 KiB | 31 |  | — | ✅ within (scale 0.98x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/longline/100KiB | 3.05 µs (not judged: CPU differs from the reference machine) | 1.74 KiB | 7 |  | — | ✅ within |
| BenchmarkEditorViewType/longline/10MiB | 3.62 µs (not judged: CPU differs from the reference machine) | 2.32 KiB | 13 |  | — | ✅ within (scale 1.19x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/longline-wrap/100KiB | 43.3 µs (not judged: CPU differs from the reference machine) | 2.07 KiB | 9 |  | — | ✅ within |
| BenchmarkEditorViewType/longline-wrap/10MiB | 85.8 µs (not judged: CPU differs from the reference machine) | 2.8 KiB | 16 |  | — | ✅ within (scale 1.98x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/minimap/100KiB | 406 µs (not judged: CPU differs from the reference machine) | 2.71 KiB | 7 |  | — | ✅ within |
| BenchmarkEditorViewType/minimap/10MiB | 369 µs (not judged: CPU differs from the reference machine) | 5.1 KiB | 31 |  | — | ✅ within (scale 0.91x vs 100KiB <= 2x) |
| BenchmarkEditorViewType/longline-wrap-minimap/100KiB | 282 µs (not judged: CPU differs from the reference machine) | 3.13 KiB | 21 |  | — | ✅ within |
| BenchmarkEditorViewType/longline-wrap-minimap/10MiB | 324 µs (not judged: CPU differs from the reference machine) | 4.17 KiB | 34 |  | — | ✅ within (scale 1.15x vs 100KiB <= 2x) |
| BenchmarkEditorViewEnter/go/100KiB | 44.9 µs (not judged: CPU differs from the reference machine) | 2.68 KiB | 7 |  | — | ✅ within |
| BenchmarkEditorViewEnter/go/10MiB | 39.2 µs (not judged: CPU differs from the reference machine) | 3.25 KiB | 13 |  | — | ✅ within (scale 0.87x vs 100KiB <= 2x) |
| BenchmarkEditorViewUndoRedo/go/100KiB | 55.9 µs (not judged: CPU differs from the reference machine) | 91.5 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewUndoRedo/go/10MiB | 50.7 µs (not judged: CPU differs from the reference machine) | 1.45 KiB | 18 |  | — | ✅ within (scale 0.91x vs 100KiB <= 2x) |
| BenchmarkEditorViewCaretMove/go/100KiB | 50.6 µs (not judged: CPU differs from the reference machine) | 32 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewCaretMove/go/10MiB | 36 µs (not judged: CPU differs from the reference machine) | 32 B | 0 |  | — | ✅ within (scale 0.71x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPageMove/go/100KiB | 41.1 µs (not judged: CPU differs from the reference machine) | 41 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewPageMove/go/10MiB | 39.8 µs (not judged: CPU differs from the reference machine) | 46.5 B | 0 |  | — | ✅ within (scale 0.97x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewScroll/go/100KiB | 48.6 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewScroll/go/10MiB | 44 µs (not judged: CPU differs from the reference machine) | 0 B | 0 |  | — | ✅ within (scale 0.91x vs 100KiB <= 1.5x) |
| BenchmarkEditorViewPaste/go/100KiB | 3.67 ms (not judged: CPU differs from the reference machine) | 1.18 MiB | 1970 |  | — | — not judged |
| BenchmarkEditorViewPaste/go/10MiB | 3.8 ms (not judged: CPU differs from the reference machine) | 1.19 MiB | 2085 |  | — | ✅ within (scale 1.03x vs 100KiB <= 2x) |
| BenchmarkEditorViewOpen/go/100KiB | 1.2 ms (not judged: CPU differs from the reference machine) | 170 KiB | 1186 |  | — | — not judged |
| BenchmarkEditorViewOpen/go/10MiB | 1.12 ms (not judged: CPU differs from the reference machine) | 170 KiB | 1186 |  | — | ✅ within (scale 0.93x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/java/100KiB | 1.43 ms (not judged: CPU differs from the reference machine) | 194 KiB | 1133 |  | — | — not judged |
| BenchmarkEditorViewOpen/java/10MiB | 1.39 ms (not judged: CPU differs from the reference machine) | 194 KiB | 1133 |  | — | ✅ within (scale 0.97x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/json/100KiB | 82.8 µs (not judged: CPU differs from the reference machine) | 122 KiB | 517 |  | — | — not judged |
| BenchmarkEditorViewOpen/json/10MiB | 81.1 µs (not judged: CPU differs from the reference machine) | 122 KiB | 517 |  | — | ✅ within (scale 0.98x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/markdown/100KiB | 1.56 ms (not judged: CPU differs from the reference machine) | 235 KiB | 1195 |  | — | — not judged |
| BenchmarkEditorViewOpen/markdown/10MiB | 1.62 ms (not judged: CPU differs from the reference machine) | 234 KiB | 1195 |  | — | ✅ within (scale 1.04x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/log/100KiB | 121 µs (not judged: CPU differs from the reference machine) | 88.6 KiB | 501 |  | — | — not judged |
| BenchmarkEditorViewOpen/log/10MiB | 124 µs (not judged: CPU differs from the reference machine) | 88.7 KiB | 501 |  | — | ✅ within (scale 1.03x vs 100KiB <= 4x) |
| BenchmarkEditorViewOpen/mixed/100KiB | 106 µs (not judged: CPU differs from the reference machine) | 90.8 KiB | 506 |  | — | — not judged |
| BenchmarkEditorViewOpen/mixed/10MiB | 111 µs (not judged: CPU differs from the reference machine) | 90.9 KiB | 506 |  | — | ✅ within (scale 1.04x vs 100KiB <= 4x) |
| BenchmarkEditorViewPrepareReload/go/100KiB | 123 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 833.3 | — | — not judged |
| BenchmarkEditorViewPrepareReload/go/10MiB | 9.73 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5896 | MB/s: 1078 | — | — not judged |
| BenchmarkEditorViewPrepareReload/java/100KiB | 113 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 903.7 | — | — not judged |
| BenchmarkEditorViewPrepareReload/java/10MiB | 9.03 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5893 | MB/s: 1162 | — | — not judged |
| BenchmarkEditorViewPrepareReload/json/100KiB | 144 µs (not judged: CPU differs from the reference machine) | 109 KiB | 68 | MB/s: 713.3 | — | — not judged |
| BenchmarkEditorViewPrepareReload/json/10MiB | 15.1 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5829 | MB/s: 694 | — | — not judged |
| BenchmarkEditorViewPrepareReload/markdown/100KiB | 86.5 µs (not judged: CPU differs from the reference machine) | 109 KiB | 69 | MB/s: 1186 | — | — not judged |
| BenchmarkEditorViewPrepareReload/markdown/10MiB | 6.57 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5982 | MB/s: 1598 | — | — not judged |
| BenchmarkEditorViewPrepareReload/log/100KiB | 42 µs (not judged: CPU differs from the reference machine) | 109 KiB | 69 | MB/s: 2440 | — | — not judged |
| BenchmarkEditorViewPrepareReload/log/10MiB | 4.01 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5996 | MB/s: 2617 | — | — not judged |
| BenchmarkEditorViewPrepareReload/mixed/100KiB | 70.4 µs (not judged: CPU differs from the reference machine) | 109 KiB | 69 | MB/s: 1455 | — | — not judged |
| BenchmarkEditorViewPrepareReload/mixed/10MiB | 6.23 ms (not judged: CPU differs from the reference machine) | 10.5 MiB | 5994 | MB/s: 1682 | — | — not judged |
| BenchmarkEditorViewRewrap/go/100KiB | 98.6 µs (not judged: CPU differs from the reference machine) | 256 B | 15 |  | — | — not judged |
| BenchmarkEditorViewRewrap/go/10MiB | 6.81 ms (not judged: CPU differs from the reference machine) | 152 B | 9 |  | — | — not judged |
| BenchmarkEditorViewMemory/go/100KiB | 3.84 ms | 286 KiB | 1293 | retained-B: 65 KiB, retained/text-byte: 0.6503 | — | ✅ within |
| BenchmarkEditorViewMemory/go/10MiB | 13.2 ms | 10.7 MiB | 7121 | retained-B: 450 KiB, retained/text-byte: 0.04395 | — | ✅ within |
| BenchmarkEditorViewMemory/json/100KiB | 2.59 ms | 238 KiB | 624 | retained-B: 63.2 KiB, retained/text-byte: 0.6313 | — | ✅ within |
| BenchmarkEditorViewMemory/json/10MiB | 17.3 ms | 10.7 MiB | 6385 | retained-B: 444 KiB, retained/text-byte: 0.04334 | — | ✅ within |
| BenchmarkEditorViewMemory/markdown/100KiB | 3.91 ms | 387 KiB | 1310 | retained-B: 105 KiB, retained/text-byte: 1.053 | — | ✅ within |
| BenchmarkEditorViewMemory/markdown/10MiB | 10.5 ms | 10.8 MiB | 7223 | retained-B: 496 KiB, retained/text-byte: 0.04844 | — | ✅ within |
| BenchmarkEditorViewMemory/mixed/100KiB | 2.66 ms | 207 KiB | 614 | retained-B: 42.5 KiB, retained/text-byte: 0.425 | — | ✅ within |
| BenchmarkEditorViewMemory/mixed/10MiB | 9.02 ms | 10.6 MiB | 6539 | retained-B: 434 KiB, retained/text-byte: 0.04237 | — | ✅ within |
| BenchmarkEditorViewReload/go/100KiB | 36.5 ns (not judged: CPU differs from the reference machine) | 183 B | 0 |  | — | ✅ within |
| BenchmarkEditorViewReload/go/10MiB | 67.8 ns (not judged: CPU differs from the reference machine) | 183 B | 0 |  | — | ✅ within (scale 1.86x vs 100KiB <= 2x) |

This report never fails the build; overruns are findings to fix or to justify in the PR.

