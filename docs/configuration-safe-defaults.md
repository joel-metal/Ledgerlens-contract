# Safe defaults and forbidden configurations

This document maps production-relevant parameters to the threat they control,
their default, their bounds, and the operational tradeoff.

> **Where the rationale lives.** The *reason* each bound and default was chosen
> is recorded next to the constant's declaration in code (see
> `contracts/ledgerlens-score/src/constants.rs`) and summarized in the
> [constant rationale index](constant-rationale-index.md). The tables below
> state the values and their operational tradeoff; when you change a value,
> update the co-located rationale comment and the index entry as well (see the
> review convention in `CONTRIBUTING.md`).

The baseline assumption is fail-closed risk gating: when data is missing,
stale beyond the configured model, embargoed, or held in pending-finality
state, downstream protocols should assume the wallet is not safe.

## On-chain parameter bounds registry

All governed parameters are validated against a single on-chain bounds
registry (`parameter_governance.rs`). Every setter routes through one shared
validation function, and the upgrade smoke checks consult the same table, so a
value that is individually valid but unsafe in combination cannot be landed by
governance or by an upgrade. The registry is exposed read-only via
`get_parameter_bounds()` for tooling and the production deployment validator,
and the table itself can only be changed by upgrade, never by runtime
governance.

### Hard bounds table

| Parameter | Min | Max | Cross-parameter constraint |
|---|---:|---:|---|
| Parameter | Min | Max | Cross-parameter constraint | Rationale |
|---|---:|---:|---|---|
| `risk_threshold` | `0` | `100` | `hysteresis_margin < risk_threshold` | [index](constant-rationale-index.md#risk_threshold) |
| `global_min_confidence` | `0` | `100` | — | [index](constant-rationale-index.md#global_min_confidence) |
| `staleness_window` | `1s` | `2_592_000s` | `staleness_window > cooldown` | [index](constant-rationale-index.md#staleness_window) |
| `finality_buffer` | `0s` | `86_400s` | — | [index](constant-rationale-index.md#finality_buffer) |
| `cooldown` | `60s` | `86_400s` | `cooldown < staleness_window` | [index](constant-rationale-index.md#cooldown) |
| `adaptive_rate_limit` | `variance_scale >= 0` | unbounded | — | [index](constant-rationale-index.md#adaptive_rate_limit) |
| `burst_capacity` | `1` | `1_000` | — | [index](constant-rationale-index.md#burst_capacity) |
| `score_floor_policy` | `high_water_mark ∈ [50,100]` | — | `floor_value < high_water_mark` | [index](constant-rationale-index.md#score_floor_policy) |
| `hysteresis_margin` | `0` | `100` | `hysteresis_margin < risk_threshold` | [index](constant-rationale-index.md#hysteresis_margin) |
| `escalation_threshold` | `2` | `20` | — | [index](constant-rationale-index.md#escalation_threshold) |
| `consensus_config (k, epsilon)` | `k >= 1` | `epsilon <= 100` | `k <= signer_count` | [index](constant-rationale-index.md#consensus_config) |
| `adaptive_epsilon` | `min >= 0` | `max <= 100` | `min <= max` | [index](constant-rationale-index.md#adaptive_epsilon) |
| `privacy_epsilon` | `0` | `1_000_000` | — | [index](constant-rationale-index.md#privacy_epsilon) |
| `hll_precision` | `4` | `16` | — | [index](constant-rationale-index.md#hll_precision) |
| `history_max_depth` | `1` | `50` | — | [index](constant-rationale-index.md#history_max_depth) |
| `upgrade_delay` | `172_800s` | `1_209_600s` | — | [index](constant-rationale-index.md#upgrade_delay) |
| `reveal_window` | `1` | `86_400` | — | [index](constant-rationale-index.md#reveal_window) |
| `heartbeat_alert_threshold` | `1s` | `86_400s` | — | [index](constant-rationale-index.md#heartbeat_alert_threshold) |
| `oracle_staleness_threshold` | `1s` | `86_400s` | — | [index](constant-rationale-index.md#oracle_staleness_threshold) |
| `pair_volatility_window` | `1` | `86_400` | — | [index](constant-rationale-index.md#pair_volatility_window) |
| `momentum_window` | `1` | `86_400` | — | [index](constant-rationale-index.md#momentum_window) |
| `momentum_alert_threshold` | `0` | `100` | — | [index](constant-rationale-index.md#momentum_alert_threshold) |
| `admin_threshold` | `1` | `admin_set.len()` | `admin_threshold <= admin_set.len()` | [index](constant-rationale-index.md#admin_threshold) |
| `service_threshold` | `1` | `service_set.len()` | `service_threshold <= service_set.len()` | [index](constant-rationale-index.md#service_threshold) |
| `deletion_approval_policy` | disabled | enabled | approver non-admin and disjoint from admin set | [index](constant-rationale-index.md#deletion_approval_policy) |
| `gate_callers` | — | — | bounded allowlist | [index](constant-rationale-index.md#gate_callers) |

## Core gate parameters

| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration |
|---|---|---:|---|---|---|
| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration | Rationale |
|---|---|---:|---|---|---|---|
| `risk_threshold` | High-risk wallet admitted to swaps / borrows / LP actions | `75` | `0..=100` in practice | Keep in a policy-approved band and review with historical false-positive / false-negative data | Threshold so high that known risky scores routinely pass | [index](constant-rationale-index.md#risk_threshold) |
| `global_min_confidence` | Low-confidence “safe” score admitted as if trustworthy | `0` | `0..=100` | Set non-zero for any protocol using the score for capital access | `0` together with permissive downstream lending or LP admission | [index](constant-rationale-index.md#global_min_confidence) |
| `staleness_window` | Long-old score treated as current | `604_800s` | must be `> 0` | Keep short enough that score freshness matches protocol risk horizon | Extremely long windows for fast-moving AMM / lending flows | [index](constant-rationale-index.md#staleness_window) |
| `finality_buffer` | Unreviewed score becomes live immediately | `0s` | `0..=86_400s` | Use non-zero for higher-value or operator-reviewed deployments | Assuming pending scores are already safe to consume | [index](constant-rationale-index.md#finality_buffer) |
| `cooldown` | Score thrash / spam / rapid manipulation | `3_600s` | `60..=86_400s` | Keep long enough to bound adversarial resubmission churn | Tiny cooldown paired with low confidence floors | [index](constant-rationale-index.md#cooldown) |
| `adaptive_rate_limit` | Volatility spikes bypass flat cooldown assumptions | disabled | `variance_scale >= 0` | Enable for highly reactive pairs only if operators can explain the variance model | Large variance scale without operational monitoring | [index](constant-rationale-index.md#adaptive_rate_limit) |
| `burst_capacity` | Excessive short-term resubmission | implementation-defined bounded integer | non-zero positive integer | Keep minimal; size it to legitimate remediation bursts | Large burst capacity combined with short cooldown | [index](constant-rationale-index.md#burst_capacity) |
| `score_floor_policy` | Whitewashing known high-risk wallets with suddenly low scores | disabled with `(80,20)` defaults | `high_water_mark ∈ [50,100]`, `floor_value < high_water_mark` | Enable in production scoring systems that can face signer compromise or collusion | Disabled in environments where “score zeroing” would create material downstream loss | [index](constant-rationale-index.md#score_floor_policy) |
| `hysteresis_margin` | Gate flapping near threshold | `0` | must stay `< risk_threshold` | Use small positive margin if downstream actions are sensitive to oscillation | Margin near threshold that delays exit from high-risk state too aggressively | [index](constant-rationale-index.md#hysteresis_margin) |
| `escalation_threshold` | Repeated high-risk breaches ignored as isolated events | `5` | `2..=20` | Lower for fast-response monitoring; higher only with good operator staffing | `1`-equivalent behavior via external automation that pages on every blip | [index](constant-rationale-index.md#escalation_threshold) |

## Data-quality and model controls

| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration |
|---|---|---:|---|---|---|
| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration | Rationale |
|---|---|---:|---|---|---|---|
| `consensus_config (k, epsilon)` | Weak model consensus or over-wide disagreement accepted | `(2, 5)` | validated on write | Keep `k` aligned to signer diversity and `epsilon` aligned to tolerated disagreement | `k` too small for signer count, or `epsilon` so large that consensus is nominal only | [index](constant-rationale-index.md#consensus_config) |
| `adaptive_epsilon` | Static consensus slack under changing variance | disabled | bounded by configured min/max | Use only with explicit min/max policy and monitoring | Enabled with wide min/max and no alerting | [index](constant-rationale-index.md#adaptive_epsilon) |
| `privacy_epsilon` | Private aggregate query leaks too much signal | `0` (disabled) | `0` or scaled positive integer | Treat `0` as exact query mode; if enabling privacy, document composition budget off-chain | Small epsilon assumed to solve composition by itself | [index](constant-rationale-index.md#privacy_epsilon) |
| `hll_precision` | Unique-wallet estimate too noisy or too expensive | `8` | `4..=16` | Keep default unless the deployment has measured need | Over-precision that wastes resources without operational value | [index](constant-rationale-index.md#hll_precision) |
| `cluster_boundaries` | Inconsistent risk segmentation | implementation default | ordered list | Version-control and export them with policy review | Unreviewed boundary changes that alter downstream treatment silently | [index](constant-rationale-index.md#cluster_boundaries) |

## Resource-bounding and liveness controls

| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration |
|---|---|---:|---|---|---|
| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration | Rationale |
|---|---|---:|---|---|---|---|
| `history_max_depth` | Unbounded history growth | `10` | `1..=50` | Keep near default unless analytics require more | Depth increases without storage-cost review | [index](constant-rationale-index.md#history_max_depth) |
| `upgrade_delay` | Instant governance capture | `172_800s` | `172_800..=1_209_600s` | Keep at least 48 hours; longer for externally governed deployments | Shrinking delay to the minimum during active incident response without public notice | [index](constant-rationale-index.md#upgrade_delay) |
| `reveal_window` | Commit-reveal liveness failure or indefinite hanging | bounded contract setting | positive bounded integer | Size to realistic operator latency | Oversized windows that delay dispute resolution materially | [index](constant-rationale-index.md#reveal_window) |
| `heartbeat_alert_threshold` | Silent service outage undetected | `3_600s` | positive bounded integer | Keep shorter than your incident SLO | Alert threshold longer than tolerated outage duration | [index](constant-rationale-index.md#heartbeat_alert_threshold) |
| `oracle_staleness_threshold` | Confidence adjusted by dead oracle feed | `3_600s` | positive bounded integer | Keep matched to external oracle cadence | Much larger than the oracle’s expected publish interval | [index](constant-rationale-index.md#oracle_staleness_threshold) |
| `pair_volatility_window` | Variance estimate dominated by stale data | implementation default | positive bounded integer | Align with market regime you care about | Windows so long that old shocks dominate current behavior | [index](constant-rationale-index.md#pair_volatility_window) |
| `momentum_window` / `momentum_alert_threshold` | Missing fast directional shifts | implementation default | bounded integer settings | Calibrate together; alert threshold without window context is meaningless | Tiny threshold with noisy short window | [index](constant-rationale-index.md#momentum_window) |

## Authorization and irreversible operations

| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration |
|---|---|---:|---|---|---|
| Parameter | Threat controlled | Default | Bounds / rules | Safe production guidance | Forbidden or high-risk configuration | Rationale |
|---|---|---:|---|---|---|---|
| `admin_set` / `admin_threshold` | Single-key governance capture | legacy single-admin mode | threshold must not exceed set size | Use multisig in production | Threshold `0` in a deployment that claims multisig governance | [index](constant-rationale-index.md#admin_threshold) |
| `service_set` / `service_threshold` | Single-signer score submission compromise | legacy single-service mode | threshold must not exceed set size | Use diversified signers or threshold signatures | Single signer with no compensating operational control | [index](constant-rationale-index.md#service_threshold) |
| `deletion_approval_policy` | Routine admins irreversibly delete scores or history | disabled | if enabled, approver must be non-admin and remain disjoint from admin set | Enable wherever deletion is a material privacy or legal action | Enabled with no approver, or approver overlapping the admin key/admin set | [index](constant-rationale-index.md#deletion_approval_policy) |
| `gate_callers` / `gate_enforcement_mode` | Unauthorized contracts query or enforce gates incorrectly | open by default unless restricted | bounded allowlist | Restrict callers when the deployment model expects explicit integrators | Assuming allowlist enforcement exists while gate remains open | [index](constant-rationale-index.md#gate_callers) |

## Operational notes

- Public ABI impact from this change set:
  - `set_deletion_approval_policy`
  - `get_deletion_approval_policy`
  - `export_configuration`
  - `get_parameter_bounds` (read-only bounds registry entry point)
- Event impact:
  - adds `del_pol`
- Storage impact:
  - adds deletion-policy keys only; existing storage is unchanged
  - the bounds registry is a compile-time table, not runtime storage, so it
    cannot be mutated by governance
- Worst relevant bounded-resource case:
  - `export_configuration()` is bounded by a fixed list of global parameters and
    the capped pending-proposal index
  - `get_parameter_bounds()` returns a fixed-size table
- Rationale maintenance:
  - every annotated constant in `contracts/ledgerlens-score/src/constants.rs`
    carries a co-located comment summarizing why its value was chosen and
    linking back to this document and to
    [constant-rationale-index.md](constant-rationale-index.md)
  - changing an annotated constant requires re-checking the linked rationale;
    see the review convention in `CONTRIBUTING.md`
