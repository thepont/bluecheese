//! The twenty-two smells of chapter 3, "Bad Smells in Code".
//!
//! *Refactoring: Improving the Design of Existing Code* (Fowler, 1999) names
//! twenty-two smells. The Java tool this port is checked against exposes
//! nineteen of them:
//!
//! - **Duplicate Code** and **Speculative Generality** have detectors in its
//!   source, but both are commented out of the enumeration its command line
//!   parses, so no label reaches them;
//! - **Incomplete Library Class** has a detector class that is registered
//!   nowhere at all.
//!
//! All twenty-two are listed here regardless. A tool that did not know a smell
//! would answer "unknown" to a name the book uses, and the difference between
//! *not ported yet* and *never heard of it* is the difference between a plan and
//! a bug report.
//!
//! Names follow the reference tool, which is what its reports print and so what
//! a port is held to. That is the book's wording for every smell but one:
//! chapter 3 heads the first "Duplicated Code" and the tool prints "Duplicate
//! Code".

/// One smell of the book.
pub struct Smell {
    /// The label the command line accepts. Where the reference tool has one it
    /// is used unchanged, so that its reports and this tool's can be compared
    /// and so that a port does not have to translate names to be checked. The
    /// three labels it has no label for follow the same style.
    pub label: &'static str,
    /// The name the smell is reported under.
    pub name: &'static str,
    /// Whether the reference tool can be asked for this smell at all, and so
    /// whether its report over a corpus can be parity evidence for a port of it.
    /// False means the port has to be characterised by hand, as the Rust side of
    /// the corpus already has to be.
    pub in_reference_tool: bool,
}

impl std::fmt::Display for Smell {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "'{}' ({})", self.label, self.name)
    }
}

/// The book's list, in the order the chapter takes them.
pub const SMELLS: &[Smell] = &[
    Smell {
        label: "dup-code",
        name: "Duplicate Code",
        in_reference_tool: false,
    },
    Smell {
        label: "long-method",
        name: "Long Method",
        in_reference_tool: true,
    },
    Smell {
        label: "large-class",
        name: "Large Class",
        in_reference_tool: true,
    },
    Smell {
        label: "long-params",
        name: "Long Parameter List",
        in_reference_tool: true,
    },
    Smell {
        label: "divergent-change",
        name: "Divergent Change",
        in_reference_tool: true,
    },
    Smell {
        label: "shotgun",
        name: "Shotgun Surgery",
        in_reference_tool: true,
    },
    Smell {
        label: "feature-envy",
        name: "Feature Envy",
        in_reference_tool: true,
    },
    Smell {
        label: "data-clumps",
        name: "Data Clumps",
        in_reference_tool: true,
    },
    Smell {
        label: "prim-obsession",
        name: "Primitive Obsession",
        in_reference_tool: true,
    },
    Smell {
        label: "switch-stmts",
        name: "Switch Statements",
        in_reference_tool: true,
    },
    Smell {
        label: "parallel-hierarchy",
        name: "Parallel Inheritance Hierarchies",
        in_reference_tool: true,
    },
    Smell {
        label: "lazy-class",
        name: "Lazy Class",
        in_reference_tool: true,
    },
    Smell {
        label: "spec-gen",
        name: "Speculative Generality",
        in_reference_tool: false,
    },
    Smell {
        label: "temp-field",
        name: "Temporary Field",
        in_reference_tool: true,
    },
    Smell {
        label: "message-chains",
        name: "Message Chains",
        in_reference_tool: true,
    },
    Smell {
        label: "middle-man",
        name: "Middle Man",
        in_reference_tool: true,
    },
    Smell {
        label: "intimacy",
        name: "Inappropriate Intimacy",
        in_reference_tool: true,
    },
    Smell {
        label: "alt-classes",
        name: "Alternative Classes with Different Interfaces",
        in_reference_tool: true,
    },
    Smell {
        label: "incomp-lib-class",
        name: "Incomplete Library Class",
        in_reference_tool: false,
    },
    Smell {
        label: "data-class",
        name: "Data Class",
        in_reference_tool: true,
    },
    Smell {
        label: "refused-bequest",
        name: "Refused Bequest",
        in_reference_tool: true,
    },
    Smell {
        label: "comments",
        name: "Comments",
        in_reference_tool: true,
    },
];

/// The smell with this command-line label, if the book names one.
pub fn by_label(label: &str) -> Option<&'static Smell> {
    SMELLS.iter().find(|smell| smell.label == label)
}

/// The smell reported under this name, if the book names one. Reports are what
/// this is for: a golden file names smells rather than labelling them.
pub fn by_name(name: &str) -> Option<&'static Smell> {
    SMELLS.iter().find(|smell| smell.name == name)
}

/// Every label, for a message that has to list what the tool knows.
pub fn labels() -> Vec<&'static str> {
    SMELLS.iter().map(|smell| smell.label).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    /// Pin count and endpoints without duplicating the entire transcription.
    #[test]
    fn the_list_is_the_books_twenty_two() {
        assert_eq!(SMELLS.len(), 22);
        assert_eq!(SMELLS.first().map(|s| s.name), Some("Duplicate Code"));
        assert_eq!(SMELLS.last().map(|s| s.name), Some("Comments"));
    }

    /// Two detectors are commented out of the reference CLI and one is never
    /// registered. If a later reference version exposes them, update this audit
    /// and recapture goldens to establish parity evidence for their ports.
    #[test]
    fn the_three_the_reference_tool_cannot_be_asked_for_are_the_ones_we_audited() {
        let unsupported: Vec<&str> = SMELLS
            .iter()
            .filter(|smell| !smell.in_reference_tool)
            .map(|smell| smell.label)
            .collect();
        assert_eq!(
            unsupported,
            ["dup-code", "spec-gen", "incomp-lib-class"],
            "in the book's order"
        );
    }

    /// Duplicate labels make detectors unreachable; duplicate names make
    /// serialized reports ambiguous.
    #[test]
    fn labels_and_names_are_each_unique() {
        for field in ["label", "name"] {
            let seen: BTreeSet<&str> = SMELLS
                .iter()
                .map(|smell| {
                    if field == "label" {
                        smell.label
                    } else {
                        smell.name
                    }
                })
                .collect();
            assert_eq!(seen.len(), SMELLS.len(), "duplicate {field}");
        }
    }

    #[test]
    fn a_label_or_name_the_book_does_not_use_finds_nothing() {
        assert!(
            by_label("dead-code").is_none(),
            "not a smell of the chapter"
        );
        assert!(by_name("Banana").is_none());
        assert_eq!(
            by_label("switch-stmts").map(|s| s.name),
            Some("Switch Statements")
        );
    }
}
