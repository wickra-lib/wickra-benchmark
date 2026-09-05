## Cross-language golden parity for the R binding.
##
## For each committed golden/commands/*.json (a full command envelope), drive
## wkbench_command and assert the response equals golden/expected/<name>.json
## byte for byte. The binding returns the core's canonical output verbatim, so
## byte equality *is* the cross-language parity check -- the same assertion the
## other nine bindings make.
##
## This file is deliberately excluded from the source tarball by .Rbuildignore:
## the corpus lives at the repository root, above the package, so `R CMD check`
## on the tarball could not resolve it. CI runs this file explicitly from the
## repository root instead. Failing loudly there beats skipping quietly here --
## which is what the previous combined test did, leaving a shipped test that
## looked as though it had checked parity when it had not.

library(wickrabenchmark)

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
if (is.null(g)) {
  stop("golden/ not found above ", getwd(), " - run this from the repository root")
}

fixtures <- list.files(file.path(g, "commands"), pattern = "[.]json$", full.names = TRUE)
stopifnot(length(fixtures) >= 8)

for (cmd_path in fixtures) {
  name <- basename(cmd_path)
  cmd_json <- paste(readLines(cmd_path, warn = FALSE), collapse = "\n")
  expected <- trimws(paste(
    readLines(file.path(g, "expected", name), warn = FALSE),
    collapse = "\n"
  ))
  bench <- wkbench_new()
  got <- wkbench_command(bench, cmd_json)
  if (!identical(trimws(got), expected)) {
    stop("golden response mismatch for ", name)
  }
}

cat("wickra-benchmark R golden parity passed (", length(fixtures), " fixtures)\n", sep = "")
