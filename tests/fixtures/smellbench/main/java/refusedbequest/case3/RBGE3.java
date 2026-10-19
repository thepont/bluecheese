package refusedbequest.case3;

enum MaterialGood {
  CLAY,
  CONCRETE,
  PLASTIC,
  SANDSTONE,
  STONE,
  CEMENT
}

class BlockGood {
  private final double length;
  private final double width;
  private final double height;

  public BlockGood(double length, double width, double height) {
    if (length <= 0 || width <= 0 || height <= 0) {
      throw new IllegalArgumentException("All dimensions must be positive");
    }
    this.length = length;
    this.width = width;
    this.height = height;
  }

  public double getLength() {
    return length;
  }

  public double getWidth() {
    return width;
  }

  public double getHeight() {
    return height;
  }

  public double getVolume() {
    return length * width * height;
  }

  public double getSurfaceArea() {
    return 2 * (length * width + length * height + width * height);
  }
}

class BuildingBrickGood extends refusedbequest.case3.BlockGood {
  private final refusedbequest.case3.MaterialGood material;

  public BuildingBrickGood(refusedbequest.case3.MaterialGood material, double length, double width, double height) {
    super(length, width, height);
    if (material == null) {
      throw new IllegalArgumentException("material must not be null");
    }
    this.material = material;
  }

  public refusedbequest.case3.MaterialGood getMaterial() {
    return material;
  }

  public boolean canClickWith(refusedbequest.case3.BuildingBrickGood other) {
    if (other == null) return false;
    return this.material == other.material
            && Double.compare(getLength(), other.getLength()) == 0
            && Double.compare(getWidth(),  other.getWidth())  == 0
            && Double.compare(getHeight(), other.getHeight()) == 0;
  }
}

class LegoPartGood extends refusedbequest.case3.BlockGood {
  private final String partId;
  private final int studCount;
  private final String color;

  public LegoPartGood(String partId,
                      double length,
                      double width,
                      double height,
                      int studCount,
                      String color) {
    super(length, width, height);
    if (partId == null || partId.isBlank())
      throw new IllegalArgumentException("partId required");
    if (studCount < 0)
      throw new IllegalArgumentException("studCount must be ≥0");
    if (color == null || color.isBlank())
      throw new IllegalArgumentException("color required");
    this.partId    = partId;
    this.studCount = studCount;
    this.color     = color;
  }

  public String getPartId()    { return partId; }
  public int    getStudCount() { return studCount; }
  public String getColor()     { return color; }

  public boolean fitsOnTopOf(refusedbequest.case3.LegoPartGood other) {
    if (other == null) return false;
    return this.getLength() <= other.getLength()
            && this.getWidth()  <= other.getWidth();
  }

  public boolean canAttachTo(refusedbequest.case3.LegoPartGood other) {
    return other != null
            && this.studCount == other.studCount
            && fitsOnTopOf(other);
  }

  public void attachTo(refusedbequest.case3.LegoPartGood other) {
    if (!canAttachTo(other)) {
      throw new IllegalStateException(
              partId + " cannot attach to " + (other == null ? "null" : other.partId));
    }
    System.out.println("Attached " + partId + " onto " + other.partId);
  }

  public String getMarketingTagline() {
    return String.format(
            "Unleash creativity with %s – %d studs, vibrant %s!",
            partId, studCount, color
    );
  }

  public double calculateRetailPrice(double costPrice, double profitMarginPercent) {
    if (costPrice < 0 || profitMarginPercent < 0)
      throw new IllegalArgumentException("Costs and margins must be non-negative");
    return costPrice * (1 + profitMarginPercent / 100.0);
  }

  public String generateEmailSubject() {
    return String.format(
            "New Arrival: %s in %s – %d-stud fun awaits!",
            partId, color, studCount
    );
  }
}