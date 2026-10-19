package specgen.case3;

interface GeometricObjectGood {
    public double calculateArea();

    public void calculatePerimeter();
}

class CircleGood implements specgen.case3.GeometricObjectGood {
    double radius;

    public CircleGood(double radius) {
        this.radius = radius;
    }

    @Override
    public double calculateArea() {
        return Math.PI * Math.pow(this.radius, 2);
    }

    @Override
    public void calculatePerimeter() {
        System.out.println("Circumference: " + 2 * Math.PI * this.radius);
    }

}
