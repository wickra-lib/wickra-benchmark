/* A minimal C example: load a curated benchmark case and its dataset, recompute
 * the report with the wickra-benchmark C ABI, and assert it reproduces — both
 * `passed` (the report matches the frozen expectation) and `hash_match` (its
 * canonical hash matches). This is the whole product in one file.
 *
 * No JSON parser is needed: the case JSON is read verbatim and embedded as the
 * `case` value, the CSV is turned into a candle array by hand, and the response
 * is inspected with a substring search. DATA_DIR is injected by CMake. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_benchmark.h"

/* Read an entire text file into a freshly malloc'd, NUL-terminated buffer. */
static char *slurp(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *buf = (char *)malloc((size_t)n + 1);
    if (buf) {
        size_t got = fread(buf, 1, (size_t)n, f);
        buf[got] = '\0';
    }
    fclose(f);
    return buf;
}

/* Turn a `time,open,high,low,close,volume` CSV (with a header row) into a JSON
 * candle array. Returns a freshly malloc'd string. */
static char *candles_json(const char *csv_path) {
    FILE *f = fopen(csv_path, "r");
    if (!f) {
        fprintf(stderr, "cannot open %s\n", csv_path);
        return NULL;
    }
    size_t cap = 1 << 16, len = 0;
    char *out = (char *)malloc(cap);
    if (!out) {
        fclose(f);
        return NULL;
    }
    len += (size_t)sprintf(out + len, "[");
    char line[256];
    int first = 1, header = 1;
    while (fgets(line, sizeof line, f)) {
        long long t;
        double o, h, l, c, v;
        if (sscanf(line, "%lld,%lf,%lf,%lf,%lf,%lf", &t, &o, &h, &l, &c, &v) != 6) {
            header = 0; /* the header row fails to parse and is skipped */
            continue;
        }
        (void)header;
        if (cap - len < 256) {
            cap *= 2;
            out = (char *)realloc(out, cap);
        }
        len += (size_t)sprintf(out + len,
                               "%s{\"time\":%lld,\"open\":%.17g,\"high\":%.17g,"
                               "\"low\":%.17g,\"close\":%.17g,\"volume\":%.17g}",
                               first ? "" : ",", t, o, h, l, c, v);
        first = 0;
    }
    sprintf(out + len, "]");
    fclose(f);
    return out;
}

/* Run a command and return its response using the length-out protocol. */
static char *run(WickraBenchmark *b, const char *cmd) {
    int len = wickra_benchmark_command(b, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (buf) {
        wickra_benchmark_command(b, cmd, buf, (size_t)len + 1);
    }
    return buf;
}

int main(void) {
    char *case_json = slurp(DATA_DIR "/cases/sma-crossover-01.json");
    char *data_json = candles_json(DATA_DIR "/datasets/sma-uptrend.csv");
    if (!case_json || !data_json) {
        return 1;
    }

    size_t cap = strlen(case_json) + strlen(data_json) + 64;
    char *cmd = (char *)malloc(cap);
    snprintf(cmd, cap, "{\"cmd\":\"run_case\",\"case\":%s,\"data\":%s}", case_json, data_json);

    WickraBenchmark *b = wickra_benchmark_new();
    char *resp = run(b, cmd);
    int ok = resp && strstr(resp, "\"passed\":true") && strstr(resp, "\"hash_match\":true");

    printf("wickra-benchmark %s\n", wickra_benchmark_version());
    printf("sma-crossover-01: %s\n", ok ? "REPRODUCED (passed + hash_match)" : "MISMATCH");

    free(resp);
    free(cmd);
    free(data_json);
    free(case_json);
    wickra_benchmark_free(b);
    if (!ok) {
        fprintf(stderr, "the case did not reproduce\n");
        return 1;
    }
    return 0;
}
