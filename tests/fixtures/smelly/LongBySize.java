// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// Size alone, in a different shape from LongMethod's: statements that are all
// calls rather than assignments. One invocation per line, so the logical line
// count is the number of lines below.
package smelly;

public class LongBySize {

    private final StringBuilder log = new StringBuilder();

    public void record() {
        log.append(1);
        log.append(2);
        log.append(3);
        log.append(4);
        log.append(5);
        log.append(6);
        log.append(7);
        log.append(8);
        log.append(9);
        log.append(10);
        log.append(11);
        log.append(12);
        log.append(13);
        log.append(14);
        log.append(15);
        log.append(16);
        log.append(17);
        log.append(18);
        log.append(19);
        log.append(20);
        log.append(21);
        log.append(22);
        log.append(23);
        log.append(24);
        log.append(25);
        log.append(26);
        log.append(27);
        log.append(28);
        log.append(29);
        log.append(30);
        log.append(31);
    }
}
