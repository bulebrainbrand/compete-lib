#[macro_export]
macro_rules! printVec {
    ($vector:ident) => {
        for ele in $vector {
            println!("{}", ele)
        }
    };
}
#[macro_export]
macro_rules! Yes {
    () => {
        println!("Yes")
    };
}
#[macro_export]
macro_rules! No {
    () => {
        println!("No")
    };
}
#[macro_export]
macro_rules! YesNo {
    ($expr:expr) => {
        println!("{}", if $expr { "Yes" } else { "No" })
    };
}
