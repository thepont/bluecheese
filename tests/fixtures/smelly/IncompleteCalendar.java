import vendor.Calendar;

class CalendarExtensions {
    static boolean isWeekend(Calendar calendar) {
        return calendar.getDay() > 5;
    }

    static Calendar nextBusinessDay(Calendar calendar) {
        return calendar.plusDays(1);
    }
}
