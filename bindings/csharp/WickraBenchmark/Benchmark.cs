using System.Runtime.InteropServices;
using System.Text;

namespace Wickra.Benchmark;

/// <summary>
/// A benchmark runner driven by JSON commands, over the Wickra C ABI. Create one,
/// drive it with command JSON (<c>run_case</c>, <c>run_suite</c>,
/// <c>list_cases</c>, <c>version</c>) and read back the response JSON — the same
/// protocol as the CLI and every other binding. It is stateless — the case, suite
/// and data arrive with each command.
/// </summary>
public sealed class Benchmark : IDisposable
{
    private readonly BenchmarkHandle _handle;

    /// <summary>Create a benchmark runner.</summary>
    public Benchmark()
    {
        IntPtr ptr = Native.wickra_benchmark_new();
        if (ptr == IntPtr.Zero)
        {
            throw new InvalidOperationException("wickra-benchmark: failed to create a benchmark");
        }
        _handle = new BenchmarkHandle(ptr);
    }

    /// <summary>Apply a command JSON and return the response JSON.</summary>
    /// <remarks>
    /// Uses the C ABI's length-out protocol: a first call learns the length, then
    /// the response is read into a caller-owned buffer. Domain errors (a bad case,
    /// an unknown command) come back in-band as <c>{"ok":false,...}</c> JSON, not as
    /// an exception.
    /// </remarks>
    /// <exception cref="InvalidOperationException">A required argument was unusable or a panic was caught.</exception>
    public string Command(string cmdJson)
    {
        ObjectDisposedException.ThrowIf(_handle.IsInvalid, this);

        byte[] cmd = Utf8(cmdJson);
        IntPtr h = _handle.DangerousGetHandle();
        int n = Native.wickra_benchmark_command(h, cmd, null, 0);
        if (n < 0)
        {
            throw new InvalidOperationException($"wickra-benchmark: command failed (code {n})");
        }
        var buf = new byte[n + 1];
        Native.wickra_benchmark_command(h, cmd, buf, (nuint)buf.Length);
        return Encoding.UTF8.GetString(buf, 0, n);
    }

    /// <summary>The library version.</summary>
    public static string Version() =>
        Marshal.PtrToStringUTF8(Native.wickra_benchmark_version()) ?? string.Empty;

    /// <summary>Free the native benchmark handle.</summary>
    public void Dispose() => _handle.Dispose();

    /// <summary>Encode a string as NUL-terminated UTF-8 for the C ABI.</summary>
    private static byte[] Utf8(string s)
    {
        int len = Encoding.UTF8.GetByteCount(s);
        var buf = new byte[len + 1];
        Encoding.UTF8.GetBytes(s, 0, s.Length, buf, 0);
        return buf;
    }
}

/// <summary>A safe handle owning a native benchmark pointer.</summary>
internal sealed class BenchmarkHandle : SafeHandle
{
    public BenchmarkHandle(IntPtr handle)
        : base(IntPtr.Zero, ownsHandle: true) => SetHandle(handle);

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        Native.wickra_benchmark_free(handle);
        return true;
    }
}
