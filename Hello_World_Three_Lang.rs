fn greet_world() {
    println!("Hello world!");     

    let south_korea = "안녕!";         
    let spain = "Hola Mundo";                

    let regions = [south_korea, spain];     

    for region in regions.iter() {               
            println!("{}", &region);             
    }
}

fn main() {
    greet_world();                               
}
