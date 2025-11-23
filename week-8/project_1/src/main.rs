//Validating Staff Level
use std::io;

fn main() {
    // Vectors for staff categories
    let office = vec!["intern", "administrator", "senior administrator", "office manager", "director", "ceo"];
    let academic = vec!["-", "research assistant", "phd candidate", "post-doc researcher", "senior lecturer", "dean"];
    let lawyer = vec!["paralegal", "junior associate", "associate", "senior associate 1-2", "senior associate 3-4", "partner"];
    let teacher = vec!["placement", "classroom teacher", "snr teacher", "leading teacher", "deputy principal", "principal"];

    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    println!("\nSTAFF CATEGORY OPTIONS:");
    println!("office, academic, lawyer, teacher\n");

    //Inputs & Validation
    println!("Enter your designated staff category: ");
    let mut category = String::new();
    io::stdin().read_line(&mut category).expect("Error");
    let category = category.trim().to_lowercase();

    println!("\nJob Title:\n
        Office Administrator  => Intern | Administrator | Senior Administrator | Office Manager | Director | CEO
        Academic              => - | Research Assistant | PhD Candidate | Post-Doc Researcher | Senior Lecturer | Dean
        Lawyer                => Paralegal | Junior Associate | Associate | Senior Associate 1-2 | Senior Associate 3-4 | Partner
        Teacher               => Placement |  Classroom Teacher |  Snr Teacher | Leading Teacher |  Deputy Principal | Principal\n");

    println!("\nEnter your job title (as on the table):");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Error");
    let title = title.trim().to_lowercase();

    let mut index = 100; // placeholder for not found

    // Office Admin
    if category == "office" {
        for i in 0..office.len() {
            if office[i] == title {
                index = i;
            }
        }
    }
    // Academic
    else if category == "academic" {
        for i in 0..academic.len() {
            if academic[i] == title {
                index = i;
            }
        }
    }
    // Lawyer
    else if category == "lawyer" {
        for i in 0..lawyer.len() {
            if lawyer[i] == title {
                index = i;
            }
        }
    }
    // Teacher
    else if category == "teacher" {
        for i in 0..teacher.len() {
            if teacher[i] == title {
                index = i;
            }
        }
    }
    // Invalid category
    else {
        println!("Invalid staff category!");
        return;
    }

    //Report
    if index < levels.len() {
        println!("\nPosition Level: {}", levels[index]);
    } else {
        println!("Job title not found in this category!");
    }
}   