fn main() {
	let matrix = vec![vec![  1,  2,  3,  4, 5 ],
										vec![ 16, 17, 18, 19, 6 ], 
										vec![ 15, 24, 25, 20, 7 ], 
										vec![ 14, 23, 22, 21, 8 ], 
										vec![ 13, 12, 11, 10, 9 ]];

	println!("{:?}", snail(matrix));
}

fn snail(mut matrix: Vec<Vec<u32>>) -> Vec<u32> {
	let mut output: Vec<u32> = vec![];

	loop {
		output.append(&mut matrix.remove(0));

		if matrix.is_empty() { break }

		output.append(&mut matrix.iter_mut()
				.map(|list| list.pop().unwrap())
				.collect());

		let mut last_row = matrix.pop().unwrap();
		last_row.reverse();
		output.append(&mut last_row);

		if matrix.is_empty() { break }

		let mut left_column: Vec<u32> = matrix.iter_mut()
			.map(|list| list.remove(0))
			.collect();
		left_column.reverse();
		output.append(&mut left_column);
	}

	output
}


#[test]
fn test_snail_number_spiral() {
	let matrix =   vec![vec![  1,  2,  3,  4, 5 ],
											vec![ 16, 17, 18, 19, 6 ], 
											vec![ 15, 24, 25, 20, 7 ], 
											vec![ 14, 23, 22, 21, 8 ], 
											vec![ 13, 12, 11, 10, 9 ]];

	let expected_output: Vec<u32> = (1..=25).collect();
	assert_eq!(snail(matrix), expected_output);
}

#[test]
fn test_snail() {
	let matrix = vec![vec![ 1, 2, 3 ],
										vec![ 4, 5, 6 ],
										vec![ 7, 8, 9 ]];

	let expected_output = vec![ 1, 2, 3, 6, 9, 8, 7, 4, 5 ];
	assert_eq!(snail(matrix), expected_output);
}

#[test]
fn test_snail_even_length_matrix() {
	let matrix = vec![vec![  1,  2,  3,  4 ],
										vec![  5,  6,  7,  8 ],
										vec![  9, 10, 11, 12 ],
										vec![ 13, 14, 15, 16 ]];

	let expected_output = vec![ 1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10 ];
	assert_eq!(snail(matrix), expected_output);
}
