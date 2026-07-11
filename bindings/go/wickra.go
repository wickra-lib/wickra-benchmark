// Package wickra provides idiomatic Go bindings for wickra-benchmark over its C
// ABI hub: create a Benchmark, drive it with command JSON (run_case, run_suite,
// list_cases, version) and read back the response JSON — the same protocol as
// the CLI and every other binding.
//
// The binding links the prebuilt C ABI library, staged per platform under
// ./lib/<goos>_<goarch>/, with the header vendored under ./include.
package wickra

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/lib/linux_amd64 -lwickra_benchmark -Wl,-rpath,${SRCDIR}/lib/linux_amd64
#cgo linux,arm64 LDFLAGS: -L${SRCDIR}/lib/linux_arm64 -lwickra_benchmark -Wl,-rpath,${SRCDIR}/lib/linux_arm64
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/lib/darwin_amd64 -lwickra_benchmark -Wl,-rpath,${SRCDIR}/lib/darwin_amd64
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/lib/darwin_arm64 -lwickra_benchmark -Wl,-rpath,${SRCDIR}/lib/darwin_arm64
#cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/windows_amd64 -l:wickra_benchmark.dll
#cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib/windows_arm64 -l:wickra_benchmark.dll
#include <stdlib.h>
#include "wickra_benchmark.h"
*/
import "C"

import (
	"fmt"
	"runtime"
	"unsafe"
)

// Benchmark is driven by JSON commands. It is stateless — the case, suite and
// data arrive with each command.
type Benchmark struct {
	handle *C.WickraBenchmark
}

// New creates a benchmark handle. Call Close when done (a finalizer also frees
// it, but explicit Close is preferred).
func New() *Benchmark {
	b := &Benchmark{handle: C.wickra_benchmark_new()}
	runtime.SetFinalizer(b, (*Benchmark).Close)
	return b
}

// Command applies a command JSON and returns the response JSON. It uses the C
// ABI's length-out protocol: a first call learns the length, then the response
// is read into a caller-owned buffer.
func (b *Benchmark) Command(cmdJSON string) (string, error) {
	ccmd := C.CString(cmdJSON)
	defer C.free(unsafe.Pointer(ccmd))

	n := C.wickra_benchmark_command(b.handle, ccmd, nil, 0)
	if n < 0 {
		return "", fmt.Errorf("wickra-benchmark: command failed (code %d)", int(n))
	}
	buf := make([]byte, int(n)+1)
	C.wickra_benchmark_command(
		b.handle,
		ccmd,
		(*C.char)(unsafe.Pointer(&buf[0])),
		C.size_t(len(buf)),
	)
	return string(buf[:n]), nil
}

// Close frees the benchmark handle. Safe to call more than once.
func (b *Benchmark) Close() {
	if b.handle != nil {
		C.wickra_benchmark_free(b.handle)
		b.handle = nil
	}
	runtime.SetFinalizer(b, nil)
}

// Version returns the library version.
func Version() string {
	return C.GoString(C.wickra_benchmark_version())
}
