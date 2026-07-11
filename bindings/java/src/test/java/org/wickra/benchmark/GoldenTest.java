package org.wickra.benchmark;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;

class GoldenTest {
    // Cross-language golden parity: for each committed golden/commands/*.json (a
    // full command envelope), drive command_json and assert the response equals
    // golden/expected/<name>.json byte-for-byte. The binding returns the core's
    // canonical command_json string verbatim, so byte equality is the exact
    // cross-language parity check. The fixtures arrive in a later phase; until
    // then this test finds no fixtures and passes trivially.

    private static Path goldenDir() {
        Path dir = Path.of("").toAbsolutePath();
        for (int i = 0; i < 10 && dir != null; i++) {
            Path g = dir.resolve("golden");
            if (Files.isDirectory(g.resolve("commands"))) {
                return g;
            }
            dir = dir.getParent();
        }
        return null;
    }

    @Test
    void goldenParity() throws IOException {
        Path g = goldenDir();
        if (g == null) {
            return; // fixtures not present yet
        }
        try (Stream<Path> commands = Files.list(g.resolve("commands"))) {
            for (Path cmdPath : commands.filter(p -> p.toString().endsWith(".json")).toList()) {
                String name = cmdPath.getFileName().toString();
                String cmd = Files.readString(cmdPath);
                String expected = Files.readString(g.resolve("expected").resolve(name)).strip();
                try (Benchmark bench = new Benchmark()) {
                    assertEquals(expected, bench.command(cmd));
                }
            }
        }
    }
}
