/* R .Call glue for the wickra-benchmark C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_benchmark.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkbench_finalize(SEXP ext) {
    WickraBenchmark *h = (WickraBenchmark *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_benchmark_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraBenchmark *handle_of(SEXP ext) {
    WickraBenchmark *h = (WickraBenchmark *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-benchmark: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkbench_version(void) {
    return Rf_mkString(wickra_benchmark_version());
}

SEXP wkbench_new(void) {
    WickraBenchmark *h = wickra_benchmark_new();
    if (!h) {
        Rf_error("wickra-benchmark: failed to create a benchmark");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkbench_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkbench_command(SEXP ext, SEXP cmd_json) {
    WickraBenchmark *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_benchmark_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-benchmark: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_benchmark_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkbench_version", (DL_FUNC)&wkbench_version, 0},
    {"wkbench_new", (DL_FUNC)&wkbench_new, 0},
    {"wkbench_command", (DL_FUNC)&wkbench_command, 2},
    {NULL, NULL, 0}};

void R_init_wickrabenchmark(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
