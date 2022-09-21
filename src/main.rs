use std::collections::HashMap;
use enigo::*;
use std::{thread, time};
fn main() {


    thread::sleep(time::Duration::from_secs(2));
    
    // struct for setting the coordinates
    #[derive(Debug, Clone, Copy)]
    struct Coords { 
    x: i32,//within 1-10
    y: i32, //within 1-3
    page: i32, //within 1-2
    letter: char //what letter or number is it 
    }
    //initializing some variables
    let mut enigo = Enigo::new();

    let mut alphabet: HashMap<char, Coords> = HashMap::new();

    let mut pos = Coords{x:1,y:4,page:1,letter:'A'};
///////////////////////////////////////////////////////////////////////////////
    //bottom row of letters
    let t = Coords{x:1,y:1,page:1,letter:'T'};
    let u = Coords{x:2,y:1,page:1,letter:'U'};
    let v = Coords{x:3,y:1,page:1,letter:'V'};
    let w = Coords{x:4,y:1,page:1,letter:'W'};
    let x = Coords{x:5,y:1,page:1,letter:'X'};
    let y = Coords{x:6,y:1,page:1,letter:'Y'};
    let z = Coords{x:7,y:1,page:1,letter:'Z'};
    //middle bottom row of letters
    let m = Coords{x:1,y:2,page:1,letter:'M'};
    let n = Coords{x:2,y:2,page:1,letter:'N'};
    let o = Coords{x:3,y:2,page:1,letter:'O'};
    let p = Coords{x:4,y:2,page:1,letter:'P'};
    let q = Coords{x:5,y:2,page:1,letter:'Q'};
    let r = Coords{x:6,y:2,page:1,letter:'R'};
    let s = Coords{x:7,y:2,page:1,letter:'S'};
    //middle top row of letters
    let g = Coords{x:1,y:3,page:1,letter:'G'};
    let h = Coords{x:2,y:3,page:1,letter:'H'};
    let i = Coords{x:3,y:3,page:1,letter:'I'};
    let j = Coords{x:4,y:3,page:1,letter:'J'};
    let k = Coords{x:5,y:3,page:1,letter:'K'};
    let l = Coords{x:6,y:3,page:1,letter:'L'};
    //top row of letters
    let a = Coords{x:1,y:4,page:1,letter:'A'};
    let b = Coords{x:2,y:4,page:1,letter:'B'};
    let c = Coords{x:3,y:4,page:1,letter:'C'};
    let d = Coords{x:4,y:4,page:1,letter:'D'};
    let e = Coords{x:5,y:4,page:1,letter:'E'};
    let f = Coords{x:6,y:4,page:1,letter:'F'};
    //top row of numbers
    let _0 = Coords{x:1,y:4,page:2,letter:'0'};
    let _1 = Coords{x:2,y:4,page:2,letter:'1'};
    let _2 = Coords{x:3,y:4,page:2,letter:'2'};
    let _3 = Coords{x:4,y:4,page:2,letter:'3'};
    let _4 = Coords{x:5,y:4,page:2,letter:'4'};
    //bottom row of numbers
    let _5 = Coords{x:1,y:3,page:2,letter:'5'};
    let _6 = Coords{x:2,y:3,page:2,letter:'6'};
    let _7 = Coords{x:3,y:3,page:2,letter:'7'};
    let _8 = Coords{x:4,y:3,page:2,letter:'8'};
    let _9 = Coords{x:5,y:3,page:2,letter:'9'};
    //UPPER and lower buttons
    let upper = Coords{x:7,y:4,page:2,letter:'_'}; //underscore UPPER
    let lower = Coords{x:9,y:4,page:1,letter:'-'}; //dash lower
///////////////////////////////////////////////////////////////////////////////
    //inserting every letters struct into the hashmap
    alphabet.insert('A', a);
    alphabet.insert('B', b);
    alphabet.insert('C', c);
    alphabet.insert('D', d);
    alphabet.insert('E', e);
    alphabet.insert('F', f);
    alphabet.insert('G', g);
    alphabet.insert('H', h);
    alphabet.insert('I', i);
    alphabet.insert('J', j);
    alphabet.insert('K', k);
    alphabet.insert('L', l);
    alphabet.insert('M', m);
    alphabet.insert('N', n);
    alphabet.insert('O', o);
    alphabet.insert('P', p);
    alphabet.insert('Q', q);
    alphabet.insert('R', r);
    alphabet.insert('S', s);
    alphabet.insert('T', t);
    alphabet.insert('U', u);
    alphabet.insert('V', v);
    alphabet.insert('W', w);
    alphabet.insert('X', x);
    alphabet.insert('Y', y);
    alphabet.insert('Z', z);
    //inserting every number struct into the hashmap
    alphabet.insert('0', _0);
    alphabet.insert('1', _1);
    alphabet.insert('2', _2);
    alphabet.insert('3', _3);
    alphabet.insert('4', _4);
    alphabet.insert('5', _5);
    alphabet.insert('6', _6);
    alphabet.insert('7', _7);
    alphabet.insert('8', _8);
    alphabet.insert('9', _9);
    //inserting the UPPER and lower buttons
    alphabet.insert('_', upper); // underscore UPPER 
    alphabet.insert('-', lower); // dash lower
///////////////////////////////////////////////////////////////////////////////
    
    //setting the array of characters from the mystery gift code
    let gift_code = "1CG6MD2DF2ZH"; //mystery gift code
    let code_letters: Vec<char> = gift_code.chars().collect();
    let mut on_right_page = true;
    let mut switched = false;
    //start at A (1,4) page 1
    //move to character
    //if character is on another page, switch page
    //if page switches go back to (1,4) on either page
    //move to next character
    for character in code_letters {
        let char_struct = alphabet[&character];
        //check if number, then check page we're on
        if character.is_numeric() == true { //is character a number
            println!("{} is a number", character);
            //does page = page 2
            if pos.page == char_struct.page { // are we on the right page
                on_right_page = true; //yes
            }else {
                on_right_page = false; //no
            }
        }else if character.is_numeric() == false { //is character a letter
            println!("{} is not a number", character);
            //does page = page 1
            if pos.page == char_struct.page { // are we on the right page 
                on_right_page = true; //yes
            }else { 
                on_right_page = false; //no
            }
        }
        println!("are we on the right page: {}", on_right_page);
        //if we're not on the same page, go to it
        if on_right_page == false {
            if pos.page == 1 { //are we on page 1? if so, go to page 2
                //move cursor to the lower button and click it twice
                //go to lower button
                for i in 1..=(alphabet[&'-'].y - pos.y) { //how far down are we from the button
                    enigo.key_click(Key::Layout('w'));
                    thread::sleep(time::Duration::from_secs(1/8));
                    println!("moved up");
                }
                for i in 1..=(alphabet[&'-'].x - pos.x) { //how far left are we from the button
                    enigo.key_click(Key::Layout('d'));
                    thread::sleep(time::Duration::from_secs(1/8));
                    println!("moved right");
                }
                for i in 1..=2 {
                    enigo.key_click(Key::Return);
                    thread::sleep(time::Duration::from_secs(1));
                }
                //when we are on page 2 set pos to page 2
                pos.page = 2;
            }else { //else we are on page 2, and need to go to page 1
                //move cursor to the UPPER button and click it once
                //go to UPPER button
                for i in 1..=(alphabet[&'_'].y - pos.y) { //how far up are we from the button
                    enigo.key_click(Key::Layout('w'));
                    thread::sleep(time::Duration::from_secs(1/8));
                    println!("moved up");
                }
                for i in 1..=(alphabet[&'_'].x - pos.x) { //how far right are we from the button
                    enigo.key_click(Key::Layout('d'));
                    thread::sleep(time::Duration::from_secs(1/8));
                    println!("moved right");
                }
                for i in 1..=1 {
                    enigo.key_click(Key::Return);
                    thread::sleep(time::Duration::from_secs(1));
                }
                //when we are on page 1 set pos to page 1
                pos.page = 1;
            }
            switched = true; //we switched pages
        }
        if on_right_page == true {
            switched = false;
        }
        println!("{}", switched);
        //if you switched pages go to (1,4)
        if switched == true {
            //move cursor to (1,4)

            if pos.page == 2 { //are we on page 2
                //set pos to the UPPER button 
                pos = alphabet[&'_']; //underscore is UPPER button
                //we need to move to '0'
                for i in 1..=6 { //move cursor to the left 6 times
                    enigo.key_click(Key::Layout('a'));
                    thread::sleep(time::Duration::from_secs(1/8));
                };
            }else { //else we are on page 1
                //set pos to the lower button
                pos = alphabet[&'-']; //dash is lower button
                for i in 1..=8 { //move cursor to the left 8 times
                    enigo.key_click(Key::Layout('a'));
                    thread::sleep(time::Duration::from_secs(1/8));
                };
            }
            //set pos to A if letter and 0 if number 
            if character.is_numeric() { //is character a number
                pos = alphabet[&'0'];
            }else { //else character is a letter
                pos = alphabet[&'A'];
            }
        }
        
        //find character
        //compare y coordinates
        if char_struct.y < pos.y { //is the character below our posistion
            for i in 1..=(pos.y - char_struct.y) { 
                enigo.key_click(Key::Layout('s'));
                thread::sleep(time::Duration::from_secs(1/8));
            }
        }else if pos.y < char_struct.y { //is our posistion below the character
            for i in 1..=(char_struct.y - pos.y) { 
                enigo.key_click(Key::Layout('w'));
                thread::sleep(time::Duration::from_secs(1/8));
            }
        }
        //compare x coordinates
        if char_struct.x < pos.x { //is the character to the left of our posistion
            //move left until we have the same x coordinate
            println!("pos x is farther right");
            for i in 1..=(pos.x - char_struct.x) { 
                enigo.key_click(Key::Layout('a'));
                thread::sleep(time::Duration::from_secs(1/8));
            }
        }else if pos.x < char_struct.x { //is our posistion to the left of the character
            //move right until we have the same x coordinate
            println!("character x is farther right");
            for i in 1..=(char_struct.x - pos.x) { 
                enigo.key_click(Key::Layout('d'));
                thread::sleep(time::Duration::from_secs(1/8));
            }
        }
        enigo.key_click(Key::Return);
        thread::sleep(time::Duration::from_secs(1/2));
        pos = char_struct;
        //rinse and repeat 
    }
}
