

fn parse(input:&str){
    
}
mod part_1{


    fn solve(input:&str)->i32{
        0
    }

    #[cfg(test)]
    mod test{
        use super::solve;
        #[test]
        fn example(){
            let input = super::super::input::example;
            assert_eq!(solve(input),11);
        }
        #[test]
        fn question(){
            dbg!(solve(super::super::input::actual));
        }
    }
}

mod part_2{
    use std::collections::HashMap;



    fn solve(input:&str)->i32{
        
        0

    }

    #[cfg(test)]
    mod test{
        use super::solve;
        #[test]
        fn example(){
            let input = super::super::input::example;
            assert_eq!(solve(input),31);
        }
        #[test]
        fn question(){
            

            dbg!(solve(super::super::input::actual));
        }
    }
}

mod input{
    pub const example:&str = "3";

    pub const actual:&str = "";
}