
use std::time::Instant;
//use colored::*;

mod accessibility;
mod error;
mod options;
mod result;
mod voronota;

fn main()
{
	println!( "\nCalclate residue Relative Surface Area ( RSA ) by using Voronota tool.\n" );
	println!( "Olechnovič, Kliment, and Česlovas Venclovas. \"VoroContacts: a tool for the analysis of interatomic contacts in macromolecular structures.\" Bioinformatics (2021)." );
	println!( "Olechnovič, Kliment, and Česlovas Venclovas. \"Voronota: a fast and reliable tool for computing the vertices of the Voronoi diagram of atomic balls.\" Journal of computational chemistry 35.8 (2014)." );

	/* Elapsed time : Start */
	let start = Instant::now();

	/* Set options. */
	let opts = options::Options::new();
	opts.show_parameter();

	/* Set Vectors for Voronota. */
	let mut data = voronota::Voronota::new();

	/* Execute Voronota. */
	voronota::exe_voronota(
		&( opts.input  ).as_str(),
		&( opts.output ).as_str(),
		&( opts.format ).as_str()
	);

	/* Check Voronota data. */
	data.check_voronota_data();

	/* Read Voronota output and Get infomation. */
	data.read_voronota_info( &( opts.output ).as_str() );

	/* Show Voronota purified solvent information. */
	//data.show_solvent_info();

	/* Make ASA list a residue. */
	let (
		chainid_list, // : Vec<String>,
		resseq_list , // : Vec<usize>,
		resname_list, // : Vec<String>,
		asa_list      // : Vec<f64>
	)
	= data.make_asa_list();

	/*
	for i in 0 .. chainid_list.len() {
		print!( "{}\t", chainid_list[ i ] );
		print!( "{}\t", resseq_list [ i ] );
		print!( "{}\t", resname_list[ i ] );
		print!( "{}\t", asa_list    [ i ] );
		print!( "\n"                      );
	}
	*/

	/* Calculate RSA. */
	let rsa_list : Vec<f64> = accessibility::calc_rsa( &resname_list, &asa_list );

	let state_list : Vec<&str> = accessibility::detect_state( &rsa_list );

	/*
	for i in 0 .. rsa_list.len() {
		print!( "{}\t"   , chainid_list[ i ] );
		print!( "{}\t"   , resseq_list [ i ] );
		print!( "{}\t"   , resname_list[ i ] );
		print!( "{:.3}\t", asa_list    [ i ] );
		print!( "{:.3}\t", rsa_list    [ i ] * 100.0 );
		print!( "{}\t"   , state_list  [ i ] );
		print!( "\n"                         );
	}
	*/

	/* Show result. */
	result::show_result(
		&chainid_list,
		&resseq_list ,
		&resname_list,
		&asa_list    ,
		&rsa_list    ,
		&state_list
	);

	/* Save result. */
	result::save_result(
		&chainid_list,
		&resseq_list ,
		&resname_list,
		&asa_list    ,
		&rsa_list    ,
		&state_list  ,
		&( opts.output )
	);

	println!( "\nProgram completed !!!\n" );

	/* Elapsed time : End */
	let end = start.elapsed();
	println!( "Total elapsed time : {:?}", end );

}
