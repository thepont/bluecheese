mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("date_helpers.java", source);
    bluecheese::detect(&[file], "incomp-lib-class").expect("run Incomplete Library Class")
}

#[test]
fn reports_repeated_helper_implementations_for_the_same_imported_type() {
    let findings = scan(
        "import vendor.Calendar;\n\
         class WeekendExtensions {\n\
             static boolean isWeekend(Calendar calendar) { return calendar.getDay() > 5; }\n\
         }\n\
         class WeekendPolicy {\n\
             static boolean shouldSkip(Calendar calendar) { return calendar.getDay() > 5; }\n\
         }",
    );

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.smell_name == "Incomplete Library Class")
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "WeekendExtensions")
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "WeekendPolicy")
    );
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("Calendar"))
    );
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

    let ordinary_extensions = scan(
        "import vendor.Calendar;\n\
         class CalendarExtensions {\n\
             static boolean isWeekend(Calendar calendar) { return calendar.getDay() > 5; }\n\
             static Calendar nextBusinessDay(Calendar calendar) { return calendar.plusDays(1); }\n\
         }",
    );
    assert!(ordinary_extensions.is_empty());

    let different_helpers = scan(
        "import vendor.Calendar;\n\
         class WeekendExtensions { static boolean isWeekend(Calendar calendar) { return calendar.getDay() > 5; } }\n\
         class NextDayPolicy { static Calendar nextDay(Calendar calendar) { return calendar.plusDays(1); } }",
    );
    assert!(different_helpers.is_empty());
}

#[test]
fn reports_duplicated_rust_extension_implementations_for_an_imported_type() {
    let file = parse(
        "date_helpers.rs",
        "use chrono::DateTime;\n\
         trait WeekdayExt { fn is_weekend(&self) -> bool; }\n\
         impl WeekdayExt for DateTime { fn is_weekend(&self) -> bool { self.weekday().number_from_monday() > 5 } }\n\
         trait WeekendPolicy { fn should_skip(&self) -> bool; }\n\
         impl WeekendPolicy for DateTime { fn should_skip(&self) -> bool { self.weekday().number_from_monday() > 5 } }",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "WeekdayExt")
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "WeekendPolicy")
    );
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("DateTime"))
    );
}

#[test]
fn ignores_rust_local_type_extensions_and_single_method_traits() {
    let local = parse(
        "date_helpers.rs",
        "struct DateTime;\n\
         trait DateTimeExt { fn is_weekend(&self) -> bool; fn next_day(&self) -> Self; }\n\
         impl DateTimeExt for DateTime {\n\
             fn is_weekend(&self) -> bool { self.weekday() }\n\
             fn next_day(&self) -> Self { self.add_days(1) }\n\
         }",
    );
    let single = parse(
        "date_helpers.rs",
        "use chrono::DateTime;\n\
         trait DateTimeExt { fn is_weekend(&self) -> bool; }\n\
         impl DateTimeExt for DateTime {\n\
             fn is_weekend(&self) -> bool { self.weekday() }\n\
         }",
    );

    assert!(
        bluecheese::detect(&[local], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
    assert!(
        bluecheese::detect(&[single], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );

    let different_extensions = parse(
        "date_helpers.rs",
        "use chrono::DateTime;\n\
         trait WeekdayExt { fn is_weekend(&self) -> bool; }\n\
         impl WeekdayExt for DateTime { fn is_weekend(&self) -> bool { self.weekday().number_from_monday() > 5 } }\n\
         trait NextDayExt { fn next_day(&self) -> DateTime; }\n\
         impl NextDayExt for DateTime { fn next_day(&self) -> DateTime { self.add_days(1) } }",
    );
    assert!(
        bluecheese::detect(&[different_extensions], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn reports_duplicated_typescript_runtime_patches_for_an_augmented_type() {
    let file = parse(
        "observable.ts",
        "import { Observable as RxObservable } from 'rxjs';\n\
         declare module 'rxjs' {\n\
             interface Observable<T> {\n\
                 map<U>(fn: (value: T) => U): Observable<U>;\n\
                 filter(predicate: (value: T) => boolean): Observable<T>;\n\
             }\n\
         }\n\
         RxObservable.prototype.map = function(value) { return value.trim().toLowerCase(); };\n\
         RxObservable.prototype.filter = function(value) { return value.trim().toLowerCase(); };",
    );
    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Observable");
    assert!(findings[0].additional_info.contains("rxjs"));
    assert_eq!(findings[0].line_numbers.len(), 2);
}

#[test]
fn ignores_typescript_declarations_without_multiple_runtime_patches() {
    let file = parse(
        "observable.ts",
        "import { Observable } from 'rxjs';\n\
         declare module 'rxjs' {\n\
             interface Observable<T> { map<U>(fn: (value: T) => U): Observable<U>; }\n\
         }\n\
         Observable.prototype.map = function(fn) { return this; };",
    );

    assert!(
        bluecheese::detect(&[file], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );

    let different_patches = parse(
        "observable.ts",
        "import { Observable } from 'rxjs';\n\
         declare module 'rxjs' { interface Observable<T> { map<U>(fn: (value: T) => U): Observable<U>; filter(predicate: (value: T) => boolean): Observable<T>; } }\n\
         Observable.prototype.map = function(value) { return value.trim().toLowerCase(); };\n\
         Observable.prototype.filter = function(value) { return value.length > 0; };",
    );
    assert!(
        bluecheese::detect(&[different_patches], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn reports_duplicated_javascript_prototype_patches_for_an_imported_type() {
    let file = parse(
        "observable.js",
        "import { Observable } from 'rxjs';\n\
         Observable.prototype.map = function(value) { return value.trim().toLowerCase(); };\n\
         Observable.prototype.filter = function(value) { return value.trim().toLowerCase(); };",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Observable");
    assert!(findings[0].additional_info.contains("rxjs"));
}

#[test]
fn ignores_javascript_prototype_patches_with_different_implementations() {
    let file = parse(
        "observable.js",
        "import { Stream } from 'stream-lib';\n\
         Stream.prototype.normalize = function(value) { return value.trim().toLowerCase(); };\n\
         Stream.prototype.isEmpty = function(value) { return value.length === 0; };",
    );

    assert!(
        bluecheese::detect(&[file], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );

    let identity_patches = parse(
        "stream.js",
        "import { Stream } from 'stream-lib';\n\
         Stream.prototype.first = function() { return this; };\n\
         Stream.prototype.second = function() { return this; };",
    );
    assert!(
        bluecheese::detect(&[identity_patches], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}
