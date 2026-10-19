package largeclass.case1;

import static java.lang.Math.abs;

interface IAddress {
  void printAddress();
}

interface IBankAccount {
  void addDeposit(double money);
  void withdrawMoney(double money);
  void printBankDetails();
}

interface IPerson {
  void addAddress(IAddress address);
  void addBankAccount(IBankAccount bankAccount);
  void printPersonDetails();
}
class AddressGood implements IAddress {
  private String street;
  private String city;
  private String state;
  private String zipcode;

  public AddressGood() {
    this.street = "N/A";
    this.city = "N/A";
    this.state = "N/A";
    this.zipcode = "N/A";
  }
  public AddressGood(String street, String city, String state, String zipcode) {
    this.street = street;
    this.city = city;
    this.state = state;
    this.zipcode = zipcode;
  }

  @Override
  public void printAddress() {
    System.out.println("Address: " + this.street + ", " + this.city + ", " + this.state + ", " + this.zipcode);
  }
}

class BankAccountGood implements IBankAccount {
  private String bankAccountNumber;
  private double bankAccountBalance;
  private String bankAccountType;

  public BankAccountGood() {
    this.bankAccountNumber = "N/A";
    this.bankAccountBalance = 0;
    this.bankAccountType = "N/A";
  }

  public BankAccountGood(String bankAccountNumber, String bankAccountType){
    this.bankAccountNumber = bankAccountNumber;
    this.bankAccountBalance = 0;
    this.bankAccountType = bankAccountType;
  }

  public void addDeposit(double money) {
    this.bankAccountBalance += abs(money);
  }

  public void withdrawMoney(double money) {
    this.bankAccountBalance -= abs(money);
  }

  public void printBankDetails() {
    System.out.println("Bank Account: " + this.bankAccountNumber + ", " + this.bankAccountType + ", " + this.bankAccountBalance);
  }
}

class PersonGood implements IPerson {
  private String firstname;
  private String lastname;
  private IAddress address;
  private IBankAccount bankAccount;

  public PersonGood(String firstname, String lastname) {
    this.firstname = firstname;
    this.lastname = lastname;
    address = new AddressGood();
    bankAccount = new BankAccountGood();
  }

  public void addAddress(IAddress address) {
    this.address = address;
  }

  public void addBankAccount(IBankAccount bankAccount) {
    this.bankAccount = bankAccount;
  }

  public void printPersonDetails() {
    System.out.println("Name: " + this.firstname + " " + this.lastname);
    address.printAddress();
    bankAccount.printBankDetails();
  }
}
