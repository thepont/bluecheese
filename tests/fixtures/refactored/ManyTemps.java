// The after half of the pair whose before half is
// tests/fixtures/smelly/ManyTemps.java, and frozen.
//
// Replace Method with Method Object: the locals become fields of a new object,
// and the method that used to hold the body is now one line.
//
// This refactoring does NOT silence the detector, and this fixture exists to
// make sure nobody makes it look as though it does. The long body has moved into
// the new object's compute(), where it is still long, and it is now reported
// under ManyTempsCalculator instead of ManyTemps. The book is explicit that the
// method object is a step and not the cure: the cure is the Extract Method you
// then apply inside compute(). A detector that went quiet here would be lying
// about where the smell went.

package refactored;

public class ManyTemps {

    public double average(int[] values) {
        return new ManyTempsCalculator(values).compute();
    }
}

class ManyTempsCalculator {

    private final int[] values;
    private double sum = 0;
    private int count = 0;
    private double highest = 0;
    private double lowest = 0;
    private double spread = 0;

    ManyTempsCalculator(int[] values) {
        this.values = values;
    }

    double compute() {
        sum += values[0];
        sum += values[1];
        sum += values[2];
        sum += values[3];
        sum += values[4];
        sum += values[5];
        sum += values[6];
        sum += values[7];
        sum += values[8];
        sum += values[9];
        sum += values[10];
        sum += values[11];
        sum += values[12];
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        highest = values[0];
        highest = values[1];
        highest = values[2];
        highest = values[3];
        lowest = values[4];
        lowest = values[5];
        lowest = values[6];
        lowest = values[7];
        spread = highest - lowest;
        spread += 1;
        spread += 2;
        return (sum / count) + spread;
    }
}
