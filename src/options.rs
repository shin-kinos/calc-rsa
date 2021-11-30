
use std::env;
use std::process;

pub struct Options {
	pub input   : String,
	pub output  : String,
	//pub chainid : String,
	pub format  : String,
}

impl Options {
	pub fn new() -> Options
	{
		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_o : &String = &String::new();
		//let mut arg_c : &String = &String::from( "all-chains" );
		let mut arg_f : &String = &String::from( "mmcif"      );

		if argc < 5 { show_usage( &argv[ 0 ] ) };

		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"-i" => { i += 1; arg_i = &argv[ i ]; }
				"-o" => { i += 1; arg_o = &argv[ i ]; }
				//"-c" => { i += 1; arg_c = &argv[ i ]; }
				"-f" => { i += 1; arg_f = &argv[ i ]; }
				"-h" => { show_usage( &argv[ 0 ] );   }
				_    => { show_usage( &argv[ 0 ] );   }
			}
			i += 1;
		}

		match ( *arg_f ).as_str() {
			"mmcif" | "pdb" => (),
			_               => show_usage( &argv[ 0 ] ),
		}

		Options {
			input   : arg_i.to_string(),
			output  : arg_o.to_string(),
			//chainid : arg_c.to_string(),
			format  : arg_f.to_string(),
		}
	}

	pub fn show_parameter( &self )
	{
		println!( "\nParameter set :"                           );
		println!( "===========================================" );
		println!( "Input filename    : {}", self.input          );
		println!( "Onput filename    : {}", self.output         );
		println!( "Input file format : {}", self.format         );
		//println!( "Chain ID          : {}", self.chainid        );
		println!( "===========================================" );
	}
}

fn show_usage( arg : &String )
{
	println!( "Usage: {} [Options] \n\nOptions :\n\n", *arg );
	println!( "    -i    Input filename, REQUIRED." );
	println!( "    -o    Output filename, REQUIRED." );
	println!( "    -f    Input file format ('mmcif' or 'pdb', default 'mmcif')." );
	//println!( "    -c    Chain ID (default 'all-chains'), if 'all-chains', it calculates RSA to all the chains." );
	println!( "    -h    Print this help, ignore all other arguments." );
	println!( "\n" );

	process::exit( 1 );
}
