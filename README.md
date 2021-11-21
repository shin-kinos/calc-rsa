# calc-rsa 

A Rust program that calculates residues' Accessible Surface Area (ASA) and Relative Solvent Accessibility (RSA) in a protein structure by using Voronota. 

## Description 

* It calculates residues' ASA and RSA in a protein structure. 
* The calculation method is based on Voronota and it's algorithm [1, 2]. 

## Requirements 

This program uses **Voronota** tool. Please make it executable by setting PATH. 

* Voronota ( https://bioinformatics.lt/wtsam/vorocontacts/help/standalone ) 

## Installation 

You can compile this program with `Cargo`. 📦🦀 

[e.g.] 

```
% cd calc-rsa-main 
% cargo build --release 
``` 

Then the object file is generated in `./target/release` directory. 

## Workflow 

### 1. Input a protein structure information 

The input file format is `mmCIF` or `PDB`. 

### 2. Voronota execution 

Voronota calculates the residues' contact surface areas and ASAs on the input protein structure. 

### 3. Calculate RSA 

Based on the Voronota's output information, the RSA of each residue is calculated. 

The RSA of each residue is scaled as a proportion to the ASA of Ala-X-Ala tripeptide. The Ala-X-Ala tripeptide ASAs (square Å) are below [3]: 

*  Ala : `110.2`
*  Arg : `229.0`
*  Asn : `146.4`
*  Asp : `144.1`
*  Cys : `140.4`
*  Gln : `178.6`
*  Glu : `174.7`
*  Gly : `78.7 `
*  His : `181.9`
*  Ile : `185.0`
*  Leu : `183.1`
*  Lys : `205.7`
*  Met : `200.1`
*  Phe : `200.7`
*  Pro : `141.9`
*  Ser : `117.2`
*  Thr : `138.7`
*  Trp : `240.5`
*  Tyr : `213.7`
*  Val : `153.7` 

### 4. Detect the states of the residues on the protein 

As additional information, the residues are derived into **three** types of residue state on the protein, buried (RSA < 9%), intermediate (9% ≦ RSA ≦ 36%) or exposed (RSA > 36%), based on Rost and Sander's description [4]. 

## Input file format 

`mmCIF` or `PDB`. 

See some example files in `demo` directoly. 

## Usage 

Options : 

`-i` : Input filename, REQUIRED. 

`-o` : Output filename, REQUIRED. 

`-f` : Input file format ("mmcif" or "pdb", default "mmcif"). 

`-h` : Print this help, ignore all other arguments.

[e.g.] 

``` 
% ./calc-rsa -i 6y2e.cif -o output -f mmcif
``` 

## Output file format 

Number`\t`Chain ID`\t`Residue number`\t`Residue name`\t`ASA (Å^2)`\t`RSA (%)`\t`State

[e.g. in `6vz8.cif`]

```
Number	ChainID	ResSeq	ResName	ResASA	ResRSA	State
1	D	87	PHE	123.587	61.578	exposed
2	D	88	ILE	143.863	77.764	exposed
3	D	89	SER	52.191	44.532	exposed
4	D	90	ARG	110.269	48.152	exposed
5	D	91	PHE	55.610	27.708	intermediate
6	D	92	ALA	38.437	34.879	intermediate
7	D	93	PRO	125.483	88.431	exposed
8	D	94	ASP	109.431	75.941	exposed
9	D	95	GLN	77.522	43.405	exposed
10	D	96	PRO	74.338	52.388	exposed
11	D	97	ARG	9.907	4.326	buried
12	D	98	LYS	76.347	37.116	exposed
13	D	99	GLY	2.420	3.074	buried
14	D	100	ALA	7.927	7.193	buried
15	D	101	ASP	13.114	9.101	intermediate
16	D	102	ILE	3.951	2.136	buried
17	D	103	LEU	19.058	10.408	intermediate
18	D	104	VAL	4.990	3.247	buried
19	D	105	GLU	10.533	6.029	buried
20	D	106	ALA	7.953	7.217	buried

```   

## References 

1. Olechnovič, Kliment, and Česlovas Venclovas. "Voronota: a fast and reliable tool for computing the vertices of the Voronoi diagram of atomic balls." Journal of computational chemistry 35.8 (2014): 672-681. 
2. Olechnovič, Kliment, and Česlovas Venclovas. "VoroContacts: a tool for the analysis of interatomic contacts in macromolecular structures." Bioinformatics (2021). 
3. Ahmad, Shandar, et al. "ASAView: database and tool for solvent accessibility representation in proteins." BMC bioinformatics 5.1 (2004): 1-5. 
4. Rost, Burkhard, and Chris Sander. "Conservation and prediction of solvent accessibility in protein families." Proteins: Structure, Function, and Bioinformatics 20.3 (1994): 216-226. 
