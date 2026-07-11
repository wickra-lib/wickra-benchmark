using Wickra.Benchmark;
using Xunit;

namespace WickraBenchmark.Tests;

public class GoldenTests
{
    // Cross-language golden parity: for each committed golden/commands/*.json (a
    // full command envelope), drive command_json and assert the response equals
    // golden/expected/<name>.json byte-for-byte. The binding returns the core's
    // canonical command_json string verbatim, so byte equality is the exact
    // cross-language parity check. The fixtures arrive in a later phase; until
    // then this test finds no fixtures and passes trivially.

    private static string? GoldenDir()
    {
        string? dir = AppContext.BaseDirectory;
        for (int i = 0; i < 10 && !string.IsNullOrEmpty(dir); i++)
        {
            string g = Path.Combine(dir, "golden");
            if (Directory.Exists(Path.Combine(g, "commands")))
            {
                return g;
            }
            dir = Path.GetDirectoryName(dir);
        }
        return null;
    }

    [Fact]
    public void GoldenParity()
    {
        string? g = GoldenDir();
        if (g is null)
        {
            return; // fixtures not present yet
        }
        foreach (string cmdPath in Directory.GetFiles(Path.Combine(g, "commands"), "*.json"))
        {
            string name = Path.GetFileName(cmdPath);
            string cmd = File.ReadAllText(cmdPath);
            string expected = File.ReadAllText(Path.Combine(g, "expected", name)).Trim();
            using var bench = new Benchmark();
            Assert.Equal(expected, bench.Command(cmd));
        }
    }
}
