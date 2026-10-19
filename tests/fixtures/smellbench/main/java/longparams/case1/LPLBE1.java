package longparams.case1;

class BankAccountBad {
    private String accountNumber;
    private String accountType;
    private double balance;

    public BankAccountBad(String accountNumber, String accountType, double balance) {
        this.accountNumber = accountNumber;
        this.accountType = accountType;
        this.balance = balance;
    }

    public String getAccountDetails() {
        return "Account Number: " + this.accountNumber + ", Account Type: " + this.accountType + ", Balance: " + this.balance;
    }
}

class AddressBad {
    private String street;
    private String city;
    private String state;
    private String zip;

    public AddressBad(String street, String city, String state, String zip) {
        this.street = street;
        this.city = city;
        this.state = state;
        this.zip = zip;
    }

    public String getAddressDetails() {
        return "Address: " + this.street + ", " + this.city + ", " + this.state + ", " + this.zip;
    }
}

class PersonBad {
    private String name;
    private int age;
    private AddressBad address;
    private BankAccountBad bankAccount;

    public PersonBad(String name, int age, String street, String city, String state, String zip, String accountNumber, String accountType, double balance) {
        this.name = name;
        this.age = age;
        this.address = new AddressBad(street, city, state, zip);
        this.bankAccount = new BankAccountBad(accountNumber, accountType, balance);
    }

    public String getPersonDetails() {
        return "Name: " + this.name + ", Age: " + this.age + ", " + this.address.getAddressDetails() + ", " + this.bankAccount.getAccountDetails();
    }
}