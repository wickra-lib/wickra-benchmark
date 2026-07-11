# A runnable R example: load a curated benchmark case and its dataset, recompute
# the report with the wickra-benchmark C ABI binding, and assert it reproduces —
# both `passed` (the report matches the frozen expectation) and `hash_match`
# (its canonical hash matches).
#
#   cargo build -p wickra-benchmark-c --release
#   export WKBENCH_INC="$PWD/bindings/c/include"
#   export WKBENCH_LIB="$PWD/target/release"
#   export LD_LIBRARY_PATH="$WKBENCH_LIB:$LD_LIBRARY_PATH"   # PATH on Windows
#   R CMD INSTALL bindings/r
#   Rscript examples/r/run.R

library(wickrabenchmark)

# Walk up from the working directory to find examples/data.
data_dir <- function() {
  dir <- normalizePath(getwd(), mustWork = FALSE)
  for (i in seq_len(10)) {
    candidate <- file.path(dir, "examples", "data")
    if (dir.exists(candidate)) {
      return(candidate)
    }
    dir <- dirname(dir)
  }
  stop("examples/data not found")
}

candles_json <- function(csv_path) {
  df <- read.csv(csv_path)
  rows <- vapply(seq_len(nrow(df)), function(i) {
    paste0(
      '{"time":', format(df$time[i], scientific = FALSE),
      ',"open":', sprintf("%.17g", df$open[i]),
      ',"high":', sprintf("%.17g", df$high[i]),
      ',"low":', sprintf("%.17g", df$low[i]),
      ',"close":', sprintf("%.17g", df$close[i]),
      ',"volume":', sprintf("%.17g", df$volume[i]), "}"
    )
  }, character(1))
  paste0("[", paste(rows, collapse = ","), "]")
}

data <- data_dir()
case_json <- paste(
  readLines(file.path(data, "cases", "sma-crossover-01.json"), warn = FALSE),
  collapse = "\n"
)
candles <- candles_json(file.path(data, "datasets", "sma-uptrend.csv"))

bench <- wkbench_new()
response <- wkbench_command(
  bench,
  paste0('{"cmd":"run_case","case":', case_json, ',"data":', candles, "}")
)

cat("wickra-benchmark", wkbench_version(), "\n")
ok <- grepl('"passed":true', response, fixed = TRUE) &&
  grepl('"hash_match":true', response, fixed = TRUE)
stopifnot(ok)
cat("sma-crossover-01: REPRODUCED (passed + hash_match)\n")
