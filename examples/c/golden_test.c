// Golden parity and batch equivalence, driven through the plain C ABI.
//
// The other nine bindings each carry these two assertions; C carried neither,
// because its only coverage was Rust unit tests inside bindings/c/src/lib.rs --
// which exercise the ABI from the language that defines it, not from the
// language that consumes it. This file is compiled the way a consumer compiles,
// against the generated header.
//
// Two claims:
//
//   golden  -- every committed golden/commands/*.json must produce exactly the
//              bytes in golden/expected/<name>.json. Byte equality is the
//              cross-language parity check, because every binding returns the
//              core's canonical string verbatim.
//   batch   -- run_suite is the batch form of run_case: it fans the cases out
//              (over rayon when the parallel feature is on) and re-sorts by id
//              before tallying, so the two paths share an engine but not a
//              control flow. A case's result inside a suite must be byte-equal
//              to that case run on its own.
//
// GOLDEN_DIR and DATA_DIR are injected by CMake.
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_benchmark.h"

// Longest committed fixture is a few hundred kB; a suite embeds five of them.
#define MAX_JSON (8 * 1024 * 1024)

static char *slurp(const char *path, size_t *len_out) {
    FILE *f = fopen(path, "rb");
    if (!f) {
        return NULL;
    }
    char *buf = malloc(MAX_JSON);
    if (!buf) {
        fclose(f);
        return NULL;
    }
    size_t n = fread(buf, 1, MAX_JSON - 1, f);
    fclose(f);
    buf[n] = '\0';
    if (len_out) {
        *len_out = n;
    }
    return buf;
}

// The committed files end with a newline; the ABI response does not.
static void rstrip(char *s) {
    size_t n = strlen(s);
    while (n > 0 && (s[n - 1] == '\n' || s[n - 1] == '\r' || s[n - 1] == ' ')) {
        s[--n] = '\0';
    }
}

static char *command(WickraBenchmark *b, const char *cmd) {
    int len = wickra_benchmark_command(b, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return NULL;
    }
    char *out = malloc((size_t)len + 1);
    if (!out) {
        return NULL;
    }
    if (wickra_benchmark_command(b, cmd, out, (size_t)len + 1) < 0) {
        free(out);
        return NULL;
    }
    return out;
}

static const char *FIXTURES[] = {
    "breakout-channel-01", "buy-and-hold-01",   "ema-trend-follow-01",
    "rsi-mean-reversion-01", "sma-crossover-01", "suite-list",
    "suite-run",             "version",
};

static int golden_parity(WickraBenchmark *b) {
    int failures = 0;
    for (size_t i = 0; i < sizeof FIXTURES / sizeof FIXTURES[0]; i++) {
        char cmd_path[1024];
        char exp_path[1024];
        snprintf(cmd_path, sizeof cmd_path, "%s/commands/%s.json", GOLDEN_DIR, FIXTURES[i]);
        snprintf(exp_path, sizeof exp_path, "%s/expected/%s.json", GOLDEN_DIR, FIXTURES[i]);

        char *cmd = slurp(cmd_path, NULL);
        char *expected = slurp(exp_path, NULL);
        if (!cmd || !expected) {
            fprintf(stderr, "cannot read fixture %s\n", FIXTURES[i]);
            free(cmd);
            free(expected);
            failures++;
            continue;
        }
        rstrip(expected);

        char *got = command(b, cmd);
        if (!got || strcmp(got, expected) != 0) {
            fprintf(stderr, "golden mismatch for %s\n", FIXTURES[i]);
            failures++;
        }
        free(cmd);
        free(expected);
        free(got);
    }
    if (failures == 0) {
        printf("golden parity: %zu fixtures byte-identical\n",
               sizeof FIXTURES / sizeof FIXTURES[0]);
    }
    return failures;
}

// suite-run embeds the whole suite; each per-case fixture runs one of its cases
// against the same data. So every run_case response must appear verbatim inside
// the suite report -- both are canonical and whitespace-free.
static int batch_equivalence(WickraBenchmark *b) {
    char path[1024];
    snprintf(path, sizeof path, "%s/commands/suite-run.json", GOLDEN_DIR);
    char *suite_cmd = slurp(path, NULL);
    if (!suite_cmd) {
        fprintf(stderr, "cannot read suite-run.json\n");
        return 1;
    }
    char *report = command(b, suite_cmd);
    free(suite_cmd);
    if (!report) {
        return 1;
    }

    int failures = 0;
    for (size_t i = 0; i < 5; i++) {  // the five run_case fixtures
        snprintf(path, sizeof path, "%s/commands/%s.json", GOLDEN_DIR, FIXTURES[i]);
        char *cmd = slurp(path, NULL);
        if (!cmd) {
            failures++;
            continue;
        }
        char *alone = command(b, cmd);
        free(cmd);
        if (!alone || !strstr(report, alone)) {
            fprintf(stderr, "%s differs between the suite and the case run alone\n", FIXTURES[i]);
            failures++;
        }
        free(alone);
    }

    // The suite sorts by id regardless of the order its cases were listed in.
    const char *first = strstr(report, "breakout-channel-01");
    const char *last = strstr(report, "sma-crossover-01");
    if (!first || !last || first > last) {
        fprintf(stderr, "the suite report is not sorted by id\n");
        failures++;
    }
    if (!strstr(report, "\"failed\":0")) {
        fprintf(stderr, "every blessed case in the suite must reproduce\n");
        failures++;
    }
    free(report);

    if (failures == 0) {
        printf("batch equivalence: 5 cases identical inside and outside the suite\n");
    }
    return failures;
}

int main(void) {
    WickraBenchmark *b = wickra_benchmark_new();
    if (!b) {
        fprintf(stderr, "wickra_benchmark_new returned NULL\n");
        return 1;
    }

    int failures = golden_parity(b) + batch_equivalence(b);
    wickra_benchmark_free(b);

    if (failures) {
        fprintf(stderr, "%d check(s) failed\n", failures);
        return 1;
    }
    printf("wickra-benchmark C golden + batch checks passed\n");
    return 0;
}
