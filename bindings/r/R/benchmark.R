#' The wickra-benchmark library version.
#' @return A version string.
#' @export
wkbench_version <- function() {
  .Call(C_wkbench_version)
}

#' Create a benchmark runner.
#' @return A `wickra_benchmark` handle (an external pointer).
#' @export
wkbench_new <- function() {
  .Call(C_wkbench_new)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param benchmark A benchmark handle from [wkbench_new()].
#' @param cmd_json A command JSON string.
#' @return The response as a JSON string.
#' @export
wkbench_command <- function(benchmark, cmd_json) {
  .Call(C_wkbench_command, benchmark, cmd_json)
}
