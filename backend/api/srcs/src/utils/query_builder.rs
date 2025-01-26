pub struct QueryBuilder {
	queries: Vec<String>,
	result: String,
	updated: bool,
}

impl QueryBuilder {
	pub fn to_query<'a>(&'a mut self) -> &'a str {
		self.update_result();

		&self.result
	}

	fn update_result(&mut self) {
		if self.updated == true {
			self.result = self.queries.join("\n");
			self.updated = false;
		}
	}
}

impl QueryBuilder {

	pub fn where_(mut self, condition: &str) -> Self {
		self.push(format!(
			"WHERE {}",
			condition
		));

		self
	}

	pub fn limit(mut self, limit: u32) -> Self {
		self.push(format!(
			"LIMIT {}",
			limit,
		));

		self
	}

	pub fn having(mut self, condition: &str) -> Self {
		self.push(format!(
			"HAVING {}",
			condition,
		));

		self
	}

	pub fn group_by(mut self, column: &str) -> Self {
		self.push(format!(
			"GROUP BY {}",
			column,
		));

		self
	}



	fn push(&mut self, value: String) {
		self.updated = true;

		self.queries.push(value)
	}
}


//--- INIT ---//

impl QueryBuilder {
	
	pub fn select(table: &str, columns: &str) -> Self {
		let query = format!(
			"SELECT {} FROM {}",
			columns,
			table,
		);

		Self {
			queries: vec![query],
			result: String::new(),
			updated: true,
		}
	}

	pub fn insert(table: &str, columns: &str, values: &str) -> Self {
		let query = format!(
			"INSERT INTO {} ({}) VALUES ({})",
			table,
			columns,
			values,
		);

		Self {
			queries: vec![query],
			result: String::new(),
			updated: true,
		}
	}

	pub fn delete(table: &str) -> Self {
		let query = format!(
			"DELETE FROM {}",
			table,
		);

		Self {
			queries: vec![query],
			result: String::new(),
			updated: true,
		}
	}

	pub fn update(table: &str, set: &str) -> Self {
		let query = format!(
			"UPDATE {} SET {}",
			table,
			set,
		);

		Self {
			queries: vec![query],
			result: String::new(),
			updated: true,
		}
	}

}
