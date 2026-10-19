package dupcode.case3;

class SimpleCalculatorGood {
    int a;
    int b;

    public SimpleCalculatorGood(int a, int b) {
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
}