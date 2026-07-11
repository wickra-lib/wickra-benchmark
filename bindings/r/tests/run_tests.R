## Plain-R tests for the wickra-benchmark R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickrabenchmark)

strategy <- paste0(
  '{"symbol":"BTCUSDT","timeframe":"1h",',
  '"indicators":{"ema_fast":{"type":"Ema","params":[5]},',
  '"ema_slow":{"type":"Ema","params":[15]}},',
  '"entry":{"cross_above":["ema_fast","ema_slow"]},',
  '"exit":{"cross_below":["ema_fast","ema_slow"]},',
  '"sizing":{"type":"fixed_fraction","fraction":0.95},',
  '"costs":{"taker_bps":5,"slippage":{"type":"fixed_bps","bps":2}}}'
)

candles <- function() {
  parts <- vapply(0:39, function(i) {
    b <- 100.0 + sin(i * 0.4) * 8.0
    paste0(
      '{"time":', format(1700000000 + i * 3600, scientific = FALSE),
      ',"open":', b, ',"high":', b + 1.0, ',"low":', b - 1.0,
      ',"close":', b + 0.5, ',"volume":1000.0}'
    )
  }, character(1))
  paste0("[", paste(parts, collapse = ","), "]")
}

## A placeholder expected/hash: the run recomputes the real report; the test
## only checks the response shape, not that the case passes.
run_case_request <- function() {
  the_case <- paste0(
    '{"id":"sma-crossover-01","description":"smoke","strategy":', strategy,
    ',"dataset_ref":"d.csv","expected":{},"expected_hash":"',
    paste(rep("0", 64), collapse = ""), '"}'
  )
  paste0('{"cmd":"run_case","case":', the_case, ',"data":', candles(), '}')
}

## version
stopifnot(nzchar(wkbench_version()))

## run_case returns a shaped CaseResult
bench <- wkbench_new()
result <- wkbench_command(bench, run_case_request())
stopifnot(grepl('"id":"sma-crossover-01"', result, fixed = TRUE))
hash <- regmatches(result, regexpr('"hash":"[0-9a-f]{64}"', result))
stopifnot(length(hash) == 1)

## byte-stable (determinism)
stopifnot(identical(wkbench_command(bench, run_case_request()), result))

## an unknown command is an in-band error, not a hard error
inband <- wkbench_command(bench, '{"cmd":"nope"}')
stopifnot(grepl('"ok":false', inband, fixed = TRUE))

## cross-language golden parity: for each committed golden/commands/*.json (a
## full command envelope), drive command_json and assert the response equals
## golden/expected/<name>.json byte-for-byte. The binding returns the core's
## canonical command output verbatim, so byte equality is the exact
## cross-language parity check. The fixtures arrive in a later phase; until then
## the golden section is skipped.
golden_dir <- function() {
  d <- normalizePath(getwd(), mustWork = FALSE)
  for (i in seq_len(8)) {
    g <- file.path(d, "golden")
    if (dir.exists(file.path(g, "commands"))) {
      return(g)
    }
    d <- dirname(d)
  }
  NULL
}

g <- golden_dir()
if (!is.null(g)) {
  for (cmd_path in list.files(file.path(g, "commands"), pattern = "\\.json$", full.names = TRUE)) {
    name <- basename(cmd_path)
    cmd_json <- paste(readLines(cmd_path, warn = FALSE), collapse = "\n")
    expected <- trimws(paste(
      readLines(file.path(g, "expected", name), warn = FALSE), collapse = "\n"
    ))
    gbench <- wkbench_new()
    got <- wkbench_command(gbench, cmd_json)
    stopifnot(identical(trimws(got), expected))
  }
}

cat("wickra-benchmark R tests passed\n")
