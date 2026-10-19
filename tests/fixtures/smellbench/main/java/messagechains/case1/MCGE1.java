package messagechains.case1;

import java.util.ArrayList;
import java.util.List;

class AddressGood {
  private String street;
  private String city;
  private String zipCode;

  public AddressGood(String street, String city, String zipCode) {
    this.street = street;
    this.city = city;
    this.zipCode = zipCode;
  }

  public String getAddress() {
    return street + ", " + city + ", " + zipCode;
  }
}

class PersonGood {
  private String salutation;
  private String name;
  private AddressGood address;

  public PersonGood(String salutation, String name, AddressGood address) {
    this.salutation = salutation;
    this.name = name;
    this.address = address;
  }

  public String getName() {
    return salutation + " " + name;
  }

  public String getAddress() {
    return address.getAddress();
  }
}

class OrderGood {
  private PersonGood person;
  private int orderNumber;
  private List<String> items;
  private String orderStatus;
  private double orderTotal;
  public OrderGood(PersonGood person, int orderNumber) {
    this.person = person;
    this.orderNumber = orderNumber;
    this.items = new ArrayList<String>();
    this.orderStatus = "Pending";
    this.orderTotal = 0;
  }

  public void addItem(String item) {
    orderTotal+=10;
    items.add(item);
  }

  public void placeOrder() {
    orderStatus = "Placed";
    System.out.println("Order placed successfully. Cost: " + orderTotal);
  }

  public void printOrderDetails() {
    System.out.println("Order Number: " + orderNumber);
    System.out.println("Order Status: " + orderStatus);
    System.out.println("Order Total: " + orderTotal);
    System.out.println("Items: ");
    for(String item : items) {
      System.out.println(item);
    }
    System.out.println("Person: " + person.getName());
    System.out.println("Address: " + person.getAddress());
  }
}

class OrderHistoryGood {
  private List<OrderGood> orders;

  public OrderHistoryGood() {
    this.orders = new ArrayList<OrderGood>();
  }

  public void addOrder(OrderGood order) {
    orders.add(order);
  }

  public void printOrderHistory() {
    for(OrderGood order : orders) {
      order.printOrderDetails();
    }
  }
}