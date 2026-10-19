package largeclass.case4;

class CalculatorVariation {
    private int a, b;

    public CalculatorVariation(int a, int b) {
        this.a = a;
        this.b = b;
    }
    public int add() {
        return a + b;
    }

    public int subtract() {
        return a - b;
    }

    public float divide() {
        return a / b;
    }

    public float multiply() {
        return a * b;
    }
}

class TrigonometricCalculatorVariation {
    private double a;

    public TrigonometricCalculatorVariation(double a) {
        this.a = a;
    }

    public double sine() {
        return Math.sin(a);
    }

    public double cosine() {
        return Math.cos(a);
    }

    public double tan() {
        return Math.tan(a);
    }

    public double inverseTan() {
        return Math.tanh(a);
    }
}
