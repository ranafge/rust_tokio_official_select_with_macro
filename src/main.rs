/*
    `tokio::select` waits for multiple concurrenct branches, returing when the  `first` branch completes
    cancelling the remaining branches.
    The `Select!` macro must be used inside of asynch functions, clousers, and blocks.
    The `Select!` macro accepts one or more branches with the following pattern:
    <pattern> = <aync operation> (, if <precondition>)? => <handler>,
    Additionally the `select!` macro may include a single, optional else branch, which evaluates if none of the 
    other branches match their patterns.
*/