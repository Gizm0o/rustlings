// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


pub struct ReportCard {
    pub grade: Grade, // Change the type to Grade enum
    pub student_name: String,
    pub student_age: u8,
}

// Define a new enum Grade to represent both numerical and alphabetical grades
#[derive(Debug, PartialEq)]
pub enum Grade {
    Numeric(f32),
    Alphabetic(String),
}

impl ReportCard {
    pub fn print(&self) -> String {
        match &self.grade {
            Grade::Numeric(n) => format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, n),
            Grade::Alphabetic(g) => format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, g),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
        grade: Grade::Alphabetic("A+".to_string()), // Change the grade to Alphabetic("A+")
        student_name: "Gary Plotter".to_string(),
        student_age: 11,
    };
    assert_eq!(
        report_card.print(),
        "Gary Plotter (11) - achieved a grade of A+"
    );
}
}

// Writeup
// The first thing we need to do is to define a new enum Grade that can represent both numerical and alphabetical grades. 
// We can do this by defining a new enum Grade with two variants: Numeric(f32) and Alphabetic(String). 
// We then need to change the type of the grade field in the ReportCard struct to be of type Grade enum.
//
// In the print method, we need to pattern match on the grade field to determine whether it is a Numeric or Alphabetic grade.
// We then use the format! macro to create a string that includes the student's name, age, and grade.
//
// In the second test, we change the grade to Grade::Alphabetic("A+") to show that our changes allow alphabetical grades.
// We then assert that the output of the print method is as expected.