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
fn reports_repeated_timezone_workarounds_around_imported_library_calls() {
    let findings = scan(
        "import java.util.Calendar;\n\
         import java.util.TimeZone;\n\
         class BookingDisplay {\n\
             long localMillis(Calendar calendar, TimeZone zone) { return calendar.getTimeInMillis() + zone.getOffset(calendar.getTimeInMillis()); }\n\
         }\n\
         class ReminderDisplay {\n\
             long reminderMillis(Calendar calendar, TimeZone zone) { return calendar.getTimeInMillis() + zone.getOffset(calendar.getTimeInMillis()); }\n\
         }",
    );

    assert_eq!(findings.len(), 4);
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "BookingDisplay")
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "ReminderDisplay")
    );
    assert_eq!(
        findings
            .iter()
            .filter(|finding| finding.additional_info.contains("Calendar"))
            .count(),
        2
    );
    assert_eq!(
        findings
            .iter()
            .filter(|finding| finding.additional_info.contains("TimeZone"))
            .count(),
        2
    );
}

#[test]
fn ignores_duplicated_code_that_does_not_call_the_imported_type() {
    let findings = scan(
        "import vendor.Calendar;\n\
         class BookingDisplay {\n\
             String localTime(Calendar calendar, int hour, int offset) { int localHour = (hour + offset + 24) % 24; return Integer.toString(localHour); }\n\
         }\n\
         class ReminderDisplay {\n\
             String label(Calendar calendar, int hour, int offset) { int localHour = (hour + offset + 24) % 24; return Integer.toString(localHour); }\n\
         }",
    );

    assert!(findings.is_empty());
}

#[test]
fn ignores_library_workarounds_centralized_in_a_shared_adapter() {
    let findings = scan(
        "import java.util.Calendar;\n\
         import java.util.TimeZone;\n\
         class CalendarAdapter {\n\
             static long localMillis(Calendar calendar, TimeZone zone) { return calendar.getTimeInMillis() + zone.getOffset(calendar.getTimeInMillis()); }\n\
         }\n\
         class BookingDisplay {\n\
             long localMillis(Calendar calendar, TimeZone zone) { return CalendarAdapter.localMillis(calendar, zone); }\n\
         }\n\
         class ReminderDisplay {\n\
             long reminderMillis(Calendar calendar, TimeZone zone) { return CalendarAdapter.localMillis(calendar, zone); }\n\
         }",
    );

    assert!(findings.is_empty());
}

#[test]
fn reports_repeated_byte_order_correction_around_byte_buffer_calls() {
    let findings = scan(
        "import java.nio.ByteBuffer;\n\
         class HeaderReader {\n\
             short littleEndianWord(ByteBuffer bytes) { int high = bytes.get() & 0xff; int low = bytes.get() & 0xff; return (short) ((low << 8) | high); }\n\
         }\n\
         class PacketReader {\n\
             short packetType(ByteBuffer bytes) { int high = bytes.get() & 0xff; int low = bytes.get() & 0xff; return (short) ((low << 8) | high); }\n\
         }",
    );

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("ByteBuffer"))
    );
}

#[test]
fn reports_repeated_unicode_to_ascii_workaround_around_static_library_calls() {
    let findings = scan(
        "import java.text.Normalizer;\n\
         class SearchIndex {\n\
             String asciiKey(String text) { String decomposed = Normalizer.normalize(text, Normalizer.Form.NFD); return decomposed.replaceAll(\"[^\\\\x00-\\\\x7F]\", \"\"); }\n\
         }\n\
         class UrlBuilder {\n\
             String asciiPath(String text) { String decomposed = Normalizer.normalize(text, Normalizer.Form.NFD); return decomposed.replaceAll(\"[^\\\\x00-\\\\x7F]\", \"\"); }\n\
         }",
    );

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("Normalizer"))
    );
}

#[test]
fn reports_repeated_marshalling_around_jni_native_calls() {
    let findings = scan(
        "import java.nio.charset.StandardCharsets;\n\
         class NativeCodec { static native int decode(byte[] utf8, int length); }\n\
         class ImageReader {\n\
             int read(String text) { byte[] utf8 = text.getBytes(StandardCharsets.UTF_8); return NativeCodec.decode(utf8, utf8.length); }\n\
         }\n\
         class DocumentReader {\n\
             int parse(String text) { byte[] utf8 = text.getBytes(StandardCharsets.UTF_8); return NativeCodec.decode(utf8, utf8.length); }\n\
         }",
    );

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("NativeCodec"))
    );
}

#[test]
fn reports_repeated_numeric_string_conversion_before_library_calls() {
    let findings = scan(
        "import vendor.RangeIndex;\n\
         class UserLookup {\n\
             int find(String id) { return RangeIndex.find(Integer.parseInt(id)); }\n\
         }\n\
         class OrderLookup {\n\
             int find(String id) { return RangeIndex.find(Integer.parseInt(id)); }\n\
         }",
    );

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("RangeIndex"))
    );
}

#[test]
fn reports_repeated_completion_projection_around_an_lrs_api() {
    let findings = scan(
        "import vendor.LrsClient;\n\
         import vendor.Agent;\n\
         import vendor.Activity;\n\
         class CourseDashboard {\n\
             boolean isComplete(LrsClient lrs, Agent user, Activity course) { return lrs.getStatements(user, course).stream().anyMatch(statement -> statement.result().completion()); }\n\
         }\n\
         class LearnerReport {\n\
             boolean completed(LrsClient lrs, Agent user, Activity course) { return lrs.getStatements(user, course).stream().anyMatch(statement -> statement.result().completion()); }\n\
         }",
    );

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("LrsClient"))
    );
}

#[test]
fn reports_equivalent_lrs_completion_pipelines_with_stream_and_loop_syntax() {
    let findings = scan(
        "import vendor.LrsClient;\n\
         import vendor.Agent;\n\
         import vendor.Activity;\n\
         class CourseDashboard {\n\
             boolean isComplete(LrsClient lrs, Agent user, Activity course) { return lrs.getStatements(user, course).stream().anyMatch(statement -> statement.result().completion()); }\n\
         }\n\
         class LearnerReport {\n\
             boolean completed(LrsClient lrs, Agent user, Activity course) { for (Statement statement : lrs.getStatements(user, course)) { if (statement.result().completion()) return true; } return false; }\n\
         }",
    );

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("LrsClient"))
    );
}

#[test]
fn does_not_merge_lrs_completion_and_success_projections() {
    let findings = scan(
        "import vendor.LrsClient;\n\
         import vendor.Agent;\n\
         import vendor.Activity;\n\
         class CourseDashboard {\n\
             boolean isComplete(LrsClient lrs, Agent user, Activity course) { return lrs.getStatements(user, course).stream().anyMatch(statement -> statement.result().completion()); }\n\
         }\n\
         class QuizReport {\n\
             boolean passed(LrsClient lrs, Agent user, Activity course) { for (Statement statement : lrs.getStatements(user, course)) { if (statement.result().success()) return true; } return false; }\n\
         }",
    );

    assert!(findings.is_empty());
}

#[test]
fn does_not_match_completion_from_an_unrelated_collection_to_an_lrs_query() {
    let findings = scan(
        "import vendor.LrsClient;\n\
         import vendor.Agent;\n\
         import vendor.Activity;\n\
         import vendor.Statement;\n\
         import java.util.List;\n\
         class CourseDashboard {\n\
             boolean isComplete(LrsClient lrs, Agent user, Activity course, List<Statement> cached) { lrs.getStatements(user, course); return cached.stream().anyMatch(statement -> statement.result().completion()); }\n\
         }\n\
         class LearnerReport {\n\
             boolean completed(LrsClient lrs, Agent user, Activity course, List<Statement> cached) { lrs.getStatements(user, course); for (Statement statement : cached) { if (statement.result().completion()) return true; } return false; }\n\
         }",
    );

    assert!(findings.is_empty());
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

#[test]
fn reports_equivalent_rust_lrs_completion_pipelines_with_stream_and_loop_syntax() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::LrsClient;\n\
         use vendor::Agent;\n\
         use vendor::Activity;\n\
         struct CourseDashboard;\n\
         impl CourseDashboard {\n\
             fn is_complete(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
         }\n\
         struct LearnerReport;\n\
         impl LearnerReport {\n\
             fn completed(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 for statement in lrs.get_statements(user, course) {\n\
                     if statement.result().completion() { return true; }\n\
                 }\n\
                 false\n\
             }\n\
         }",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "CourseDashboard")
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "LearnerReport")
    );
    assert!(
        findings
            .iter()
            .all(|finding| finding.additional_info.contains("LrsClient"))
    );
}

#[test]
fn does_not_merge_rust_lrs_completion_and_success_projections() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::LrsClient;\n\
         use vendor::Agent;\n\
         use vendor::Activity;\n\
         struct CourseDashboard;\n\
         impl CourseDashboard {\n\
             fn is_complete(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
         }\n\
         struct QuizReport;\n\
         impl QuizReport {\n\
             fn passed(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 for statement in lrs.get_statements(user, course) {\n\
                     if statement.result().success() { return true; }\n\
                 }\n\
                 false\n\
             }\n\
         }",
    );

    assert!(
        bluecheese::detect(&[file], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn does_not_pair_a_rust_lrs_query_with_completion_from_an_unrelated_collection() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::LrsClient;\n\
         use vendor::Agent;\n\
         use vendor::Activity;\n\
         use vendor::Statement;\n\
         struct CourseDashboard;\n\
         impl CourseDashboard {\n\
             fn is_complete(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
         }\n\
         struct LearnerReport;\n\
         impl LearnerReport {\n\
             fn completed(lrs: &LrsClient, user: &Agent, course: &Activity, cached: &[Statement]) -> bool {\n\
                 lrs.get_statements(user, course);\n\
                 for statement in cached {\n\
                     if statement.result().completion() { return true; }\n\
                 }\n\
                 false\n\
             }\n\
         }",
    );

    assert!(
        bluecheese::detect(&[file], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn does_not_use_a_rust_lrs_query_when_completion_reads_a_different_item() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::LrsClient;\n\
         use vendor::Agent;\n\
         use vendor::Activity;\n\
         use vendor::Statement;\n\
         struct Dashboard;\n\
         impl Dashboard {\n\
             fn complete(lrs: &LrsClient, user: &Agent, course: &Activity, cached: &[Statement]) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|_| cached.iter().any(|item| item.result().completion()))\n\
             }\n\
         }\n\
         struct Report;\n\
         impl Report {\n\
             fn completed(lrs: &LrsClient, user: &Agent, course: &Activity, cached: &[Statement]) -> bool {\n\
                 for _item in lrs.get_statements(user, course) {\n\
                     if cached.iter().any(|item| item.result().completion()) { return true; }\n\
                 }\n\
                 false\n\
             }\n\
         }",
    );

    assert!(
        bluecheese::detect(&[file], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn recognizes_rust_lrs_types_from_grouped_imports() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::{LrsClient as Lrs, Agent, Activity};\n\
         struct Dashboard;\n\
         impl Dashboard {\n\
             fn complete(lrs: &Lrs, user: &Agent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
         }\n\
         struct Report;\n\
         impl Report {\n\
             fn completed(lrs: &Lrs, user: &Agent, course: &Activity) -> bool {\n\
                 for statement in lrs.get_statements(user, course) {\n\
                     if statement.result().completion() { return true; }\n\
                 }\n\
                 false\n\
             }\n\
         }",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 2);
}

#[test]
fn does_not_merge_rust_lrs_pipelines_with_different_input_types() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::{LrsClient, Agent, AdminAgent, Activity};\n\
         struct Dashboard;\n\
         impl Dashboard {\n\
             fn complete(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
         }\n\
         struct AdminReport;\n\
         impl AdminReport {\n\
             fn completed(lrs: &LrsClient, user: &AdminAgent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
         }",
    );

    assert!(
        bluecheese::detect(&[file], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn does_not_report_repeated_rust_lrs_pipelines_in_only_one_owner() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::{LrsClient, Agent, Activity};\n\
         struct Dashboard;\n\
         impl Dashboard {\n\
             fn complete(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
             fn is_completed(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                 lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
             }\n\
         }",
    );

    assert!(
        bluecheese::detect(&[file], "incomp-lib-class")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn reports_repeated_rust_lrs_pipelines_in_free_functions() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::{LrsClient, Agent, Activity};\n\
         fn dashboard_complete(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
             lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
         }\n\
         fn report_completed(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
             for statement in lrs.get_statements(user, course) {\n\
                 if statement.result().completion() { return true; }\n\
             }\n\
             false\n\
         }",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name.ends_with("dashboard_complete"))
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name.ends_with("report_completed"))
    );
}

#[test]
fn reports_repeated_rust_lrs_pipelines_inside_a_module() {
    let file = parse(
        "lrs_progress.rs",
        "use vendor::{LrsClient, Agent, Activity};\n\
         mod dashboards {\n\
             struct Dashboard;\n\
             impl Dashboard {\n\
                 fn complete(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                     lrs.get_statements(user, course).iter().any(|statement| statement.result().completion())\n\
                 }\n\
             }\n\
             struct Report;\n\
             impl Report {\n\
                 fn completed(lrs: &LrsClient, user: &Agent, course: &Activity) -> bool {\n\
                     for statement in lrs.get_statements(user, course) {\n\
                         if statement.result().completion() { return true; }\n\
                     }\n\
                     false\n\
                 }\n\
             }\n\
         }",
    );

    let findings = bluecheese::detect(&[file], "incomp-lib-class").expect("run detector");

    assert_eq!(findings.len(), 2);
}
