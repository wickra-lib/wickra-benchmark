## Batch equivalence: a suite must produce exactly what the cases produce alone.
##
## run_suite is the batch form of run_case. It fans the cases out -- over rayon
## when the parallel feature is on -- and re-sorts the results by id before
## tallying, so the two paths share an engine but not a control flow. Nothing
## else holds them to the same answer.
##
## Everything here builds its own data, so it travels in the source tarball and
## runs from the installed package with nothing above it.

library(wickrabenchmark)

zero_hash <- paste(rep("0", 64), collapse = "")

## Deliberately out of id order: a suite that only ever sees sorted input cannot
## show that it sorts.
order_ <- c(3, 1, 2)

strategy <- function(fast, slow) {
  paste0(
    '{"symbol":"BTCUSDT","timeframe":"1h",',
    '"indicators":{"ema_fast":{"type":"Ema","params":[', fast, ']},',
    '"ema_slow":{"type":"Ema","params":[', slow, ']}},',
    '"entry":{"cross_above":["ema_fast","ema_slow"]},',
    '"exit":{"cross_below":["ema_fast","ema_slow"]},',
    '"sizing":{"type":"fixed_fraction","fraction":0.95},',
    '"costs":{"taker_bps":5,"slippage":{"type":"fixed_bps","bps":2}}}'
  )
}

candles <- function(seed) {
  parts <- vapply(0:39, function(i) {
    b <- 100.0 + sin(i * 0.4 + seed) * 8.0
    paste0(
      '{"time":', format(1700000000 + i * 3600, scientific = FALSE),
      ',"open":', b, ',"high":', b + 1.0, ',"low":', b - 1.0,
      ',"close":', b + 0.5, ',"volume":1000.0}'
    )
  }, character(1))
  paste0("[", paste(parts, collapse = ","), "]")
}

case_json <- function(n) {
  paste0(
    '{"id":"case-0', n, '","description":"batch equivalence",',
    '"strategy":', strategy(3 + n, 12 + n), ',',
    '"dataset_ref":"d', n, '.csv",',
    '"expected":{},"expected_hash":"', zero_hash, '"}'
  )
}

suite_json <- function(ids) {
  paste0('{"name":"batch","cases":[',
         paste(vapply(ids, case_json, character(1)), collapse = ","), "]}")
}

datasets_json <- function() {
  parts <- vapply(seq_along(order_), function(i) {
    paste0('"d', order_[i], '.csv":', candles(i - 1))
  }, character(1))
  paste0("{", paste(parts, collapse = ","), "}")
}

run_suite <- function(bench, ids) {
  wkbench_command(bench, paste0(
    '{"cmd":"run_suite","suite":', suite_json(ids),
    ',"datasets":', datasets_json(), "}"
  ))
}

bench <- wkbench_new()

## Each case on its own.
alone <- vapply(seq_along(order_), function(i) {
  wkbench_command(bench, paste0(
    '{"cmd":"run_case","case":', case_json(order_[i]),
    ',"data":', candles(i - 1), "}"
  ))
}, character(1))

report <- run_suite(bench, order_)

## Sorted by id, whatever order the cases were listed in.
stopifnot(regexpr("case-01", report, fixed = TRUE) <
            regexpr("case-02", report, fixed = TRUE))
stopifnot(regexpr("case-02", report, fixed = TRUE) <
            regexpr("case-03", report, fixed = TRUE))

## run_case returns exactly one CaseResult and run_suite returns those same
## objects in an array -- both canonical, both whitespace-free. So the standalone
## response must appear verbatim inside the report.
for (response in alone) {
  stopifnot(grepl(response, report, fixed = TRUE))
}

stopifnot(grepl('"failed":3', report, fixed = TRUE))
stopifnot(grepl('"passed":0', report, fixed = TRUE))

## The listed order must not reach the report.
stopifnot(identical(run_suite(bench, order_), run_suite(bench, rev(order_))))

cat("wickra-benchmark R batch equivalence passed\n")
