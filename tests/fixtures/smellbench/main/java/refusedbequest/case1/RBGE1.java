package refusedbequest.case1;

enum MouseDirectionGood {
  UP("up"),
  DOWN("down"),
  LEFT("left"),
  RIGHT("right");

  private final String direction;

  MouseDirectionGood(String direction) {
    this.direction = direction;
  }

  @Override
  public String toString() {
    return direction;
  }
}
class ComputerGood {
  private String brand;
  private String model;
  private Character key;
  private boolean switchedOn;
  MouseDirectionGood mouseDirection;
  public ComputerGood(String brand, String model) {
    this.brand = brand;
    this.model = model;
    this.key = '\0';
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
  public void moveMouse(MouseDirectionGood direction) {
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

class MobileGood {
  private ComputerGood computer;
  private Character key;
  public MobileGood(String brand, String model) {
    computer = new ComputerGood(brand, model);
  }

  public void switchOnOff() {
    computer.switchOnOff();
  }

  public void printDetails() {
    computer.printDetails();
  }
  public void clickScreen(Character clickedKey) {
    this.key = clickedKey;
    System.out.println("Clicked screen at: " + key);
  }
}
