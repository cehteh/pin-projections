Yet another pin projection helper macro

Creates zero-cost projection functions for pinned objects.

Example usage:
```
use pin_projections::project;
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
    // 1. for a pinned shared reference
    project!(structural_pinned as first_entry() -> Pin<&Entry>);

    // 2. and one for a pinned mutable reference.
    project!(structural_pinned as first_entry_mut() -> Pin<&mut Entry>);

    // 3. without the 'as function()' part the projection is named the same as the member
    project!(structural_pinned -> Pin<&Entry>);

    // When no projection name is given then mutable and immutable projections are
    // mutually exclusive. The following would then collide with the definition above.
    // project!(structural_pinned -> Pin<&mut Entry>);

    // 4. non structural pinned members are similar, just without the Pin<>
    project!(not_structural_pinned as second_entry() -> &Entry);

    // 5. one for a mutable reference.
    project!(not_structural_pinned as second_entry_mut() -> &mut Entry);

    // 6. all projections can be defined unsafe if necessary.
    project!(unsafe structural_pinned as unsafe_projection() -> &mut Entry);
}

fn main() {
    let mut example = Box::pin(
        Example{
             structural_pinned: Entry(42),
             not_structural_pinned: Entry(99),
        }
    );

    // This is Pin<&Example>
    let example_ref = example.as_ref();

    // for 1.
    assert_eq!(example_ref.first_entry().0, 42);

    // for 3.
    assert_eq!(example_ref.structural_pinned().0, 42);

    // for 4.
    assert_eq!(example_ref.second_entry().0, 99);
}
```
