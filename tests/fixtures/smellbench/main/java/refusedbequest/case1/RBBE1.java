package refusedbequest.case1;

enum MouseDirectionBad {
    UP("up"),
    DOWN("down"),
    LEFT("left"),
    RIGHT("right");

    private final String direction;

    MouseDirectionBad(String direction) {
        this.direction = direction;
    }

    @Override
    public String toString() {
        return direction;
    }
}
class ComputerBad {
    protected String brand;
    protected String model;
    protected Character key;

    protected MouseDirectionBad mouseDirection;
    protected boolean switchedOn;

    public ComputerBad(String brand, String model) {
        this.brand = brand;
        this.model = model;
        this.key = '\0';
        this.switchedOn = false;
    }
    public void printDetails(){
        System.out.println("Brand: " + brand + " Model: " + model);
    }
    public void switchOnOff() {
      System.out.println(switchedOn? turnOff() : turnOn());
    }
    public void clickKeyboard(Character clickedKey) {
      this.key = clickedKey;
      System.out.println("Clicked key: " + key);
    }
    public void moveMouse(MouseDirectionBad direction) {
      this.mouseDirection = direction;
      System.out.println("Mouse moved to: " + mouseDirection.toString());
    }
    private String turnOn() {
      switchedOn = true;
      return "Turning on.";
    }
    private String turnOff() {
      switchedOn = false;
      return "Turning off.";
    }
}

class MobileBad extends ComputerBad {
    public MobileBad(String brand, String model) {
        super(brand, model);
    }

    @Override
    public void clickKeyboard(Character clickedKey) {
        return;
    }
    @Override
    public void moveMouse(MouseDirectionBad direction) {
      return;
    }
    public void clickScreen(Character clickedKey) {
        this.key = clickedKey;
        System.out.println("Clicked screen at: " + key);
    }
}