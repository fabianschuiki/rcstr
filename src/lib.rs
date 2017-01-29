// Copyright (c) 2017 Fabian Schuiki

//! A reference counted string that acts like a regular str slice, hiding the
//! fact that it is wrapped in `Rc`.
//!
//! # Example
//! ```
//! use rcstr::RcStr;
//! use std::collections::HashSet;
//!
//! let mut map: HashSet<RcStr> = HashSet::new();
//! map.insert(RcStr::new("foo"));
//!
//! assert!(map.contains("foo"));
//! assert!(map.contains(&RcStr::new("foo")));
//! assert!(!map.contains("bar"));
//! ```

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Hash, PartialEq, PartialOrd)]
pub struct RcStr(Rc<String>);

impl RcStr {
	/// Create a new ref-counted string which is a copy of `value`.
	///
	/// # Example
	/// ```
	/// use rcstr::RcStr;
	/// use std::borrow::Borrow;
	/// let a = RcStr::new("foo");
	/// let b: &str = &a;
	/// assert_eq!(&*a, "foo");
	/// assert_eq!(*a, *"foo");
	/// assert_eq!(b, "foo");
	/// ```
	pub fn new<S: Into<String>>(value: S) -> RcStr {
		RcStr(Rc::new(value.into()))
	}
}

impl Eq for RcStr {}

impl Ord for RcStr {
	fn cmp(&self, other: &RcStr) -> Ordering {
		self[..].cmp(&other[..])
	}
}

impl fmt::Debug for RcStr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self[..].fmt(f)
	}
}

impl fmt::Display for RcStr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self[..].fmt(f)
	}
}

impl Borrow<str> for RcStr {
	fn borrow(&self) -> &str {
		&self.0[..]
	}
}

impl Deref for RcStr {
	type Target = str;
	fn deref(&self) -> &str {
		&self.0[..]
	}
}

impl AsRef<str> for RcStr {
	fn as_ref(&self) -> &str {
		&self.0[..]
	}
}
