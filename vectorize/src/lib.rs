extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn vectorize(input: TokenStream) -> TokenStream {
    let mut iterator = input.into_iter();
    let name = iterator.next().unwrap().to_string();
    iterator.next();
    let arity: i32 = iterator.next().unwrap().to_string().parse().unwrap();

    let arguments_string = (0..arity)
        .map(|i| format!("params[{}]", i))
        .collect::<Vec<String>>()
        .join(", ");

    return format!("|params: [f64; {}]| {}({})", arity, name, arguments_string).parse().unwrap()
}

//pub fn add(left: usize, right: usize) -> usize {
//    left + right
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    //#[test]
//    //fn it_works() {
//    //    vectorize!(date)
//    //}
//}
