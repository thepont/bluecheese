package parallelhierarchy.case1;

interface VehicleGood {
  void createVehicle();
  void testFunctionality();
  void printVehicleInfo();
}

abstract class AbstractVehicle implements VehicleGood {
    private String model;
    private String engineType;
    protected String factoryName;

    public AbstractVehicle(String model, String engineType, String factory) {
        this.model = model;
        this.engineType = engineType;
        this.factoryName = factory;
    }

    public void printVehicleInfo() {
        System.out.println("Model: " + model
                + ", Engine Type: " + engineType);
    }
}


class CarGood extends AbstractVehicle {


  public CarGood(String model, String engineType, String factory) {
    super(model, engineType, factory);

  }
  @Override
  public void createVehicle() {
    System.out.println("Car created at " + factoryName);
  }

  @Override
  public void testFunctionality() {
    System.out.println("Car tested at " + factoryName);
  }

}


class BikeGood extends AbstractVehicle {
  public BikeGood(String model, String engineType, String factory) {
    super(model, engineType, factory);
  }

  @Override
  public void createVehicle() {
    System.out.println("Bike created at " + factoryName);
  }

  @Override
  public void testFunctionality() {
    System.out.println("Bike tested at " + factoryName);
  }
}
