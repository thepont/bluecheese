package lazyclass.case2;

class PrintGreetingsGood {
  private String userName;
  public PrintGreetingsGood(String userName) {
    this.userName = userName;
  }
  public void printGreetings() {
    System.out.println("Hello, " + userName + "!");
  }
}
