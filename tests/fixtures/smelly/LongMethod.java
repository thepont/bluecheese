// SMELLY ON PURPOSE.
//
// This fixture exists so the Long Method detector has a real smell to find.
// Fixtures are the only place in this repository where smells are deliberate;
// `tests/self_check.rs` holds the rest of the source to that promise.
package smelly;

public final class LongMethod {

    private int runningTotal;
    private String lastError;

    // A constructor is not a method: Spoon's CtType.getMethods() does not
    // return one, so the detector must stay silent however long it grows.
    public LongMethod() {
        this.runningTotal = 0;
        this.lastError = "";
    }

    // Smelly by size. Every statement below is counted, so the logical line
    // count is the number of them: 32, against a limit of 30. Nothing branches,
    // so complexity cannot be what triggers the finding.
    public void applyShipment(int[] weights, String[] labels) {
        runningTotal = 0;
        runningTotal += weights[0];
        runningTotal += weights[1];
        runningTotal += weights[2];
        runningTotal += weights[3];
        runningTotal += weights[4];
        runningTotal += weights[5];
        runningTotal += weights[6];
        runningTotal += weights[7];
        runningTotal += weights[8];
        runningTotal += weights[9];
        runningTotal += weights[10];
        runningTotal += weights[11];
        runningTotal += weights[12];
        runningTotal += weights[13];
        runningTotal += weights[14];
        runningTotal += weights[15];
        runningTotal += weights[16];
        runningTotal += weights[17];
        runningTotal += weights[18];
        runningTotal += weights[19];
        runningTotal += weights[20];
        runningTotal += weights[21];
        runningTotal += weights[22];
        runningTotal += weights[23];
        runningTotal += weights[24];
        runningTotal += weights[25];
        runningTotal += weights[26];
        runningTotal += weights[27];
        runningTotal += weights[28];
        runningTotal += weights[29];
        lastError = labels[0];
    }

    // Short, and must stay unreported: a detector that flags everything is as
    // useless as one that flags nothing.
    public int total() {
        return runningTotal;
    }

    // Smelly by branching only: sixteen `&&` inside one `if` put complexity at
    // 18 against a limit of 15, while the method holds two statements, so the
    // size limit cannot be what triggers the finding.
    public boolean inRange(int a) {
        if (a > 0
            && a < 1
            && a < 2
            && a < 3
            && a < 4
            && a < 5
            && a < 6
            && a < 7
            && a < 8
            && a < 9
            && a < 10
            && a < 11
            && a < 12
            && a < 13
            && a < 14
            && a < 15) {
            return true;
        }
        return false;
    }
}
