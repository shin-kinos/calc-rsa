
use std::fs::File;
use std::io::Write;

pub fn show_result
(
	chainid_list : &Vec<&str> ,
	resseq_list  : &Vec<usize>,
	resname_list : &Vec<&str> ,
	asa_list     : &Vec<f64>  ,
	rsa_list     : &Vec<f64>  ,
	state_list   : &Vec<&str>
)
{
	println!( "\nResult : \n" );

	print!( "Number\t"  );
	print!( "ChainID\t" );
	print!( "ResSeq\t"  );
	print!( "ResName\t" );
	print!( "ResASA\t"  );
	print!( "ResRSA\t"  );
	print!( "State"     );
	print!( "\n"        );

	for i in 0 .. ( *rsa_list ).len() {
		print!( "{}\t"   , i + 1                          );
		print!( "{}\t"   , ( *chainid_list )[ i ]         );
		print!( "{}\t"   , ( *resseq_list  )[ i ]         );
		print!( "{}\t"   , ( *resname_list )[ i ]         );
		print!( "{:.3}\t", ( *asa_list     )[ i ]         );
		print!( "{:.3}\t", ( *rsa_list     )[ i ] * 100.0 );
		print!( "{}"     , ( *state_list   )[ i ]         );
		print!( "\n"                                      );
	}

}

pub fn save_result(
	chainid_list : &Vec<&str> ,
	resseq_list  : &Vec<usize>,
	resname_list : &Vec<&str> ,
	asa_list     : &Vec<f64>  ,
	rsa_list     : &Vec<f64>  ,
	state_list   : &Vec<&str> ,
	arg_o        : &String
)
{
	let fout_name : String = arg_o.to_owned() + &".rsa".to_string();
	let mut fout = File::create( fout_name.as_str() ).expect( "FAILED to open output file" );

	//writeln!( fout, "{}", "num\tcons\tsite" ).expect( "FAILED to write" );
	write!( fout, "{}", "Number\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "ChainID\t" ).expect( "FAILED to write" );
	write!( fout, "{}", "ResSeq\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "ResName\t" ).expect( "FAILED to write" );
	write!( fout, "{}", "ResASA\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "ResRSA\t"  ).expect( "FAILED to write" );
	write!( fout, "{}", "State"     ).expect( "FAILED to write" );
	write!( fout, "{}", "\n"        ).expect( "FAILED to write" );

	for i in 0 .. ( *rsa_list ).len() {
		write!( fout, "{}\t"   , i + 1                          ).expect( "FAILED to write" );
		write!( fout, "{}\t"   , ( *chainid_list )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{}\t"   , ( *resseq_list  )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{}\t"   , ( *resname_list )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{:.3}\t", ( *asa_list     )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "{:.3}\t", ( *rsa_list     )[ i ] * 100.0 ).expect( "FAILED to write" );
		write!( fout, "{}"     , ( *state_list   )[ i ]         ).expect( "FAILED to write" );
		write!( fout, "\n"                                      ).expect( "FAILED to write" );
	}

	println!( "\nThe output file was correctly written.\n" );
}
