/**
 *  Rust code - grammar.rs
 * 
 *  Basic grammars
 */

/*
 * data type - scalar

* integer

Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
arch	isize	usize

* integer literals

Number literals	Example
Decimal	        98_222
Hex	            0xff
Octal	          0o77
Binary	        0b1111_0000
Byte (u8 only)	b'A'

* floating point

f32
f64             default

* operators

+, -, *, /, %

* boolean

bool            true or false

* char

unicode scalar
U+0000 ~ U+D7FF, U+E000 ~ U+10FFFF
*/

// * tuple

pub fn tuple_test() {
    // create tuple(packing)
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // unpacking(destructuring)
    let (x, y, z) = tup;

    println!("The values of x, y, z are: {}, {}, {}", x, y, z);
}

// * array

// Rust에서는 배열은 고정된 길이를 갖는다는 점입니다: 
// 한번 선언되면, 이들은 크기는 커지거나 작아지지 않습니다.

pub fn array_test() {
    let arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("arr[3] = {}", arr[3]);
    println!("months[11] = {}", months[11]);
}

pub fn for_test() {
    for number in (1..10).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}