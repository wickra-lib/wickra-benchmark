// A minimal C++ example: load a curated benchmark case and its dataset, recompute
// the report with the wickra-benchmark C ABI, and assert it reproduces — both
// `passed` (the report matches the frozen expectation) and `hash_match` (its
// canonical hash matches).
//
// No JSON parser is needed: the case JSON is read verbatim and embedded as the
// `case` value, the CSV is turned into a candle array by hand, and the response
// is inspected with a substring search. DATA_DIR is injected by CMake.
#include <cstddef>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "wickra_benchmark.h"

namespace {

std::string slurp(const std::string &path) {
    std::ifstream in(path, std::ios::binary);
    std::ostringstream ss;
    ss << in.rdbuf();
    return ss.str();
}

// Turn a `time,open,high,low,close,volume` CSV (with a header row) into a JSON
// candle array.
std::string candlesJson(const std::string &csvPath) {
    std::ifstream in(csvPath);
    std::string line, out = "[";
    bool first = true;
    while (std::getline(in, line)) {
        long long t;
        double o, h, l, c, v;
        if (std::sscanf(line.c_str(), "%lld,%lf,%lf,%lf,%lf,%lf", &t, &o, &h, &l, &c, &v) != 6) {
            continue;  // the header row fails to parse and is skipped
        }
        char buf[256];
        std::snprintf(buf, sizeof buf,
                      "%s{\"time\":%lld,\"open\":%.17g,\"high\":%.17g,\"low\":%.17g,"
                      "\"close\":%.17g,\"volume\":%.17g}",
                      first ? "" : ",", t, o, h, l, c, v);
        out += buf;
        first = false;
    }
    return out + "]";
}

std::string run(WickraBenchmark *b, const std::string &cmd) {
    int len = wickra_benchmark_command(b, cmd.c_str(), nullptr, 0);
    if (len < 0) {
        std::cerr << "command failed: code " << len << "\n";
        return {};
    }
    std::vector<char> buf(static_cast<std::size_t>(len) + 1);
    wickra_benchmark_command(b, cmd.c_str(), buf.data(), buf.size());
    return std::string(buf.data());
}

}  // namespace

int main() {
    const std::string caseJson = slurp(std::string(DATA_DIR) + "/cases/sma-crossover-01.json");
    const std::string dataJson = candlesJson(std::string(DATA_DIR) + "/datasets/sma-uptrend.csv");
    if (caseJson.empty() || dataJson.empty()) {
        return 1;
    }

    const std::string cmd =
        R"({"cmd":"run_case","case":)" + caseJson + R"(,"data":)" + dataJson + "}";

    WickraBenchmark *b = wickra_benchmark_new();
    const std::string resp = run(b, cmd);
    const bool ok = resp.find("\"passed\":true") != std::string::npos &&
                    resp.find("\"hash_match\":true") != std::string::npos;

    std::cout << "wickra-benchmark " << wickra_benchmark_version() << "\n";
    std::cout << "sma-crossover-01: " << (ok ? "REPRODUCED (passed + hash_match)" : "MISMATCH")
              << "\n";

    wickra_benchmark_free(b);
    if (!ok) {
        std::cerr << "the case did not reproduce\n";
        return 1;
    }
    return 0;
}
