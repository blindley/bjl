pub mod grid;

#[test]
fn it_works() {
    
}

fn insertion_sort<T, F>(v: &mut [T], mut compare: F) where F: FnMut(&T, &T) -> Ordering {
    let len = v.len() as isize;
    let buf_v = v.as_mut_ptr();

    // 1 <= i < len;
    for i in range(1, len) {
        // j satisfies: 0 <= j <= i;
        let mut j = i;
        unsafe {
            // `i` is in bounds.
            let read_ptr = buf_v.offset(i) as *const T;

            // find where to insert, we need to do strict <,
            // rather than <=, to maintain stability.

            // 0 <= j - 1 < len, so .offset(j - 1) is in bounds.
            while j > 0 &&
                    compare(&*read_ptr, &*buf_v.offset(j - 1)) == Less {
                j -= 1;
            }

            // shift everything to the right, to make space to
            // insert this value.

            // j + 1 could be `len` (for the last `i`), but in
            // that case, `i == j` so we don't copy. The
            // `.offset(j)` is always in bounds.

            if i != j {
                let tmp = ptr::read(read_ptr);
                ptr::copy_memory(buf_v.offset(j + 1),
                                 &*buf_v.offset(j),
                                 (i - j) as usize);
                ptr::copy_nonoverlapping_memory(buf_v.offset(j),
                                                &tmp,
                                                1);
                mem::forget(tmp);
            }
        }
    }
}

fn slice_print<T: fmt::String>(v: &[T]) {
    print!("[ ");
    let len = v.len();
    if len == 0 {
        println!("]");
        return;
    }

    print!("{}", v[0]);
    for i in range(1, len) {
        print!(", {}", v[i]);
    }

    println!(" ]");
}