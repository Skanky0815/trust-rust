# Release & Versioning Guide

Dieses Projekt nutzt **Semantic Versioning** mit **Conventional Commits** für automatische Releases.

## Versionierungsschema

Die Version folgt dem Schema: `MAJOR.MINOR.PATCH`

### Automatische Versionsberechnung

Die Version wird basierend auf den Commit-Messages automatisch berechnet:

| Commit-Typ | Version | Beispiel |
|-----------|---------|----------|
| `fix:` | PATCH | `fix: patient validation error` → `1.0.1` |
| `feat:` | MINOR | `feat: add patient search` → `1.1.0` |
| `BREAKING CHANGE:` | MAJOR | `feat!: new database schema` → `2.0.0` |

## Commit-Konventionen

Verwende folgende Commit-Message-Format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Typen

- **feat:** Neue Funktionalität
- **fix:** Fehlerbehebung
- **refactor:** Code-Umstrukturierung ohne Funktionsänderung
- **test:** Test-Hinzufügung oder Änderung
- **docs:** Dokumentationsänderung
- **chore:** Build-Prozess, Dependencies, etc.
- **ci:** CI/CD-Konfigurationsänderungen

### Beispiele

#### Patch-Release (Fix)
```
fix: handle empty patient names correctly
```

#### Minor-Release (Feature)
```
feat(patient): add search endpoint

- Implemented full-text search
- Added pagination support
```

#### Major-Release (Breaking Change)
```
feat!: restructure patient event payload

BREAKING CHANGE: The `new_patient` event now requires a `trace_id` field.
```

## Workflow

### 1. **Pull Request**
   - Erstelle einen PR gegen `main`
   - GitHub Action führt `semantic-release --dry-run` aus
   - Kommentar zeigt die nächste geplante Version

### 2. **Code Review & Merge**
   - Code wird reviewed
   - PR wird gemergt in `main`

### 3. **Automatisches Release**
   - GitHub Action erkennt Merge
   - Berechnet neue Version aus Commits
   - Updated `Cargo.toml` mit neuer Version
   - Erstellt Git-Tag (z.B. `v1.1.0`)
   - Erstellt GitHub Release mit Changelog

## Beispiel PR mit Dry-Run

```
PR: Add patient deletion endpoint

feat(patient): add delete endpoint

This enables patients to be deleted from the system.

---

semantic-release Dry Run Result:
📦 Next Release: 1.1.0 (Minor)

Changes:
✨ New Features
- feat(patient): add delete endpoint

Updated: Cargo.toml, CHANGELOG.md
```

## Tags und Releases

Releases werden mit Git-Tags gekennzeichnet:
- Format: `v<version>` (z.B. `v1.2.0`)
- GitHub Releases werden automatisch erstellt
- Changelog wird automatisch generiert

## Git-Hooks (Optional)

Um Commit-Konventionen lokal zu erzwingen, kann ein Pre-Commit-Hook verwendet werden:

```bash
npm install -D husky commitlint @commitlint/config-conventional
npx husky install
npx commitlint --install
```
