---
name: Propose a benchmark case
about: Suggest a new (strategy, dataset, expected report) case for the suite
title: "[case] "
labels: case-proposal
---

**What does this case exercise?**
The strategy family, market regime, or edge case it pins that the current suite
does not (e.g. a mean-reversion strategy on a choppy range, a stop-out on a gap).

**Strategy**
The `wickra-backtest` `StrategySpec` (as JSON) the case runs. Keep it minimal and
deterministic.

**Dataset**
How the candle dataset is generated. It must be **deterministic** — a documented
closed-form formula (e.g. `close(i) = 100 + 0.5*i`), not scraped live data — so
the expected report and hash are reproducible by anyone.

**Why it belongs in the suite**
What reproducibility property it adds. Cases should be small enough to recompute
by hand and broad enough to catch a real engine divergence.

> The `expected` report and `expected_hash` are recomputed by the maintainers
> from the real engine when the case is blessed; you do not need to fill them in.
