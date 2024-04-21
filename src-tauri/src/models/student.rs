use std::collections::HashMap;

use super::contracts::Contract;

pub struct Student {
    name: String,
    age: i8,
    class_history: Vec<String>,
    frequency: HashMap<String, bool>,
    payments: HashMap<String, bool>,
    contract: Contract,
}
impl Student {
    pub fn new(student_name: &str, student_age: i8, contract_duration: i8) -> Self {
        Student {
            name: student_name.to_string(),
            age: student_age,
            class_history: Vec::new(),
            frequency: HashMap::new(),
            payments: HashMap::new(),
            contract: Contract::new(contract_duration),
        }
    }

    // Função que recebe o nome da estudante e retorna uma instância de Student
    //    pub fn find_student_by_name(students: &[Student], name: &str) -> Option<Student> {
    //        for student in students {
    //            if student.name == name {
    //                return Some(student.clone()); // Retorna uma cópia do estudante encontrado
    //            }
    //        }
    //        None // Retorna None se nenhum estudante com o nome especificado for encontrado
    //    }
}
