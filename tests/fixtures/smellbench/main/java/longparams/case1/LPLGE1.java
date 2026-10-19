package longparams.case1;

class BankAccountGood {
  private String accountNumber;
  private String accountType;
  private double balance;

  public BankAccountGood(String accountNumber, String accountType, double balance) {
    this.accountNumber = accountNumber;
    this.accountType = accountType;
    this.balance = balance;
  }

  public String getAccountDetails() {
    return "Account Number: " + this.accountNumber + ", Account Type: " + this.accountType + ", Balance: " + this.balance;
  }
}

class AddressGood {
  private String street;
  private String city;
  private String state;
  private String zip;

  public AddressGood(String street, String city, String state, String zip) {
    this.street = street;
    this.city = city;
    this.state = state;
    this.zip = zip;
  }

  public String getAddressDetails() {
    return "Address: " + this.street + ", " + this.city + ", " + this.state + ", " + this.zip;
  }
}

class PersonGood {
  private String name;
  private int age;
  private AddressGood address;
  private BankAccountGood bankAccount;

  public PersonGood(String name, int age, AddressGood address, BankAccountGood bankAccount) {
    this.name = name;
    this.age = age;
    this.address = address;
    this.bankAccount = bankAccount;
  }

  public String getPersonDetails() {
    return "Name: " + this.name + ", Age: " + this.age + ", " + this.address.getAddressDetails() + ", " + this.bankAccount.getAccountDetails();
  }
}