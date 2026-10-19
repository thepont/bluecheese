// The after half of the pair whose before half is
// tests/fixtures/smelly/ExtractMe.java, and frozen.
//
// Extract Method: the same work, in two short methods that each say what they
// do. Nothing else changed, so the detector going quiet here is the refactoring
// doing its job rather than the smell being hidden.

package refactored;

public class ExtractMe {

    private double total;

    public String build(int[] values, String[] names) {
        return String.format("%.0f%s", sum(values), labels(names));
    }

    private double sum(int[] values) {
        total = 0;
        for (int value : values) {
            total += value;
        }
        return total;
    }

    private String labels(String[] names) {
        StringBuilder report = new StringBuilder();
        for (String name : names) {
            report.append(name);
        }
        return report.toString();
    }
}
