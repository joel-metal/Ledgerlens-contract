# Constant Rationale Index

This index lists every numeric constant in the repository that carries a co-located rationale comment at its declaration. Each entry links to the fuller source document that explains *why* the value was chosen (not just what it is), so that a future change to the number can be checked against the original reasoning.

The index is referenced from the annotated source files (e.g. [`contracts/ledgerlens-score/src/constants.rs`](../contracts/ledgerlens-score/src/constants.rs)) and from [`CONTRIBUTING.md`](../CONTRIBUTING.md). Keep it current: when you add, remove, or re-annotate a constant, update this table in the same pull request.

## Annotated constants

| Constant | File | Current value | Rationale summary | Fuller source | Status |
| --- | --- | --- | --- | --- | --- |
| `MAX_SCORE` | `contracts/ledgerlens-score/src/constants.rs` | `100` | Scores are normalized to a 0–100 scale so they are human-readable and comparable across issuers; see the safe-defaults doc for the normalization contract. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |
| `MIN_SCORE` | `contracts/ledgerlens-score/src/constants.rs` | `0` | Lower bound of the normalized score scale; a score of 0 is a valid (worst) outcome, not an error sentinel. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |
| `DEFAULT_SCORE` | `contracts/ledgerlens-score/src/constants.rs` | `50` | Neutral midpoint used when insufficient history exists, so an unknown subject is neither rewarded nor penalized. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |
| `SCORE_DECIMALS` | `contracts/ledgerlens-score/src/constants.rs` | `0` | Scores are integer-valued on-chain to avoid fixed-point rounding disputes between off-chain and on-chain consumers. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |
| `MAX_HISTORY_ENTRIES` | `contracts/ledgerlens-score/src/constants.rs` | `100` | Caps per-subject history to bound storage growth and keep score recomputation within the ledger's instruction budget. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |
| `SCORE_WINDOW_LEDGERS` | `contracts/ledgerlens-score/src/constants.rs` | `17280` | One day of ledgers at ~5s close time; the scoring window is defined in whole days so the rationale is stable across ledger-time drift. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |
| `MIN_HISTORY_ENTRIES` | `contracts/ledgerlens-score/src/constants.rs` | `1` | At least one observation is required before a computed score replaces the neutral default. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |
| `BPS_DENOMINATOR` | `contracts/ledgerlens-score/src/constants.rs` | `10000` | Basis points (1/100 of a percent) are the project-wide unit for ratios, matching the rest of the contracts. | [`docs/configuration-safe-defaults.md`](./configuration-safe-defaults.md) | Documented |

> If a constant you are annotating is not yet listed above, add a row. If you cannot find a rationale anywhere in the repository, add the row with status **Undocumented — needs review** and open a follow-up (see below) rather than leaving it unexplained.

## Review convention for changing an annotated constant

Any pull request that changes the **value** of a constant listed in this index must also:

1. **Re-read the linked fuller source** for that constant and confirm the stated rationale still holds for the new value.
2. **Update the rationale comment** at the declaration if the reasoning has changed, and update the "Rationale summary" and "Current value" columns in this table.
3. **Update the fuller source document** if the new value changes the documented contract, defaults, or guarantees.
4. **State in the PR description** which annotated constant(s) changed and why the linked rationale remains valid (or how it was updated).

A change that only edits the number without addressing the rationale is considered incomplete and should be requested for changes in review. This convention is also documented in [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Follow-ups for undocumented constants

Constants whose rationale could not be located in the existing documentation are tracked here. Each must either be resolved with a reconstructed rationale (and moved to **Documented** above) or kept as a clearly scoped follow-up.

- _None currently open._ When an undocumented constant is found, add a bullet here describing the constant, the file, and the scope of the follow-up (reconstruct rationale vs. confirm historical intent), and reference the tracking issue.
</｜｜DSML｜｜ parameter>
</｜｜DSML｜｜ invoke>
</｜｜DSML｜｜ calls>