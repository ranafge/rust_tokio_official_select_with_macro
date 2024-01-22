/*
    `tokio::select` waits for multiple concurrenct branches, returing when the  `first` branch completes
    cancelling the remaining branches.
    The `Select!` macro must be used inside of asynch functions, clousers, and blocks.
    The `Select!` macro accepts one or more branches with the following pattern:
    <pattern> = <aync operation> (, if <precondition>)? => <handler>,
    Additionally the `select!` macro may include a single, optional else branch, which evaluates if none of the 
    other branches match their patterns.

    Additionaly, each branch may include an optional if precondition. if the precondition returns false, then
    the branch is disabled. async expression is still evaluated but the resulting future is never polled.



The complete lifecycle of a select! expression is as follows:

Evaluate all provided <precondition> expressions. If the precondition returns false, disable the branch for the remainder of the current call to select!. Re-entering select! due to a loop clears the “disabled” state.
Aggregate the <async expression>s from each branch, including the disabled ones. If the branch is disabled, <async expression> is still evaluated, but the resulting future is not polled.
Concurrently await on the results for all remaining <async expression>s.
Once an <async expression> returns a value, attempt to apply the value to the provided <pattern>, if the pattern matches, evaluate <handler> and return. If the pattern does not match, disable the current branch and for the remainder of the current call to select!. Continue from step 3.
If all branches are disabled, evaluate the else expression. If no else branch is provided, panic.


`select!` randomly picks a branch to check first.
*/

async fn do_stuff_async() {
    println!("DO STUFF ASYNC")
}

async fn more_async_work() {
    println!("MORE ASYNC WORK")
}

// #[tokio::main]

// async fn main() {
//     tokio::select! {
//         _ = do_stuff_async() => {
//             println!("PRINTINF FROM DO_STRUFF_ASYNC COMPLETE FIST")
//         }
//         _= more_async_work() => {
//             println!(" MORE ASYNC WORK COMPLETE FIRST")
//         }
//     }
// }

