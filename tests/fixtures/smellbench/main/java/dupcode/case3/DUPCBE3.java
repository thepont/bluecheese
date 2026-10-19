package dupcode.case3;

class SimpleCalculator {
    int a;
    int b;

    public SimpleCalculator(int a, int b) {
        this.a = a;
        this.b = b;
    }

    public int Add() {
        return a + b;
    }

    public int Subtract() {
        return a - b;
    }

    public int Multiply() {
        return a * b;
    }

    public int Divide() {
        return a / b;
    }

    public int Sum() {
        int sum = a + b;
        return sum;
    }
}