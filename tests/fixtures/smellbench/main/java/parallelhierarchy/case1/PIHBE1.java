package parallelhierarchy.case1;

interface VehicleBad {
    VehicleFactoryBad initiateProduction();
    void setNewProductionFacility(VehicleFactoryBad factory);
    void printVehicleInfo();
}

interface VehicleFactoryBad {
  void createVehicle();
  void testFunctionality();
}

abstract class AbstractVehicleBad implements VehicleBad {
    private String model;
    private String engineType;
    protected VehicleFactoryBad factory;

    public AbstractVehicleBad(String model, String engineType) {
        this.model = model;
        this.engineType = engineType;
    }
    @Override
    public VehicleFactoryBad initiateProduction() {
      return this.factory;
    }


    public void printVehicleInfo() {
        System.out.println("Model: " + model
                + ", Engine Type: " + engineType);
    }
}

abstract class AbstractVehicleFactoryBad implements VehicleFactoryBad {
    protected String factoryName;
    public AbstractVehicleFactoryBad(String name) {
        this.factoryName = name;
    }


}

class CarBad extends AbstractVehicleBad {

    public CarBad(String model, String engineType) {
        super(model, engineType);
        this.factory = new CarFactoryBad("Default " + model + " Factory");
    }

    @Override
    public void setNewProductionFacility(VehicleFactoryBad factory) {
      if (factory instanceof CarFactoryBad) {
        this.factory = factory;
      }
    }

}

class CarFactoryBad extends AbstractVehicleFactoryBad {
    public CarFactoryBad(String name) {
        super(name);
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

class BikeBad extends AbstractVehicleBad {

    public BikeBad(String model, String engineType) {
      super(model, engineType);
      this.factory = new BikeFactoryBad("Default " + model + " Factory");
    }



    @Override
    public void setNewProductionFacility(VehicleFactoryBad factory) {
      if(factory instanceof BikeFactoryBad) {
        this.factory = factory;
      }

    }
}

class BikeFactoryBad extends AbstractVehicleFactoryBad {
    public BikeFactoryBad(String name) {
        super(name);
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
