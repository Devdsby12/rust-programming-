// chearing check code  samulation
fn main(){
    let average_call_duration :i8 = 4 ;
    let weekly_calls :i8 = 3 ;
    let gender_of_caller : &str = "MALE" ;
    let gender_of_your_partner : &str = "FEMALE" ;

    let result : bool = weekly_calls > 3
         && average_call_duration > 3
        && ( (gender_of_caller == "MALE" && gender_of_your_partner =="FEMALE") 
        || (gender_of_caller == "FEMALE" && gender_of_your_partner =="MALE") ); 
    if result {
    println!("your partner cheating on you") ;
    }
    if !result {
    println!("Not cheating") ;
    
    }
}

fn main() {
    let average_call_duration: i8 = 4;
    let weekly_calls: i8 = 3;
    let gender_of_caller: &str = "MALE";
    let gender_of_your_partner: &str = "FEMALE";

    let result: bool =
        weekly_calls > 3
        && average_call_duration > 3
        && (
            (gender_of_caller == "MALE" && gender_of_your_partner == "FEMALE")
            || (gender_of_caller == "FEMALE" && gender_of_your_partner == "MALE")
        );

    if result {
        println!("your partner cheating on you");
    } else {
        println!("Not cheating");
    }
}
