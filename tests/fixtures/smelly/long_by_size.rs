// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// Size alone, in the shape Rust writes most often: a free function. It belongs
// to no type, so it is reported under a pseudo-type named after the file.
//
// There is no reference implementation for Rust smells, so this file pins *our*
// definition of a statement and not Spoon's: one statement is one named child
// of a block, and the logical line count is the count of them.

pub fn apply_shipment(weights: &[i32]) -> i32 {
    let mut running_total = 0;
    running_total += weights[0];
    running_total += weights[1];
    running_total += weights[2];
    running_total += weights[3];
    running_total += weights[4];
    running_total += weights[5];
    running_total += weights[6];
    running_total += weights[7];
    running_total += weights[8];
    running_total += weights[9];
    running_total += weights[10];
    running_total += weights[11];
    running_total += weights[12];
    running_total += weights[13];
    running_total += weights[14];
    running_total += weights[15];
    running_total += weights[16];
    running_total += weights[17];
    running_total += weights[18];
    running_total += weights[19];
    running_total += weights[20];
    running_total += weights[21];
    running_total += weights[22];
    running_total += weights[23];
    running_total += weights[24];
    running_total += weights[25];
    running_total += weights[26];
    running_total += weights[27];
    running_total += weights[28];
    running_total += weights[29];
    running_total
}
