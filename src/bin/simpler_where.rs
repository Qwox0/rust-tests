use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
struct DataImpl {
    value: isize,
}

// similar to the article:

trait DataBad {
    type Item;
    fn add(&mut self, rhs: Self::Item);
}

impl DataBad for DataImpl {
    type Item = isize;

    fn add(&mut self, rhs: Self::Item) {
        self.value += rhs;
    }
}

/// where trait bounds are way to complicated
fn send_data_bad<D>(data: D, inc: D::Item)
where
    D: DataBad + Send + Sync + Debug + 'static,
    D::Item: Send + Sync + Debug + Copy + 'static,
{
    let data = Arc::new(Mutex::new(data));
    let handles = (0..3)
        .map(|_| {
            let data = data.clone();
            thread::spawn(move || {
                data.lock().unwrap().add(inc);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        let _ = handle.join();
    }

    let data = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
    println!("{:?}", data);
}

// good solution:

/// similar to [`DataBad`] but with trait bounds.
/// only place where trait bound are needed
trait DataGood: Debug + Send + Sync + 'static {
    type Item: Send + Sync + Copy + 'static;
    fn add(&mut self, rhs: Self::Item);
}

impl DataGood for DataImpl {
    type Item = isize;

    fn add(&mut self, rhs: Self::Item) {
        self.value += rhs;
    }
}

/// only [`DataGood`] trait bound -> no where needed
/// the body is identical to [`send_data_bad`]
fn send_data_good<D: DataGood>(data: D, inc: D::Item) {
    let data = Arc::new(Mutex::new(data));
    let handles = (0..3)
        .map(|_| {
            let data = data.clone();
            thread::spawn(move || {
                data.lock().unwrap().add(inc);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        let _ = handle.join();
    }

    let data = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
    println!("{:?}", data);
}

fn main() {
    let data = DataImpl { value: 10 };
    send_data_bad(data, 2);
    let data = DataImpl { value: 10 };
    send_data_good(data, 2);
}
