package largeclass.case1;

import static java.lang.Math.abs;

class PersonBad {
  private String firstname;
  private String lastname;
  private String street;
  private String city;
  private String state;
  private String zipcode;
  private String bankAccountNumber;
  private double bankAccountBalance;
  private String bankAccountType;

  public PersonBad(String firstname, String lastname) {
    this.firstname = firstname;
    this.lastname = lastname;
    this.street = "N/A";
    this.city = "N/A";
    this.state = "N/A";
    this.zipcode = "N/A";
    this.bankAccountNumber = "N/A";
    this.bankAccountBalance = 0;
    this.bankAccountType = "N/A";
  }

  public void addAddress(String street, String city, String state, String zipcode) {
    this.street = street;
    this.city = city;
    this.state = state;
    this.zipcode = zipcode;
  }

  public void addBankAccount(String bankAccountNumber, String bankAccountType) {
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

  public void printPersonDetails() {
    System.out.println("Name: " + this.firstname + " " + this.lastname);
    System.out.println("Address: " + this.street + ", " + this.city + ", " + this.state + ", " + this.zipcode);
    System.out.println("Bank Account: " + this.bankAccountNumber + ", " + this.bankAccountType + ", " + this.bankAccountBalance);
  }
}