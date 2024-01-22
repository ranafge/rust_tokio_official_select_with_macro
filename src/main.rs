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

// Basic stream selecting

use tokio_stream:: {self as stream, StreamExt};

// #[tokio::main]
// async fn main() {
//     let mut stream1 = stream::iter(vec![1,2,3]);
//     let mut stream2 = stream::iter(vec![4,5,6]);

//     let next = tokio::select! {
//         v = stream1.next() => v.unwrap(),
//         v = stream2.next() => v.unwrap()
//     };

//     assert!(next == 1 || next == 4)
// }

//  collect the contents of two streams.

// #[tokio::main]
// async fn main() {
//     let mut stream1 = stream::iter(vec![1,2,2]);
//     let mut stream2 = stream::iter(vec![4,5,6]);

//     let mut values = vec![];

//     loop {
//         tokio::select! {
//             Some(v) =  stream1.next() => {
//                 values.push(v)
//             }
//             Some(v) = stream2.next() => values.push(v),
//             else => break,

//         }
//     }
//     values.sort();
//     assert_eq!(&vec![1,2,3,4,5,6], &values[..])
// }


use tokio::time::{self, Duration, Sleep};

// #[tokio::main]
// async fn main() {
//     let mut stream = stream::iter(vec![1,2,3,4,5,6,7]);
//     let sleep = time::sleep(Duration::from_millis(1));

//     tokio::pin!(sleep);

//     loop {
//         tokio::select! {
//             maybe_v = stream.next() => {
//                 if let Some(v) = maybe_v {
//                     println!("got = {}", v);
//                 }else {
//                     break;
//                 }
//             }
//             _= &mut sleep => {
//                 println!("time out");
//                 break
//             }
//         }
//     }
// }

// resutl
// got = 1
// got = 2
// got = 3
// got = 4
// got = 5
// got = 6
// time out

// joing two values using select!

use tokio::sync::oneshot;

// #[tokio::main]

// async fn main() {
//     let (tx1, mut rx1) = oneshot::channel();
//     let (tx2, mut rx2) = oneshot::channel();

//     tokio::spawn(async move {
//         tx1.send("first").unwrap()
//     });
//     tokio::spawn(async move {
//         tx2.send("second").unwrap()
//     });

//     let mut a = None;
//     let mut b  = None;
//     // continue looping untill receive rx1 and rx2
//     while  a.is_none() || b.is_none()   {
//         tokio::select! {
//             // receive rx1 if a is still none
//             v1 = (&mut rx1), if a.is_none() => a = Some(v1.unwrap()),
//             // receive rx2 if b is still none
//             v2 = (&mut rx2), if b.is_none() => b = Some(v2.unwrap())
//         }
//     }
//     // unwrap a and b value into a res tuple
//     let res = (a.unwrap(), b.unwrap());
//     assert_eq!(res.0, "first");
//     assert_eq!(res.1, "second");
// }


// Using the biased; mode to control polling order.
// #[tokio::main]
// async fn main(){
//     let mut count  =0u8;

//     loop {
//         tokio::select! {
//             // if you run this example without `biased;`, the polling order is
//             //pseudo-random, and the assertions on the value of count will 
//             // (probably) fail

//             biased;

//             _= async {}, if count < 1 => {
//                 count += 1;
//                 assert_eq!(count, 1)
//             }

//             _= async {} ,if count < 2 => {
//                 count += 1;
//                 assert_eq!(count, 2)
//             }
//             _= async {}, if count < 3 => {
//                 count +=1;
//                 assert_eq!(count, 3)
//             }
//             else => {
//                 break;
//             }

//         }
//     }

// }

// if you need to poll future orderly need to set biased:

// avoid racy if preconditons


async fn some_async_workd() {
    println!("some async workd")
}

#[tokio::main]
// async fn main() {
//     let sleep = time::sleep(Duration::from_millis(50));
//     tokio::pin!(sleep);

//     while !sleep.is_elapsed() {
//         tokio::select! {
//             _ = &mut sleep, if !sleep.is_elapsed() => {
//                 println!("operation time out");
//             }
//             _= some_async_workd() => {
//                 println!("operaiotn completed");
//             }
//         }
//     }
//     panic!("This example shows how not to do it!")
// }

// this will cause for race condition

async fn main() {
    let sleep = time::sleep(Duration::from_millis(20));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            _= &mut sleep => {
                println!("operaton time out");
                break;
            }
            _= some_async_workd() => {
                println!("operatio completed")
            }
        }
    }
}