use std::fs;
use std::path::Path;

#[test]
fn test_docs_exist_and_are_non_empty() {
    let docs = [
        "docs/security/contributor-review-checklists.md",
        "docs/operations/runbook.md",
    ];

    for doc_path in &docs {
        let path = Path::new(doc_path);
        assert!(
            path.exists(),
            "Documentation file missing: {}",
            doc_path
        );
        let metadata = fs::metadata(path).expect("Failed to read doc metadata");
        assert!(
            metadata.len() > 0,
            "Documentation file is empty: {}",
            doc_path
        );
    }
}

// ── Drift checks: documentation content vs. contract source ──────────────────
//
// The tests above only verify that docs exist and have bytes. These checks go
// further: they parse a handful of version numbers that `docs/interface-spec.md`
// claims about the contract and verify each one matches the actual source. The
// interface spec is the canonical, versioned integration contract for third-party
// callers, so a spec that drifts from the source silently breaks integrators.

/// Extracts `N` from `pub const CONTRACT_VERSION: u32 = N;` in `constants.rs`.
fn extract_contract_version(constants_src: &str) -> String {
    let marker = "CONTRACT_VERSION: u32 = ";
    constants_src
        .lines()
        .find_map(|line| {
            let idx = line.find(marker)?;
            Some(
                line[idx + marker.len()..]
                    .trim()
                    .trim_end_matches(';')
                    .trim()
                    .to_string(),
            )
        })
        .expect("CONTRACT_VERSION not found in constants.rs")
}

/// Extracts `N` from `interface_version: N,` inside `get_interface_metadata` in
/// `lib.rs`.
fn extract_interface_version(lib_src: &str) -> String {
    let marker = "interface_version: ";
    lib_src
        .lines()
        .find_map(|line| {
            let idx = line.find(marker)?;
            Some(
                line[idx + marker.len()..]
                    .trim()
                    .trim_end_matches(',')
                    .trim()
                    .to_string(),
            )
        })
        .expect("interface_version not found in lib.rs")
}

/// Collects the value of every `` currently `N` `` claim in a doc. `interface-spec.md`
/// uses this exact phrasing for "the contract version as of the current build".
fn extract_current_version_claims(doc_src: &str) -> Vec<&str> {
    let marker = "currently `";
    doc_src
        .lines()
        .filter_map(|line| {
            let idx = line.find(marker)?;
            let rest = &line[idx + marker.len()..];
            rest.splitn(2, '`').next()?.trim()
        })
        .collect()
}

/// Extracts `N` from the `**Interface version:** N` header of a spec doc.
fn extract_documented_interface_version(doc_src: &str) -> &str {
    let marker = "Interface version:** ";
    doc_src
        .lines()
        .find_map(|line| {
            let idx = line.find(marker)?;
            line[idx + marker.len()..].split_whitespace().next()
        })
        .expect("'Interface version:** N' header not found in interface-spec.md")
}

#[test]
fn test_interface_spec_contract_version_matches_source() {
    let constants_src =
        fs::read_to_string("contracts/ledgerlens-score/src/constants.rs")
            .expect("failed to read constants.rs");
    let code_version = extract_contract_version(&constants_src);

    let spec_src =
        fs::read_to_string("docs/interface-spec.md").expect("failed to read interface-spec.md");

    let doc_claims = extract_current_version_claims(&spec_src);
    assert!(
        !doc_claims.is_empty(),
        "docs/interface-spec.md should state the current contract version (e.g. `currently `N``)"
    );

    for claim in doc_claims {
        assert_eq!(
            claim,
            code_version,
            "docs/interface-spec.md claims the contract version is `{claim}`, but \
             CONTRACT_VERSION in constants.rs is `{code_version}` — reconcile the doc"
        );
    }
}

#[test]
fn test_interface_spec_interface_version_matches_source() {
    let lib_src =
        fs::read_to_string("contracts/ledgerlens-score/src/lib.rs").expect("failed to read lib.rs");
    let code_version = extract_interface_version(&lib_src);

    let spec_src =
        fs::read_to_string("docs/interface-spec.md").expect("failed to read interface-spec.md");
    let doc_version = extract_documented_interface_version(&spec_src);

    assert_eq!(
        code_version,
        doc_version,
        "docs/interface-spec.md header claims interface version `{doc_version}`, but \
         get_interface_metadata publishes `{code_version}` — reconcile the doc"
    );
}

// ── Constant rationale co-location checks ────────────────────────────────────
//
// Numeric constants are often chosen for a specific documented reason, but that
// reasoning can live only in a design doc or PR description, disconnected from
// the declaration. These checks enforce the convention documented in
// `docs/constant-rationale-index.md` and the contribution guide: every constant
// listed in the index must exist in `constants.rs`, and its declaration must be
// preceded by a rationale comment that either links to a `docs/` file or
// explicitly flags the rationale as undocumented / needing review.

const RATIONALE_INDEX_PATH: &str = "docs/constant-rationale-index.md";
const CONSTANTS_SRC_PATH: &str = "contracts/ledgerlens-score/src/constants.rs";

/// Extracts the constant names listed in the rationale index. Names are expected
/// to appear as `` `NAME` `` inside a markdown table row or list item.
fn extract_indexed_constant_names(index_src: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in index_src.lines() {
        let trimmed = line.trim_start();
        // Only consider table rows and list items so prose mentions don't count.
        if !(trimmed.starts_with('|') || trimmed.starts_with('-') || trimmed.starts_with('*')) {
            continue;
        }
        // Pull the first backtick-delimited identifier on the line.
        if let Some(start) = line.find('`') {
            let rest = &line[start + 1..];
            if let Some(end) = rest.find('`') {
                let candidate = rest[..end].trim();
                if !candidate.is_empty()
                    && candidate
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                {
                    names.push(candidate.to_string());
                }
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

/// Returns the 1-based line number of the `pub const NAME` declaration, if any.
fn find_const_decl_line(constants_src: &str, name: &str) -> Option<usize> {
    let needle = format!("pub const {name}");
    constants_src
        .lines()
        .position(|line| line.trim_start().starts_with(&needle))
        .map(|idx| idx + 1)
}

/// Returns the contiguous block of comment lines immediately preceding the
/// declaration on `decl_line` (1-based), walking upward over `//` comments.
fn preceding_comment_block(constants_src: &str, decl_line: usize) -> String {
    let lines: Vec<&str> = constants_src.lines().collect();
    let mut collected: Vec<&str> = Vec::new();
    let mut idx = decl_line.saturating_sub(1); // convert to 0-based, start above decl
    while idx > 0 {
        idx -= 1;
        let line = lines[idx].trim_start();
        if line.starts_with("//") {
            collected.push(line);
        } else {
            break;
        }
    }
    collected.reverse();
    collected.join("\n")
}

/// True if the comment block links to a `docs/` file or explicitly flags the
/// rationale as undocumented / needing review.
fn comment_has_rationale(comment: &str) -> bool {
    let lower = comment.to_ascii_lowercase();
    let links_doc = comment.contains("docs/");
    let flagged = lower.contains("undocumented")
        || lower.contains("needs review")
        || lower.contains("needs-review");
    links_doc || flagged
}

#[test]
fn test_constant_rationale_index_exists() {
    let path = Path::new(RATIONALE_INDEX_PATH);
    assert!(
        path.exists(),
        "Rationale index missing: {RATIONALE_INDEX_PATH} — every annotated constant \
         must be listed here"
    );
    let metadata = fs::metadata(path).expect("Failed to read rationale index metadata");
    assert!(
        metadata.len() > 0,
        "Rationale index is empty: {RATIONALE_INDEX_PATH}"
    );
}

#[test]
fn test_indexed_constants_exist_in_source() {
    let index_src =
        fs::read_to_string(RATIONALE_INDEX_PATH).expect("failed to read constant-rationale-index.md");
    let constants_src = fs::read_to_string(CONSTANTS_SRC_PATH).expect("failed to read constants.rs");

    let names = extract_indexed_constant_names(&index_src);
    assert!(
        !names.is_empty(),
        "{RATIONALE_INDEX_PATH} should list at least one annotated constant"
    );

    for name in &names {
        assert!(
            find_const_decl_line(&constants_src, name).is_some(),
            "Rationale index lists `{name}`, but no `pub const {name}` declaration \
             exists in {CONSTANTS_SRC_PATH} — the index has drifted from the source"
        );
    }
}

#[test]
fn test_indexed_constants_have_colocated_rationale() {
    let index_src =
        fs::read_to_string(RATIONALE_INDEX_PATH).expect("failed to read constant-rationale-index.md");
    let constants_src = fs::read_to_string(CONSTANTS_SRC_PATH).expect("failed to read constants.rs");

    let names = extract_indexed_constant_names(&index_src);
    assert!(
        !names.is_empty(),
        "{RATIONALE_INDEX_PATH} should list at least one annotated constant"
    );

    for name in &names {
        let decl_line = find_const_decl_line(&constants_src, name).unwrap_or_else(|| {
            panic!(
                "Rationale index lists `{name}`, but no `pub const {name}` declaration \
                 exists in {CONSTANTS_SRC_PATH}"
            )
        });

        let comment = preceding_comment_block(&constants_src, decl_line);
        assert!(
            !comment.is_empty(),
            "`pub const {name}` (line {decl_line}) has no preceding rationale comment. \
             Add a comment linking to its rationale doc (see {RATIONALE_INDEX_PATH}) or \
             flag it as undocumented / needs review"
        );
        assert!(
            comment_has_rationale(&comment),
            "`pub const {name}` (line {decl_line}) has a preceding comment but it neither \
             links to a `docs/` file nor flags the rationale as undocumented / needs review. \
             See {RATIONALE_INDEX_PATH} for the convention"
        );
    }
}