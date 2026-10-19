mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("date_helpers.java", source);
    bluecheese::detect(&[file], "incomp-lib-class").expect("run Incomplete Library Class")
}

#[test]
fn reports_repeated_helpers_extending_the_same_imported_type() {
    let findings = scan(
        "import vendor.Calendar;\n\
         class CalendarExtensions {\n\
             static boolean isWeekend(Calendar calendar) { return calendar.getDay() > 5; }\n\
             static Calendar nextBusinessDay(Calendar calendar) { return calendar.plusDays(1); }\n\
         }",
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].smell_name, "Incomplete Library Class");
    assert_eq!(findings[0].class_name, "CalendarExtensions");
    assert_eq!(findings[0].line_numbers, [2]);
    assert!(findings[0].additional_info.contains("Calendar"));
}

#[test]
fn ignores_one_off_helpers_and_helpers_for_local_types() {
    let one_off = scan(
        "import vendor.Calendar;\n\
         class CalendarExtensions {\n\
             static boolean isWeekend(Calendar calendar) { return calendar.getDay() > 5; }\n\
         }",
    );
    let local_type = scan(
        "class Calendar { int getDay() { return 1; } }\n\
         class CalendarExtensions {\n\
             static boolean isWeekend(Calendar calendar) { return calendar.getDay() > 5; }\n\
             static int day(Calendar calendar) { return calendar.getDay(); }\n\
         }",
    );

    assert!(one_off.is_empty());
    assert!(local_type.is_empty());
}
