// SMELLY ON PURPOSE, and frozen: see tests/refactored.rs for what this pins.
//
// The before half of an Extract Method pair, and the book's primary answer to
// Long Method. One method does two jobs: it accumulates and it renders. Its twin
// in tests/fixtures/refactored/ does the same work in short methods.

package smelly;

public class ExtractMe {

    private final StringBuilder report = new StringBuilder();
    private double total;

    public String build(int[] values, String[] names) {
        total = 0;
        total += values[0];
        total += values[1];
        total += values[2];
        total += values[3];
        total += values[4];
        total += values[5];
        total += values[6];
        total += values[7];
        total += values[8];
        total += values[9];
        total += values[10];
        total += values[11];
        total += values[12];
        total += values[13];
        total += values[14];
        report.append(names[0]);
        report.append(names[1]);
        report.append(names[2]);
        report.append(names[3]);
        report.append(names[4]);
        report.append(names[5]);
        report.append(names[6]);
        report.append(names[7]);
        report.append(names[8]);
        report.append(names[9]);
        report.append(names[10]);
        report.append(names[11]);
        report.append(names[12]);
        report.append(names[13]);
        report.append(names[14]);
        return report.toString();
    }
}
