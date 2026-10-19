package comments.case3;

/**
 * This class represents a user address.
 **/
class UserAddressBad {
  // Variables are final so that they can be initialized only once
  private final String street;
  private final String city;
  private final String state;

  public UserAddressBad(String street, String city, String state) {
      this.street = street;
      this.city = city;
      this.state = state;
  }

  // Code to print the address of the user in a predefined format
  public void printFormattedAddress() {
    System.out.println("The address of this user is " + street + ", " + city + ", " + state);
  }
}
