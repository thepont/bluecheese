package tempfield.case2;

import java.util.ArrayList;
import java.util.List;

enum ServiceTypeGood {
  MOBILE,
  INTERNET,
  CABLE_TV
}

class CustomerGood {
  private String name;
  private String email;
  private List<tempfield.case2.ServiceTypeGood> services = new ArrayList<>();

  public CustomerGood(String name, String email) {
    this.name = name;
    this.email = email;
  }

  public CustomerGood() {}

  public String getName() { return name; }
  public String getEmail() { return email; }

  public String getProfile() {
    return "Customer Name: " + getName() + ", Email: " + getEmail();
  }

  public void registerService(tempfield.case2.ServiceTypeGood service) {
    services.add(service);
  }

  public List<tempfield.case2.ServiceTypeGood> getServices() {
    return services;
  }
}

class NullCustomer extends tempfield.case2.CustomerGood {
  public NullCustomer() {
    super();
  }

  public boolean isNull() {
    return true;
  }
}

class VerizonUserManagementGood {
  private List<tempfield.case2.CustomerGood> mobileCustomers = new ArrayList<>();
  private List<tempfield.case2.CustomerGood> internetCustomers = new ArrayList<>();
  private List<tempfield.case2.CustomerGood> cableTVCustomers = new ArrayList<>();

  public void registerMobileCustomer(tempfield.case2.CustomerGood customer) {
    if (customer instanceof NullCustomer) {
      mobileCustomers.add(new NullCustomer());
    } else {
      mobileCustomers.add(customer);
      customer.registerService(tempfield.case2.ServiceTypeGood.MOBILE);
    }
  }

  public void registerInternetCustomer(tempfield.case2.CustomerGood customer) {
    if (customer instanceof NullCustomer) {
      internetCustomers.add(new NullCustomer());
    } else {
      internetCustomers.add(customer);
      customer.registerService(tempfield.case2.ServiceTypeGood.INTERNET);
    }
  }

  public void registerCableTVCustomer(tempfield.case2.CustomerGood customer) {
    if (customer instanceof NullCustomer) {
      cableTVCustomers.add(new NullCustomer());
    } else {
      cableTVCustomers.add(customer);
      customer.registerService(tempfield.case2.ServiceTypeGood.CABLE_TV);
    }
  }

  public void printAllCustomers() {
    System.out.println("=== MOBILE CUSTOMERS ===");
    for (tempfield.case2.CustomerGood c : mobileCustomers) {
      System.out.println(c.getProfile());
    }

    System.out.println("=== INTERNET CUSTOMERS ===");
    for (tempfield.case2.CustomerGood c : internetCustomers) {
      System.out.println(c.getProfile());
    }

    System.out.println("=== CABLE TV CUSTOMERS ===");
    if (cableTVCustomers != null) {
      for (tempfield.case2.CustomerGood c : cableTVCustomers) {
        System.out.println(c.getProfile());
      }
    } else {
      System.out.println("No Cable TV customers.");
    }
  }
}