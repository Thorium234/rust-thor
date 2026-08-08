fn main() {
 let weight:f64=90.7;
 let height:f64=1.45;
 let bmi:f64= calculate_bmi(weight,height);

 println!("your bmi is: {:.2}",bmi);
}

//BMI= height(kg)/height(m)^2
fn calculate_bmi(weight_kg:f64,height_m:f64)->f64{
    weight_kg/height_m*height_m
}

