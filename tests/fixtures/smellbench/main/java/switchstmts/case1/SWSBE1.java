package switchstmts.case1;

class ShapeBad {
  private String shape;
  public ShapeBad(String shape) {
    this.shape = shape;
  }

  public double calculateArea(int ... dimensions) {
    switch(shape) {
      case "circle":
        return Math.PI * dimensions[0] * dimensions[0];
      case "rectangle":
        return dimensions[0] * dimensions[1];
      case "triangle":
        return 0.5 * dimensions[0] * dimensions[1];
      case "square":
        return dimensions[0] * dimensions[0];
      default:
        return 0;
    }
  }
}