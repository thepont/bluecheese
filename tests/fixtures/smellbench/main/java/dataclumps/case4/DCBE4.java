package dataclumps.case4;

import java.util.Date;
import java.util.ArrayList;
import java.util.List;

class WestVillageManagement {
  private List<LeaseRecord> records;

  public WestVillageManagement() {
    records = new ArrayList<>();
  };

  public void addLeaseRecord(String tenant, Date leaseStart, Date leaseEnd, double amountInvoiced, double amountReceived) {
    records.add(new LeaseRecord(tenant, leaseStart, leaseEnd, amountInvoiced, amountReceived));
  }

  public double amountInvoiceIn(Date start, Date end) {
    double total = 0;
    for (LeaseRecord r : records) {
      if (overlaps(r.leaseStart, r.leaseEnd, start, end)) {
        total += r.amountInvoiced;
      }
    }
    return total;
  }

  public double amountReceivedIn(Date start, Date end) {
    double total = 0;
    for (LeaseRecord r : records) {
      if (overlaps(r.leaseStart, r.leaseEnd, start, end)) {
        total += r.amountReceived;
      }
    }
    return total;
  }

  public double amountOverdue(Date start, Date end) {
    double total = 0;
    for (LeaseRecord r : records) {
      if (overlaps(r.leaseStart, r.leaseEnd, start, end)) {
        total += (r.amountInvoiced - r.amountReceived);
      }
    }
    return total;
  }

  private boolean overlaps(Date start1, Date end1, Date start2, Date end2) {
    return !(end1.before(start2) || start1.after(end2));
  }

  private static class LeaseRecord {
    String tenant;
    Date leaseStart, leaseEnd;
    double amountInvoiced, amountReceived;

    LeaseRecord(String tenant, Date leaseStart, Date leaseEnd, double amountInvoiced, double amountReceived) {
      this.tenant = tenant;
      this.leaseStart = leaseStart;
      this.leaseEnd = leaseEnd;
      this.amountInvoiced = amountInvoiced;
      this.amountReceived = amountReceived;
    }
  }
}
