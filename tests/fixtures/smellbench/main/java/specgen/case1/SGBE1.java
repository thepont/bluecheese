package specgen.case1;

abstract class AccomodationBad {
  abstract double bookAccomodation(int numDays);
  abstract void printAccomodationDetails();
}

class HotelBad extends AccomodationBad {
 private final String name;
 private final String address;
 private final double perDayCost;
 private int numDaysStay;

    public HotelBad(String name, String address, double perDayCost) {
    this.name = name;
    this.address = address;
    this.perDayCost = perDayCost;
    this.numDaysStay = 0;
    }

    @Override
    public double bookAccomodation(int numDays) {
    numDaysStay = numDays;
    return calculateTotalStayCost();
    }

    @Override
    public void printAccomodationDetails() {
     if(numDaysStay == 0) {
        System.out.println("Accomodation not booked yet");
        return;
     }
       System.out.println("Hotel Name: " + name);
       System.out.println("Hotel Address: " + address);
       System.out.println("Total Cost: " + calculateTotalStayCost());
       System.out.println("Number of days stay: " + numDaysStay);
    }

    private double calculateTotalStayCost() {
        return Math.round(numDaysStay * perDayCost);
    }
}