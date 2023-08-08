/// not a real [`Iterator`]
pub struct TestIter {
    v: Vec<i16>,
}

fn next<'a>(i: &'a mut TestIter) -> Option<(i16, &'a i16)> {
    let item = i.v.pop();
    let item2 = i.v.last();
    item.zip(item2)
}

fn print(res: &Option<(i16, &i16)>) {
    println!("{:?}", res);
    if let Some((_, r)) = res {
        println!("{} -> {:?}", *r, *r as *const i16);
    }
}

fn check(iter: &mut TestIter, res: Option<(i16, &i16)>) {
    let n = next(iter);
    print(&n);
    assert_eq!(n, res);
}

fn main() {
    let mut iter = TestIter { v: vec![-10, 1, 2, 3, 4, 5] };

    check(&mut iter, Some((5, &4)));

    let n = next(&mut iter);
    print(&n);
    assert_eq!(n, Some((4, &3)));

    check(&mut iter, Some((3, &2)));

    // print(&n); // not allowed!

    check(&mut iter, Some((2, &1)));
    check(&mut iter, Some((1, &-10)));

    check(&mut iter, None);
    check(&mut iter, None);
}
