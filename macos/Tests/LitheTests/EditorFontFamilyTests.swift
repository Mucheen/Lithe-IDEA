import AppKit
import SwiftUI
import Testing
@testable import Lithe

/// Covers the editor programming-font setting: normalization and fallback of the
/// stored family, the read-only macOS font catalog that backs the picker, the
/// preservation of the bundled default, and the searchable selector's filter.
///
/// Nothing here depends on a specific third-party font being installed: the
/// bundled family is asserted through the catalog's own injection, and the
/// monospaced filter is verified generically over whatever macOS ships.
@MainActor
@Suite("Editor programming font family", .serialized)
struct EditorFontFamilyTests {
    // MARK: - Stored value policy

    @Test func storedFamilyNormalizesMissingAndEmptyValuesToBundledDefault() {
        #expect(EditorFontResolution.normalizedFamily(nil) == "JetBrains Mono")
        #expect(EditorFontResolution.normalizedFamily("") == "JetBrains Mono")
        #expect(EditorFontResolution.normalizedFamily("   ") == "JetBrains Mono")
        #expect(EditorFontResolution.normalizedFamily("\n\t ") == "JetBrains Mono")
    }

    @Test func storedFamilyKeepsUserSpellingWithoutSurroundingWhitespace() {
        #expect(EditorFontResolution.normalizedFamily("  Fira Code  ") == "Fira Code")
        #expect(EditorFontResolution.normalizedFamily("Menlo") == "Menlo")
    }

    @Test func onlyTheBundledFamilyUsesTheBundledFaceMapping() {
        #expect(EditorFontResolution.usesBundledMonospacedFamily("JetBrains Mono"))
        #expect(EditorFontResolution.usesBundledMonospacedFamily("  JetBrains Mono  "))
        #expect(!EditorFontResolution.usesBundledMonospacedFamily("Menlo"))
        // Reinstalling the bundled family under different casing is still the
        // bundled family as far as the settings model is concerned.
        #expect(!EditorFontResolution.usesBundledMonospacedFamily("jetbrains mono"))
    }

    // MARK: - Catalog resolution

    @Test func bundledFamilyIsAlwaysOfferedAndResolvable() {
        // The bundled faces ship inside the app bundle and are registered for the
        // process, so the catalog must offer them even on a machine that never
        // installed JetBrains Mono system-wide.
        #expect(MacEditorFontCatalog.isAvailable(family: "JetBrains Mono"))
        #expect(MacEditorFontCatalog.resolvedFamily("JetBrains Mono") == "JetBrains Mono")
        #expect(MacEditorFontCatalog.editorFamilies().first == "JetBrains Mono")
    }

    @Test func unknownFamilyFallsBackToBundledFamilyInsteadOfFailing() throws {
        let missing = "Lithe Missing Monospace Family"

        #expect(!MacEditorFontCatalog.isAvailable(family: missing))
        #expect(MacEditorFontCatalog.resolvedFamily(missing) == "JetBrains Mono")
        // An uninstalled preference must still produce a usable editor font.
        let font = MacEditorFontCatalog.font(family: missing, size: 13)
        #expect(font.fontName == LitheTheme.editorFont(size: 13).fontName)
    }

    @Test func catalogOffersBundledFamilyFirstWithoutDuplicates() {
        let families = MacEditorFontCatalog.editorFamilies()
        #expect(families.count > 1, "macOS always ships at least one monospaced system family")

        #expect(families.first == "JetBrains Mono")
        let keys = families.map { $0.lowercased() }
        #expect(Set(keys).count == keys.count, "a family must not be listed twice: \(families)")
    }

    @Test func catalogExcludesProportionalFamilies() {
        // Helvetica ships with every macOS version. It is installed, so a stored
        // preference for it must not be presented as missing, but it is
        // proportional, so the editor picker must not offer it.
        #expect(MacEditorFontCatalog.isAvailable(family: "Helvetica"))
        #expect(!MacEditorFontCatalog.editorFamilies().contains("Helvetica"))
    }

    /// Independent verification of the filter: measure the font the product will
    /// actually render with rather than trusting the catalog's own answer, so a
    /// regression in detection cannot pass by agreeing with itself.
    @Test func everyOfferedFamilyRendersFixedPitch() {
        let families = MacEditorFontCatalog.editorFamilies().filter { $0 != "JetBrains Mono" }
        #expect(!families.isEmpty, "expected at least one installed monospaced family")
        guard !families.isEmpty else { return }

        for family in families {
            let font = MacEditorFontCatalog.font(family: family, size: 12)
            let advances = ["i", "W", "0"].map {
                ($0 as NSString).size(withAttributes: [.font: font]).width
            }
            let first = advances[0]
            #expect(
                advances.allSatisfy { abs($0 - first) < 0.01 },
                "\(family) is offered as monospaced but its advances differ: \(advances)"
            )
        }
    }

    @Test func selectingAnInstalledFamilyProducesThatFamily() throws {
        let families = MacEditorFontCatalog.editorFamilies()
        let userFamily = try #require(
            families.first { $0 != "JetBrains Mono" },
            "expected at least one selectable system family"
        )

        #expect(MacEditorFontCatalog.isAvailable(family: userFamily))
        let font = MacEditorFontCatalog.font(family: userFamily, size: 13)
        #expect(
            font.familyName?.caseInsensitiveCompare(userFamily) == .orderedSame,
            "selected \(userFamily) but resolved \(font.familyName ?? "nil")"
        )
    }

    @Test func everyBundledWeightStillDelegatesToTheBundledFaceMapping() {
        for weight in [NSFont.Weight.regular, .medium, .semibold, .bold] {
            #expect(
                MacEditorFontCatalog.font(family: "JetBrains Mono", size: 13, weight: weight).fontName
                    == LitheTheme.editorFont(size: 13, weight: weight).fontName
            )
        }
    }

    /// The embedded editor cannot see CoreText's process-local registration, so it
    /// loads the bundled faces through an `@font-face` rule keyed by family name.
    /// If the default family and that CSS family drift apart, selecting the
    /// default would silently fall back to the system monospace font in Monaco.
    @Test func bundledDefaultFamilyMatchesTheEditorWebFontFace() throws {
        let html = URL(fileURLWithPath: #filePath)
            .deletingLastPathComponent()
            .deletingLastPathComponent()
            .deletingLastPathComponent()
            .appendingPathComponent("EditorFrontend/index.html")
        let source = try String(contentsOf: html, encoding: .utf8)

        #expect(
            source.contains("@font-face{font-family:\"\(EditorFontDefaults.monospacedFamily)\""),
            "index.html must declare the bundled editor family \(EditorFontDefaults.monospacedFamily)"
        )
    }

    // MARK: - Settings persistence

    @Test func settingsPersistAndRestoreTheEditorFontFamily() throws {
        let store = EditorFontSettingsStore()
        let settings = AppSettings(store: store)

        // An installation that never touched the setting keeps the bundled family,
        // which is what makes this feature opt-in.
        #expect(settings.editorFontFamily == "JetBrains Mono")

        settings.editorFontFamily = "Menlo"
        #expect(AppSettings(store: store).editorFontFamily == "Menlo")

        settings.restoreDefaults()
        #expect(settings.editorFontFamily == "JetBrains Mono")
        #expect(AppSettings(store: store).editorFontFamily == "JetBrains Mono")
    }

    @Test func settingsTreatAClearedStoredFamilyAsTheBundledDefault() {
        let store = EditorFontSettingsStore()
        store.set("", forKey: "settings.editorFontFamily")
        #expect(AppSettings(store: store).editorFontFamily == "JetBrains Mono")

        store.set("   ", forKey: "settings.editorFontFamily")
        #expect(AppSettings(store: store).editorFontFamily == "JetBrains Mono")
    }

    // MARK: - Searchable selector filter

    @Test func emptyQueryMatchesEveryCandidate() {
        #expect(LitheSettingsSelectSearch.matches("JetBrains Mono", query: ""))
        #expect(LitheSettingsSelectSearch.matches("JetBrains Mono", query: "   "))
        #expect(LitheSettingsSelectSearch.matches("JetBrains Mono", query: "\n"))
    }

    @Test func searchIgnoresCaseAndMatchesSubstrings() {
        #expect(LitheSettingsSelectSearch.matches("JetBrains Mono", query: "mono"))
        #expect(LitheSettingsSelectSearch.matches("JetBrains Mono", query: "MONO"))
        #expect(LitheSettingsSelectSearch.matches("JetBrains Mono", query: "jetbrains"))
        #expect(LitheSettingsSelectSearch.matches("Fira Code", query: "  fira  "))
        #expect(!LitheSettingsSelectSearch.matches("Fira Code", query: "menlo"))
    }

    @Test func searchMatchesNonLatinFamilyNames() {
        #expect(LitheSettingsSelectSearch.matches("等宽测试字体", query: "等宽"))
        #expect(!LitheSettingsSelectSearch.matches("等宽测试字体", query: "比例"))
    }
}

/// Isolated defaults store so the persistence test never reads or writes the
/// developer's real settings domain.
private final class EditorFontSettingsStore: KeyValueStore, @unchecked Sendable {
    private var values: [String: Any] = [:]

    func data(forKey key: String) -> Data? { values[key] as? Data }
    func object(forKey key: String) -> Any? { values[key] }
    func string(forKey key: String) -> String? { values[key] as? String }
    func stringArray(forKey key: String) -> [String]? { values[key] as? [String] }
    func set(_ value: Any?, forKey key: String) { values[key] = value }
}
