package messagechains.case1;

import java.util.ArrayList;
import java.util.List;

class AddressBad {
    private String street;
    private String city;
    private String zipCode;

    public AddressBad(String street, String city, String zipCode) {
        this.street = street;
        this.city = city;
        this.zipCode = zipCode;
    }

    public String getAddress() {
        return street + ", " + city + ", " + zipCode;
    }
}

class PersonBad {
    private String salutation;
    private String name;
    private AddressBad address;

    public PersonBad(String salutation, String name, AddressBad address) {
        this.salutation = salutation;
        this.name = name;
        this.address = address;
    }

    public String getName() {
        return salutation + " " + name;
    }

    public AddressBad getAddress() {
        return address;
    }
}

class OrderBad {
    private PersonBad person;
    private int orderNumber;
    private List<String> items;
    private String orderStatus;
    private double orderTotal;
    public OrderBad(PersonBad person, int orderNumber) {
        this.person = person;
        this.orderNumber = orderNumber;
        this.items = new ArrayList<String>();
        this.orderStatus = "Pending";
        this.orderTotal = 0;
    }

    public PersonBad getPerson() {
        return person;
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
    }
}

class OrderHistoryBad {
    private List<OrderBad> orders;

    public OrderHistoryBad() {
        this.orders = new ArrayList<OrderBad>();
    }

    public void addOrder(OrderBad order) {
        orders.add(order);
    }

    public void printOrderHistory() {
        for(OrderBad order : orders) {
            order.printOrderDetails();
            System.out.println("Person: " + order.getPerson().getName());
            System.out.println("Address: " + order.getPerson().getAddress().getAddress());
        }
    }
}