// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use util::hash::H256;

/// General blockchain error type.
/// 
/// All of the errors, except Fatal, are recoverable.
#[derive(Debug)]
pub enum Error {
	/// Returned on irrecoverable error (eg. when db is malformed or in incorrect state).
	Fatal(FatalError),
	/// Returned when tree route could not be found.
	TreeRouteNotFound {
		/// Hash which is not a part of the tree.
		unknown_hash: H256
	}
}

impl From<FatalError> for Error {
	fn from(error: FatalError) -> Self {
		Error::Fatal(error)
	}
}

/// Irrecoverable errors.
#[derive(Debug)]
pub enum FatalError {
	/// Called when block details are expected to be found, and lack 
	/// may cause invalid blockchain state.
	MissingBlockDetails {
		/// Hash of missing block details.
		hash: H256
	}
}
