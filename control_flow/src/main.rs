fn main() {
    let number:i32 = 6;
    
    if number %2==0{println!("É par");}else{println!("É impar");}

    // a condição deve ser do tipo bool
    let condition = true;
    if  condition{println!("0");}else{println!("1");}
    // obs: ambas condiçãoes devem retornar o mesmo tipo de variável



    // loops, o rust tem 3 tipos de loops, loop, while and for

    let mut counter = 0;

    let _result = loop{counter+=1; if counter==10{break counter*2;}};

    println!("{counter}");

    let a=[10,20,30,40,50];

    let mut index = 0;

    while index <5{
        println!("{}", a[index]);

        index +=1;

    }
    // .rev para da reverse no interável
    for x in (1..4).rev(){
        println!("{x}");
    }
    let a = [5; 10];

    let mut sum = 0;

    for x in a {

        sum += x;

    }

    println!("{sum}");









}

