// Optional C++ convenience layer over the wickra-benchmark C ABI
// (`wickra_benchmark.h`). Header-only, and hand-written: cbindgen generates the
// `.h` beside this file, not this file.
//
// C++ already reaches every export through the C header, which is `extern "C"`.
// What it does not get for free is the two things a caller otherwise repeats at
// every call site:
//
//   - the handle from `wickra_benchmark_new` must be released exactly once with
//     `wickra_benchmark_free`, including on every early return;
//   - `wickra_benchmark_command` writes into a caller-provided buffer and
//     returns the response length. When that length is not smaller than the
//     capacity it writes nothing, so a caller has to ask for the length, size a
//     buffer, and call again -- a two-step dance repeated verbatim wherever a
//     command is issued.
//
// `Benchmark` below owns the handle in a move-only type that releases at scope
// exit, and folds the dance into one call returning a `std::string`:
//
//     #include "wickra_benchmark.hpp"
//
//     wickra::benchmark::Benchmark bench;
//     std::string response = bench.command(R"({"cmd":"version"})");
//     if (response.empty()) { /* bench.last_error() has the code */ }
//
// `wickra_benchmark_version` needs no wrapper: it returns a static string the
// caller does not own.
//
// This layer throws nothing, allocates only the returned string, and adds no
// runtime cost beyond the C calls themselves.

#ifndef WICKRA_BENCHMARK_HPP
#define WICKRA_BENCHMARK_HPP

#include "wickra_benchmark.h"

#include <string>
#include <utility>
#include <vector>

namespace wickra {
namespace benchmark {

/// Move-only owner of a `WickraBenchmark *`, released with
/// `wickra_benchmark_free`.
class Benchmark {
public:
    Benchmark() noexcept : handle_(wickra_benchmark_new()), last_error_(0) {}

    /// Adopts an already-obtained handle. Ownership passes to this object.
    explicit Benchmark(WickraBenchmark *handle) noexcept
        : handle_(handle), last_error_(0) {}

    ~Benchmark() {
        if (handle_ != nullptr) {
            wickra_benchmark_free(handle_);
        }
    }

    Benchmark(const Benchmark &) = delete;
    Benchmark &operator=(const Benchmark &) = delete;

    Benchmark(Benchmark &&other) noexcept
        : handle_(other.handle_), last_error_(other.last_error_) {
        other.handle_ = nullptr;
    }

    Benchmark &operator=(Benchmark &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                wickra_benchmark_free(handle_);
            }
            handle_ = other.handle_;
            last_error_ = other.last_error_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    /// The underlying handle, for calls this wrapper does not cover. Ownership
    /// stays here.
    WickraBenchmark *get() const noexcept { return handle_; }

    /// Gives up ownership. The caller becomes responsible for freeing it.
    WickraBenchmark *release() noexcept {
        WickraBenchmark *out = handle_;
        handle_ = nullptr;
        return out;
    }

    /// Whether this owns a handle. A default-constructed `Benchmark` is empty
    /// only if the allocation failed.
    explicit operator bool() const noexcept { return handle_ != nullptr; }

    /// Apply a command envelope and return the canonical response.
    ///
    /// Returns an empty string on failure, with the ABI's negative code in
    /// `last_error()`. A *domain* error -- a malformed case, an unknown command
    /// -- is not a failure here: the core folds those into an
    /// `{"ok":false,"error":...}` response, which comes back as a normal string.
    std::string command(const std::string &cmd_json) noexcept {
        last_error_ = 0;
        if (handle_ == nullptr) {
            last_error_ = WICKRA_BENCHMARK_ERR_NULL;
            return std::string();
        }
        // First call sizes the response; passing a null buffer asks for the
        // length without writing.
        const int len = wickra_benchmark_command(handle_, cmd_json.c_str(), nullptr, 0);
        if (len < 0) {
            last_error_ = len;
            return std::string();
        }
        std::vector<char> buf(static_cast<std::size_t>(len) + 1);
        const int written =
            wickra_benchmark_command(handle_, cmd_json.c_str(), buf.data(), buf.size());
        if (written < 0) {
            last_error_ = written;
            return std::string();
        }
        return std::string(buf.data(), static_cast<std::size_t>(written));
    }

    /// The negative ABI code from the last failed `command`, or 0.
    int last_error() const noexcept { return last_error_; }

private:
    WickraBenchmark *handle_;
    int last_error_;
};

/// The library version. A static string owned by the library.
inline const char *version() noexcept { return wickra_benchmark_version(); }

}  // namespace benchmark
}  // namespace wickra

#endif  // WICKRA_BENCHMARK_HPP
