package dataclumps.case1;

import java.time.LocalDate;

class ContactGood {
  private String customerEmail;
  private String customerNumber;

  public ContactGood(String customerEmail, String customerNumber) {
    this.customerEmail = customerEmail;
    this.customerNumber = customerNumber;
  }

  public void updateContactDetails(ContactGood contactOther) {
    this.customerEmail = contactOther.customerEmail;
    this.customerNumber = contactOther.customerNumber;
  }

  public void printContactDetails() {
    System.out.println("Customer Email: " + customerEmail);
    System.out.println("Customer Number: " + customerNumber);
  }
}

class CustomerGood {
    private String customerFirstName;
    private String customerLastName;
    private ContactGood contact;

    public CustomerGood(String customerFirstName, String customerLastName, ContactGood contact) {
        this.customerFirstName = customerFirstName;
        this.customerLastName = customerLastName;
        this.contact = contact;
    }

    public boolean verifyCustomerName(CustomerGood customerOther) {
        return this.customerFirstName.equals(customerOther.customerFirstName) && this.customerLastName.equals(customerOther.customerLastName);
    }

    public void updateContactDetails(ContactGood contactOther) {
        this.contact.updateContactDetails(contactOther);
    }

    public void printCustomerDetails() {
        System.out.println("Customer Name: " + customerFirstName + " " + customerLastName);
        contact.printContactDetails();
    }
}
class BookingReferenceGood {
  private CustomerGood customer;

  private String PNR;

  public BookingReferenceGood(CustomerGood customer, String PNR) {
      this.customer = customer;
      this.PNR = PNR;
  }

  public boolean verifyBooking(BookingReferenceGood bookingDetails) {
      return this.PNR.equals(bookingDetails.PNR) && this.customer.verifyCustomerName(bookingDetails.customer);
  }

  public void updateContactDetails(ContactGood contactOther) {
    this.customer.updateContactDetails(contactOther);
  }

  public void printBookingDetails() {
    customer.printCustomerDetails();
    System.out.println("PNR: " + PNR);
  }
}
class TravelPlaceGood {
  private String placeName;
  private LocalDate flightDate;

  public TravelPlaceGood(String placeName, LocalDate flightDate) {
    this.placeName = placeName;
    this.flightDate = flightDate;
  }

  public void printTravelDetails() {
    System.out.println("Place Name: " + placeName);
    System.out.println("Flight Date: " + flightDate);
  }
}

class FlightBookingGood {
  private BookingReferenceGood bookingDetails;
  private String flightNumber;
  private TravelPlaceGood origin;
  private TravelPlaceGood returnOrigin;

  public FlightBookingGood(BookingReferenceGood bookingDetails, String flightNumber, TravelPlaceGood origin, TravelPlaceGood returnOrigin) {
    this.bookingDetails = bookingDetails;
    this.flightNumber = flightNumber;
    this.origin = origin;
    this.returnOrigin = returnOrigin;
  }

  public void updateCustomerContactDetails(BookingReferenceGood bookingOther, ContactGood contactOther) {

    if(!verifyBookingReference(bookingOther)) {
      throw new IllegalArgumentException("Cannot update contact. Customer details do not match.");
    }

    bookingDetails.updateContactDetails(contactOther);
  }

  public void updateOriginDetails(BookingReferenceGood bookingOther, TravelPlaceGood originOther) {
    if(!verifyBookingReference(bookingOther)) {
      throw new IllegalArgumentException("Cannot update origin. Customer details do not match.");
    }

    this.origin = originOther;
  }

  public void updateReturnOriginDetails(BookingReferenceGood bookingOther, TravelPlaceGood returnOriginOther) {
    if(!verifyBookingReference(bookingOther)) {
      throw new IllegalArgumentException("Cannot update return. Customer details do not match.");
    }
    this.returnOrigin = returnOriginOther;
  }

  public void printFlightDetails() {
    bookingDetails.printBookingDetails();
    System.out.println("Flight Number: " + flightNumber);
    System.out.println("Origin Details:");
    origin.printTravelDetails();
    System.out.println("Return Details:");
    returnOrigin.printTravelDetails();
  }

  private boolean verifyBookingReference(BookingReferenceGood bookingOther) {
    return this.bookingDetails.verifyBooking(bookingOther);
  }
}