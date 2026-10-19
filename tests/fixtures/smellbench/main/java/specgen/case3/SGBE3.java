package specgen.case3;

interface GeometricObject {
    public double calculateArea();

    public void calculatePerimeter();
}

abstract class Shape implements GeometricObject {
    public abstract double getDimensions();
}

class Circle extends Shape {
    double radius;

    public Circle(double radius) {
        this.radius = radius;
    }

    public double calculateArea() {
        return Math.PI * Math.pow(this.radius, 2);
    }

    public void calculatePerimeter() {
        System.out.println("Circumference: " + 2 * Math.PI * this.radius);
    }

    @Override
    public double getDimensions() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'getPerimeter'");
    }
}
