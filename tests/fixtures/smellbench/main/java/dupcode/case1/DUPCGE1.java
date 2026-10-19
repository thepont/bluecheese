package dupcode.case1;

class BankAccountGood {
  private double balance;
  private String owner;
  private String accountNumber;

  public BankAccountGood(double balance, String owner, String accountNumber) {
    this.balance = balance;
    this.owner = owner;
    this.accountNumber = accountNumber;
  }

  public void deposit(double amount) {
    balance += amount;
    printAccountDetails();
  }

  public void withdraw(double amount) {
    balance -= amount;
    printAccountDetails();
  }

  public void transfer(double amount, BankAccountGood otherAccount) {
    balance -= amount;
    otherAccount.balance += amount;
    printAccountDetails();
  }

  private void printAccountDetails() {
    System.out.println("Owner: " + owner);
    System.out.println("Account number: " + accountNumber);
    System.out.println("Balance: " + balance);
  }
}