// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute 'rustlings hint generics3' for hints!
use std::fmt;

#[derive(Debug,PartialEq)]
pub struct NumericGrade {
    grade: f32,
}

impl NumericGrade {
    fn new(grade: f32) -> Result<NumericGrade, GradeError> {
        if grade < 1.0 || grade > 5.5 {
            Err(GradeError::OutOfRange(format!("grade {} was out of range!", grade)))
        } else {
            Ok(NumericGrade{grade})
        }
    }
}

#[derive(Debug,PartialEq)]
enum AlphaGradeModifier {
    Plus,
    No,
    Minus,
}
impl fmt::Display for AlphaGradeModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlphaGradeModifier::Plus => write!(f, "+"),
            AlphaGradeModifier::No => write!(f, ""),
            AlphaGradeModifier::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct AlphaGrade {
    grade: char,
    modifier: AlphaGradeModifier,
}
impl AlphaGrade {
    fn new(grade: char, modifier: AlphaGradeModifier) -> Result<AlphaGrade, GradeError> {
        let grade_upper = grade.to_ascii_uppercase();
        if grade_upper < 'A' || grade_upper > 'F' {
            Err(GradeError::OutOfRange(format!("grade {} was out of range!", grade_upper)))
        } else {
            Ok(AlphaGrade{ grade: grade_upper, modifier: modifier })
        }
    }
}
impl fmt::Display for AlphaGrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.grade, self.modifier)
    }
}

#[derive(Debug)]
pub enum Grade {
    Alpha(AlphaGrade),
    Numeric(NumericGrade),
}
impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grade::Alpha(a) => {
                write!(f, "{}", a)
            },
            Grade::Numeric(n) => {
                write!(f, "{}", n.grade)
            },
        }
    }
}

#[derive(Debug,PartialEq)]
enum GradeError {
    OutOfRange(String),
}

pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(NumericGrade::new(2.1).unwrap()),
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
            grade: Grade::Alpha(AlphaGrade::new('a', AlphaGradeModifier::Plus).unwrap()),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }

    #[test]
    fn numeric_too_high() {
        assert_eq!(NumericGrade::new(5.6), Err(GradeError::OutOfRange(format!("grade 5.6 was out of range!"))))
    }
    #[test]
    fn numeric_upper_bound() {
        assert_eq!(NumericGrade::new(5.5), Ok(NumericGrade{grade: 5.5}))
    }
    #[test]
    fn numeric_lower_bound() {
        assert_eq!(NumericGrade::new(1.0), Ok(NumericGrade{grade: 1.0}))
    }
    #[test]
    fn numeric_too_low() {
        assert_eq!(NumericGrade::new(0.99), Err(GradeError::OutOfRange(format!("grade 0.99 was out of range!"))))   
    }
    #[test]
    fn alpha_too_high() {
        assert_eq!(AlphaGrade::new('@', AlphaGradeModifier::No), Err(GradeError::OutOfRange(format!("grade @ was out of range!"))))   
    }
    #[test]
    fn alpha_too_low() {
        assert_eq!(AlphaGrade::new('G', AlphaGradeModifier::No), Err(GradeError::OutOfRange(format!("grade G was out of range!"))))   
    }
    #[test]
    fn alpha_upper_bound() {
        assert_eq!(AlphaGrade::new('a', AlphaGradeModifier::Plus), Ok(AlphaGrade{grade: 'A', modifier: AlphaGradeModifier::Plus}))
    }
    #[test]
    fn alpha_lower_bound() {
        assert_eq!(AlphaGrade::new('f', AlphaGradeModifier::Minus), Ok(AlphaGrade{grade: 'F', modifier: AlphaGradeModifier::Minus}))
    }
    #[test]
    fn formats_test() {
        assert_eq!(
            format!("{}", Grade::Alpha(AlphaGrade::new('a', AlphaGradeModifier::Plus).unwrap())),
            "A+");
        assert_eq!(
            format!("{}", Grade::Alpha(AlphaGrade::new('b', AlphaGradeModifier::Minus).unwrap())),
            "B-");
        assert_eq!(
            format!("{}", Grade::Alpha(AlphaGrade::new('C', AlphaGradeModifier::No).unwrap())),
            "C");
    }
}
