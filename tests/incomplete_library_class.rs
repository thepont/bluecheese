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

#[test]
fn reports_rust_extension_traits_for_an_imported_type() {
    let file = parse(
        "date_helpers.rs",
        "use chrono::DateTime;\n\
         trait DateTimeExt {\n\
             fn is_weekend(&self) -> bool;\n\
             fn next_business_day(&self) -> DateTime;\n\
         }\n\
         impl DateTimeExt for DateTime {\n\
             fn is_weekend(&self) -> bool { self.weekday().number_from_monday() > 5 }\n\
             fn next_business_day(&self) -> DateTime { self.add_days(1) }\n\
         }",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "DateTimeExt");
    assert!(findings[0].additional_info.contains("DateTime"));
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
}

#[test]
fn reports_typescript_module_augmentations_with_runtime_patches() {
    let file = parse(
        "observable.ts",
        "import { Observable as RxObservable } from 'rxjs';\n\
         declare module 'rxjs' {\n\
             interface Observable<T> {\n\
                 map<U>(fn: (value: T) => U): Observable<U>;\n\
                 filter(predicate: (value: T) => boolean): Observable<T>;\n\
             }\n\
         }\n\
         RxObservable.prototype.map = function(fn) { return this; };\n\
         RxObservable.prototype.filter = function(predicate) { return this; };",
    );
    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Observable");
    assert!(findings[0].additional_info.contains("rxjs"));
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
}

#[test]
fn reports_javascript_imports_with_repeated_prototype_patches() {
    let file = parse(
        "observable.js",
        "import { Observable } from 'rxjs';\n\
         Observable.prototype.map = function(fn) { return this; };\n\
         Observable.prototype.filter = function(predicate) { return this; };",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Observable");
    assert!(findings[0].additional_info.contains("rxjs"));
}
