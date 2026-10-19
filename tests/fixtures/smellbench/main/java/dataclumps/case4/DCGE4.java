package dataclumps.case4;

import java.util.Date;
import java.util.ArrayList;
import java.util.List;

class DateRange {
  private Date start, end;

  public DateRange(Date start, Date end) {
    this.start = start;
    this.end = end;
  }

  public Date getStart() { return this.start; }
  public Date getEnd() { return this.end; }

  public boolean overlaps(DateRange otherDateRange) {
    return !(this.end.before(otherDateRange.getStart()) || this.start.after(otherDateRange.getEnd()));
  }
}

class WestVillageManagementVariation {
  private List<LeaseRecord> records = new ArrayList<>();

  public WestVillageManagementVariation() {};

  public void addLeaseRecord(String tenant, DateRange leaseRange, double amountInvoiced, double amountReceived) {
    records.add(new LeaseRecord(tenant, leaseRange, amountInvoiced, amountReceived));
  }

  public double amountInvoiceIn(DateRange dateRange) {
    double total = 0;
    for (LeaseRecord r : records) {
      if (r.leaseRange.overlaps(dateRange)) {
        total += r.amountInvoiced;
      }
    }
    return total;
  }

  public double amountReceivedIn(DateRange dateRange) {
    double total = 0;
    for (LeaseRecord r : records) {
      if (r.leaseRange.overlaps(dateRange)) {
        total += r.amountReceived;
      }
    }
    return total;
  }

  public double amountOverdue(DateRange dateRange) {
    double total = 0;
    for (LeaseRecord r : records) {
      if (r.leaseRange.overlaps(dateRange)) {
        total += (r.amountInvoiced - r.amountReceived);
      }
    }
    return total;
  }

  private static class LeaseRecord {
    String tenant;
    DateRange leaseRange;
    double amountInvoiced, amountReceived;

    LeaseRecord(String tenant, DateRange leaseRange, double amountInvoiced, double amountReceived) {
      this.tenant = tenant;
      this.leaseRange = leaseRange;
      this.amountInvoiced = amountInvoiced;
      this.amountReceived = amountReceived;
    }
  }
}
