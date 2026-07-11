// A runnable Java example: load a curated benchmark case and its dataset,
// recompute the report with the wickra-benchmark C ABI binding, and assert it
// reproduces — both `passed` (the report matches the frozen expectation) and
// `hash_match` (its canonical hash matches).
//
//   cargo build -p wickra-benchmark-c
//   mvn -f bindings/java/pom.xml -q package -DskipTests
//   javac -cp bindings/java/target/classes examples/java/Run.java -d examples/java/out
//   java --enable-native-access=ALL-UNNAMED \
//        -Dnative.lib.dir=target/debug \
//        -cp "bindings/java/target/classes;examples/java/out" Run
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import org.wickra.benchmark.Benchmark;

public final class Run {
    private static Path dataDir() {
        Path dir = Path.of("").toAbsolutePath();
        for (int i = 0; i < 10 && dir != null; i++, dir = dir.getParent()) {
            Path candidate = dir.resolve("examples").resolve("data");
            if (Files.isDirectory(candidate)) {
                return candidate;
            }
        }
        throw new IllegalStateException("examples/data not found");
    }

    private static String candlesJson(Path csv) throws Exception {
        StringBuilder sb = new StringBuilder("[");
        List<String> lines = Files.readAllLines(csv);
        boolean first = true;
        for (String line : lines.subList(1, lines.size())) { // skip the header
            String[] c = line.split(",");
            if (c.length < 6) {
                continue;
            }
            if (!first) {
                sb.append(',');
            }
            first = false;
            sb.append("{\"time\":").append(Long.parseLong(c[0].trim()))
                    .append(",\"open\":").append(Double.parseDouble(c[1]))
                    .append(",\"high\":").append(Double.parseDouble(c[2]))
                    .append(",\"low\":").append(Double.parseDouble(c[3]))
                    .append(",\"close\":").append(Double.parseDouble(c[4]))
                    .append(",\"volume\":").append(Double.parseDouble(c[5]))
                    .append('}');
        }
        return sb.append(']').toString();
    }

    public static void main(String[] args) throws Exception {
        Path data = dataDir();
        String caseJson = Files.readString(data.resolve("cases").resolve("sma-crossover-01.json"));
        String candles = candlesJson(data.resolve("datasets").resolve("sma-uptrend.csv"));

        try (Benchmark benchmark = new Benchmark()) {
            String response = benchmark.command(
                    "{\"cmd\":\"run_case\",\"case\":" + caseJson + ",\"data\":" + candles + "}");

            System.out.println("wickra-benchmark " + Benchmark.version());
            boolean ok = response.contains("\"passed\":true")
                    && response.contains("\"hash_match\":true");
            System.out.println("sma-crossover-01: "
                    + (ok ? "REPRODUCED (passed + hash_match)" : "MISMATCH"));
            if (!ok) {
                throw new IllegalStateException("the curated case must reproduce, got: " + response);
            }
        }
    }
}
