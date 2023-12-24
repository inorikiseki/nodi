## thinking in ui design

i32 f32 bool String

Person
    name: String
    age: i32

Worker
    ..Person
    salary: Salary
    job: Job

Teacher
    ..Worker

Student
    ..Person
    course: Course

Driver
    ..Worker
    car: Car

StudentCanDrive
    ..Student
    ..Driver

Police
    ..Worker
    weapon: Weapon

trait HasWeapon;

#[HasWeapon]
DriverWithWeapon
    ..Driver
    weapon: Weapon

let dww = DriverWithWeapon::new();
dww.shoot();
dww.drive();

Color{r,g,b,a}
Cubic{vertices,transform


Job: String
Salary: i32
Course: String
Car: String
Weapon: String