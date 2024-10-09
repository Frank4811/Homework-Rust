#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name,
            grade,
            major,
        }
    }

    fn introduce_yourself(&self) {
        //TODO! (implement printing info about Student
        // use match statement)
        let _grade_str  = match self.grade{
            GradeLevel::Bachelor => "Bachelor's",
            GradeLevel::Master => "Master's",
            GradeLevel::PhD => "PhD's",
        };
        let _major_str = match self.major{
            Major::ComputerScience => "Computer Science",
            Major:: ElectricalEngineering => "Electrical Engineering",
        };
        println!("Hi, I'm {}. I'm pursuing a {} degree in {}.",self.name, _grade_str, _major_str);
        }
    }

fn main() {
    let s1 = Student::new("John".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
    s1.introduce_yourself();
}