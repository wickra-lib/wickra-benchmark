using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Wickra.Benchmark;

/// <summary>Raw P/Invoke surface for the wickra-benchmark C ABI.</summary>
internal static partial class Native
{
    internal const string Lib = "wickra_benchmark";

    /// <summary>Construct a benchmark handle. Never fails.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_benchmark_new();

    /// <summary>Free a benchmark handle.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial void wickra_benchmark_free(IntPtr handle);

    /// <summary>
    /// Apply a command JSON (NUL-terminated UTF-8), writing the response into a
    /// caller-owned buffer. Returns the response length, or a negative error code.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial int wickra_benchmark_command(IntPtr handle, byte[] cmdUtf8, byte[]? outBuf, nuint cap);

    /// <summary>The library version as a static NUL-terminated string.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_benchmark_version();
}
