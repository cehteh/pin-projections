Yet another pin projection helper macro

Creates projection functions for pinned objects.

Example usage:
```
# use pin_projections::project;
use std::pin::Pin;

// Just a placeholder for illustration
struct Entry(u64);

// The structure to be projected
struct Example {
    structural_pinned: Entry,
    not_structural_pinned: Entry,
}

impl Example {
    // The projections are defined within impl

    // defining a projection for structural_pinned
    // for a pinned shared reference
    project!(structural_pinned as first_entry() -> Pin<&Entry>);

    // and one for a pinned mutable reference.
    project!(structural_pinned as first_entry_mut() -> Pin<&mut Entry>);

    // without the 'as function()' part the projection is named the same as the member
    project!(structural_pinned -> Pin<&Entry>);

    // When no projection name is given then mutable and immutable projections are
    // mutually exclusive. The following would then collide with the definition above.
    // project!(structural_pinned -> Pin<&mut Entry>);

    // non structural pinned members are similar, just without the Pin
    project!(not_structural_pinned as second_entry() -> &Entry);

    // and one for a pinned mutable reference.
    project!(not_structural_pinned as second_entry_mut() -> &mut Entry);
}
```
