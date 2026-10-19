package dataclass.case2;

class Plane {
  private double width, height, xOffset, yOffset;

  public Plane(double width, double height, double xOffset, double yOffset) {
    if (!Double.isFinite(width) || width < 0) {
      throw new IllegalArgumentException("width must be finite and >= 0");
    }
    if (!Double.isFinite(height) || height < 0) {
      throw new IllegalArgumentException("height must be finite and >= 0");
    }
    if (!Double.isFinite(xOffset)) {
      throw new IllegalArgumentException("xOffset must be finite");
    }
    if (!Double.isFinite(yOffset)) {
      throw new IllegalArgumentException("yOffset must be finite");
    }

    this.width   = (width == -0.0)  ? 0.0 : width;
    this.height  = (height == -0.0) ? 0.0 : height;
    this.xOffset = (xOffset == -0.0) ? 0.0 : xOffset;
    this.yOffset = (yOffset == -0.0) ? 0.0 : yOffset;
  }

  public double getWidth() {
    return width;
  }
  public void setWidth(double width) {
    this.width = width;
  }

  public double getHeight() {
    return height;
  }
  public void setHeight(double height) {
    this.height = height;
  }

  public double getXOffset() {
    return xOffset;
  }
  public void setXOffset(double xOffset) {
    this.xOffset = xOffset;
  }

  public double getYOffset() {
    return yOffset;
  }
  public void setYOffset(double yOffset) {
    this.yOffset = yOffset;
  }
}

class PartStudio {
  public double calculateVolume(Plane plane, double thickness) {
    if (plane == null) {
      throw new IllegalArgumentException("plane cannot be null");
    }
    if (thickness < 0) {
      throw new IllegalArgumentException("thickness cannot be negative");
    }
    if (plane.getWidth() < 0 || plane.getHeight() < 0) {
      throw new IllegalArgumentException("dataclass.case2.Plane width/height cannot be negative");
    }
    return plane.getWidth() * plane.getHeight() * thickness;
  }
}
