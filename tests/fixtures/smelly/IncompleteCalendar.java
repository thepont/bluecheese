import vendor.Calendar;

class CalendarExtensions {
    static boolean isWeekend(Calendar calendar) {
        return calendar.getDay() > 5;
    }
}

class WeekendPolicy {
    static boolean shouldSkip(Calendar calendar) {
        return calendar.getDay() > 5;
    }
}
