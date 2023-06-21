//Create a simple calculator using enums and pattern matching
/*
enum olusturmak lazım
0 a bölünebilme icin bitane kontrol lazım
4 tane fonksiyon yapmak lazım- acaba fonksiyonlar enumun mu olcak yoksa ayrı fonksiynlar mı
 */
enum Operation{
    _Add,
    _Sub,
    _Mul,
    _Div,
}

fn calculate(number1 : f64, number2 : f64, op: Operation) -> Result<f64, &'static str>{
    match op{
        Operation::_Add => Ok(number1 + number2),
        Operation::_Sub => Ok(number1 - number2),
        Operation::_Mul => Ok(number1 * number2),
        Operation::_Div => {
            if number2 == 0.0 {
                Err("Cannot divide by zero")
            } else {
                Ok(number1 / number2)
            }
        }
    }
}

fn main() {
    let op = Operation::_Div;
    match calculate(1.0, 6.0, op) {
        Ok(result) => println!("Result is : {} ", result),
        Err(err) => println!("Error: {}", err),
    }
}