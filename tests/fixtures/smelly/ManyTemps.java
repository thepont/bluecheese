// SMELLY ON PURPOSE, and frozen: see tests/refactored.rs for what this pins.
//
// The before half of a Replace Method with Method Object pair: a long body that
// leans on many locals, which is the shape the book uses to introduce that
// refactoring.

package smelly;

public class ManyTemps {

    public double average(int[] values) {
        double sum = 0;
        int count = 0;
        double highest = 0;
        double lowest = 0;
        double spread = 0;
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
