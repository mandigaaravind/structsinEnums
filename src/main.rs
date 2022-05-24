pub enum Arrow{
    Stable{
        e:u8,
        f:u8,    
    },
    Unstable{
        a:u8,
        b:u8
    },
    Red,
}

fn main(){
     let c:Arrow=Arrow::Stable{e:5,f:6};
     match c{
         Arrow::Stable{e,f}=>println!("{}",e),
         Arrow::Unstable{a,b}=>println!("{}","hello"),
         Arrow::Red=>println!("{}","to be done"),
     };

}