use core::num;

//Create a simple calculator using enums and pattern matching
/*
enum olusturmak lazım
0 a bölünebilme icin bitane kontrol lazım
4 tane fonksiyon yapmak lazım- acaba fonksiyonlar enumun mu olcak yoksa ayrı fonksiynlar mı
 */
 enum Operation{
    Add,
    Sub,
    Mul,
    Div,
}
fn calculate(number1 : f64, number2 : f64, op: Operation) -> f64{
    match op{
        Operation::Add => {
            return number1 + number2;
        }
        Operation::Sub => {
            return number1 - number2;
        }
        Operation::Mul => {
            return number1*number2;
        }
        Operation::Div => {
            return  number1/number2;
        }
    }
}

fn main() {
    let op = Operation::Div;
    let result = calculate(1.0, 1.0,op);
    println!("result is : {} " , result);
}


