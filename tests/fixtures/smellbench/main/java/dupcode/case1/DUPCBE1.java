package dupcode.case1;

class BankAccountBad {
  private double balance;
  private String owner;
  private String accountNumber;

  public BankAccountBad(double balance, String owner, String accountNumber) {
    this.balance = balance;
    this.owner = owner;
    this.accountNumber = accountNumber;
  }

  public void deposit(double amount) {
    balance += amount;
    System.out.println("Owner: " + owner);
    System.out.println("Account number: " + accountNumber);
    System.out.println("Balance: " + balance);
  }

  public void withdraw(double amount) {
    balance -= amount;
    System.out.println("Owner: " + owner);
    System.out.println("Account number: " + accountNumber);
    System.out.println("Balance: " + balance);
  }

  public void transfer(double amount, BankAccountBad otherAccount) {
    balance -= amount;
    otherAccount.balance += amount;
    System.out.println("Owner: " + owner);
    System.out.println("Account number: " + accountNumber);
    System.out.println("Balance: " + balance);
  }
}
