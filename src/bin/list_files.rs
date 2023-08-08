#![feature(type_name_of_val)]
#![feature(iterator_try_collect)]
#![feature(test)]

use std::ffi::OsString;

fn list_files1(path: impl AsRef<std::path::Path>) -> Result<Vec<OsString>, std::io::Error> {
    std::fs::read_dir(path)?.map(|a| Ok(a?.path().into_os_string())).collect()
}

fn list_files2(path: impl AsRef<std::path::Path>) -> Result<Vec<OsString>, std::io::Error> {
    Ok(std::fs::read_dir(path)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|a| a.path().into_os_string())
        .collect())
}

fn list_files3(path: impl AsRef<std::path::Path>) -> Result<Vec<OsString>, std::io::Error> {
    use itertools::Itertools;
    std::fs::read_dir(path)?.map_ok(|dir| dir.path().into_os_string()).collect()
}

fn main() {
    let a = ["1", "two", "NaN", "four", "5"];
    let filter_map_out: Vec<i32> = a.iter().filter_map(|s| s.parse().ok()).collect();
    let flat_map_out: Vec<i32> = a.iter().flat_map(|s| s.parse()).collect();
    assert_eq!(filter_map_out, flat_map_out);

    let u: Vec<Result<i32, ()>> = vec![Ok(1), Ok(2), Ok(3)];
    let collect_out = u.clone().into_iter().collect::<Result<Vec<i32>, _>>();
    let try_out = u.into_iter().try_collect::<Vec<i32>>();
    assert_eq!(try_out, Ok(vec![1, 2, 3]));
    assert_eq!(collect_out, try_out);

    let u = vec![Ok(1), Ok(2), Err(()), Ok(3)];
    let collect_out = u.clone().into_iter().collect::<Result<Vec<i32>, _>>();
    let try_out = u.into_iter().try_collect::<Vec<i32>>();
    assert_eq!(try_out, Err(()));
    assert_eq!(collect_out, try_out);

    let path = "/home/qwox/.config/nvim/templates";
    let a = list_files1(path);
    dbg!(a);
    let a = list_files2(path);
    dbg!(a);
    let a = list_files3(path);
    dbg!(a);
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::*;

    #[bench]
    fn ls1(b: &mut Bencher) {
        let path = "/home/qwox/.config/nvim/templates";
        b.iter(|| black_box(list_files1(black_box(path))));
    }

    #[bench]
    fn ls2(b: &mut Bencher) {
        let path = "/home/qwox/.config/nvim/templates";
        b.iter(|| black_box(list_files2(black_box(path))));
    }

    #[bench]
    fn ls3(b: &mut Bencher) {
        let path = "/home/qwox/.config/nvim/templates";
        b.iter(|| black_box(list_files3(black_box(path))));
    }
}
