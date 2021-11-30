
use std::process;
//use colored::*;

pub fn error_bomb( arg : &str )
{
	println!( "{}", "\n!!! ERROR !!!\n" );

	match arg {
		"data_len_not_same" => println!( "The length of all the data must be same." ),
		_                   => (),
	}

	println!( "{}", "\n!!! Program halted !!!\n" );

	process::exit( 1 );
}
