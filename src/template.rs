mod input{
    pub const example:&str = "3";

    pub const actual:&str = "";
}

fn parse(input:&str){
    
}
mod part_1{


    fn solve()->i32{
        0
    }

    #[cfg(test)]
    mod test{
        use super::super::parse;
        use super::solve;
        #[test]
        fn example(){
            let input = super::super::input::example;
            parse(input);
            assert_eq!(11,11);
        }
        #[test]
        fn question(){
            
            parse(super::super::input::actual);

            dbg!();
        }
    }
}

mod part_2{
    use std::collections::HashMap;



    fn solve()->i32{
        
        0

    }

    #[cfg(test)]
    mod test{
        use super::super::parse;
        use super::solve;
        #[test]
        fn example(){
            let input = super::super::input::example;
            parse(input);
            assert_eq!(31,31);
        }
        #[test]
        fn question(){
            
            parse(super::super::input::actual);

            dbg!();
        }
    }
}