package refusedbequest.case4;

interface VehicleGood {
    public void forward();

    public void reverse();

    public void turn(String direction);
}

interface GasVehicleGood extends refusedbequest.case4.VehicleGood {
    public void refuel();

    public void replaceEngine(String newEngine);
}

class CarGood implements refusedbequest.case4.GasVehicleGood {
    private int fuelPercentage;
    private int distanceTravelled;
    private String direction;
    private String engine;

    public CarGood(String engine) {
        this.fuelPercentage = 100;
        this.distanceTravelled = 0;
        this.direction = "Forward";
        this.engine = engine;
    }

    @Override
    public void forward() {
        this.distanceTravelled += 10;
        this.fuelPercentage -= 1;
        System.out.println("New distance travelled: " + this.distanceTravelled);
        System.out.println("Current fuel percentage: " + this.fuelPercentage);
    }

    @Override
    public void reverse() {
        this.distanceTravelled -= 10;
        this.fuelPercentage -= 1;
        System.out.println("New distance travelled: " + this.distanceTravelled);
        System.out.println("Current fuel percentage: " + this.fuelPercentage);
    }

    @Override
    public void turn(String direction) {
        this.direction = direction;
        System.out.println("New direction: " + this.direction);
    }

    public void refuel() {
        if (this.fuelPercentage == 100) {
            System.out.println("Fuel is full");
        } else {
            this.fuelPercentage = 100;
        }
    }

    public void replaceEngine(String newEngine) {
        this.engine = newEngine;
        System.out.println("New engine: " + this.engine);
    }

}

class BicycleGood implements refusedbequest.case4.VehicleGood {

    private int distanceTravelled;
    private String direction;

    public BicycleGood() {
        this.distanceTravelled = 0;
        this.direction = "Forward";
    }

    @Override
    public void forward() {
        this.distanceTravelled += 10;
        System.out.println("New distance travelled: " + this.distanceTravelled);
    }

    @Override
    public void reverse() {
        this.distanceTravelled -= 10;
        System.out.println("New distance travelled: " + this.distanceTravelled);
    }

    @Override
    public void turn(String direction) {
        this.direction = direction;
        System.out.println("New direction: " + this.direction);
    }
}
