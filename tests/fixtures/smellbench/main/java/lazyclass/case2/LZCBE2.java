package lazyclass.case2;

class PrintGreetingsBad {
    private String userName;
    public PrintGreetingsBad(String userName) {
        this.userName = userName;
    }
    public void printGreetings() {
        System.out.println("Hello, " + userName + "!");
    }
}

class PrintHelloUserBad extends PrintGreetingsBad {
    public PrintHelloUserBad(String userName) {
        super(userName);
    }

    public void printHelloUser() {
        printGreetings();
    }

}