#[macro_export]
macro_rules! dump {
    ($exp: expr) => (
        println!("{} = {:?}", stringify!($exp), $exp);
    )
}


// #[cfg(test)]
// mod tests {

//     #[test]
//     fn it_works() {
//     }
// }
