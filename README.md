# rcstr

A reference counted string that acts like a regular str slice, hiding the fact that it is wrapped in Rc. Based on the rust compiler's `RcStr` type used in the name interner table.
