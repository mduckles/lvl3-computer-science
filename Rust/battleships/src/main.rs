use std::io;

enum ShipsTypes{
    Carrier,//5
    Battleship,//4
    Destroyer,//3
    Submarine,//3
    PatrolBoat,//2
}

struct Player{
    board:Vec<Vec<String>>,
    guessingboard:Vec<Vec<String>>,
    name:String,
    num_ships:usize,
    won:bool
}



impl Player{
    fn get_name(&mut self,player:u8){
        println!("player {player} enter your name: ");
        io::stdin().read_line(&mut self.name).expect("failed to read line");
    }
    fn place_ships(&mut self,type_ship:ShipsTypes){
        let mut x = String::new();
        let mut y = String::new();
        let mut right_down= String::new();
        let length = match type_ship{
            ShipsTypes::Carrier => 5,
            ShipsTypes::Battleship => 4,
            ShipsTypes::Destroyer =>3,
            ShipsTypes::Submarine => 3,
            ShipsTypes::PatrolBoat =>2,
        };
        println!("enter the starting x cordinate: ");
        io::stdin().read_line(&mut x).expect("failed to read line");
        let x:usize = match x.trim().parse(){
            Ok(num) => num,
            Err(num) =>0,
        };
        println!("enter the starting y cordinate: ");
        io::stdin().read_line(&mut y).expect("failed to read line");
        let y:usize = match y.trim().parse(){
            Ok(num) => num,
            Err(num) => 0,
        };
        while true{
            println!("enter if you would like the ship to go down or right")
            io::stdin().read_line(&mut right_down).expect("failed to read line");
            if right_down == String::from("down").trim() || right_down == String::from("right").trim(){
                break;
            }
            else{
                println!("you have to enter down or right")
            }
    }
}

fn get_dimensions()->(usize,usize){
    let mut width = String::new();
    let mut height = String::new();
    println!("enter the width of the board: ");
    io::stdin().read_line(&mut width).expect("failed to read line");
    let width:usize = match width.trim().parse(){
        Ok(num) => num,
        Err(num) =>5,
    };
    println!("enter the height of the board: ");
    io::stdin().read_line(&mut height).expect("failed to read line");
    let height:usize = match height.trim().parse(){
        Ok(num) => num,
        Err(num) =>5,
    };
    return (width,height);
}
fn get_num_ships()->usize{
    let mut num_ships=String::new();
    println!("enter how many ships you would like each player to place: ");
    io::stdin().read_line(&mut num_ships).expect("failed to read line");
    let num_ships:usize = match num_ships.trim().parse(){
        Ok(num) => num,
        Err(num) => 3,
    };
    return num_ships;
}

fn gameloop(){
    let dimensions = get_dimensions();
    let num_ships = get_num_ships();
    let mut p1 = Player{
        board:vec![vec![String::from("🌊");dimensions.0];dimensions.1],
        guessingboard:vec![vec![String::from("🌊");dimensions.0];dimensions.1],
        name:String::new(),
        won:false,
        num_ships:num_ships,
    };
    let mut p2 = Player{
        board:vec![vec![String::from("🌊");dimensions.0];dimensions.1],
        guessingboard:vec![vec![String::from("🌊");dimensions.0];dimensions.1],
        name:String::new(),
        won:false,
        num_ships:num_ships,
};
while true{
    
}
}  

fn main() {
    while true{
        gameloop();
    }
}
