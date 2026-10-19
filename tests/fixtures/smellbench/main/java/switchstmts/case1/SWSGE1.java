package switchstmts.case1;

abstract class ShapeGood {
    abstract double calculateArea();
}

class CircleGood extends ShapeGood {
    private double radius;

    public CircleGood(double radius) {
        this.radius = radius;
    }

    @Override
    double calculateArea() {
        return Math.PI * radius * radius;
    }
}

class RectangleGood extends ShapeGood {
    private double length;
    private double breadth;

    public RectangleGood(double length, double breadth) {
        this.length = length;
        this.breadth = breadth;
    }

    @Override
    double calculateArea() {
        return length * breadth;
    }
}

class TriangleGood extends ShapeGood {
    private double base;
    private double height;

    public TriangleGood(double base, double height) {
        this.base = base;
        this.height = height;
    }

    @Override
    double calculateArea() {
        return 0.5 * base * height;
    }
}

class SquareGood extends ShapeGood {
    private double side;

    public SquareGood(double side) {
        this.side = side;
    }

    @Override
    double calculateArea() {
        return side * side;
    }
}
