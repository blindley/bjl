pub mod grid;

#[test]
fn it_works() {
    let v = vec![1,2,3,4,5];
    slice_print(v.as_slice());
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

fn insertion_sort<T, F>(v: &mut [T], mut compare: F) where F: FnMut(&T, &T) -> Ordering {
    let len = v.len() as isize;
    let buf_v = v.as_mut_ptr();

    for i in range(1, len) {
        let mut j = i;
        unsafe {
            let read_ptr = buf_v.offset(i) as *const T;

            while j > 0 &&
                    compare(&*read_ptr, &*buf_v.offset(j - 1)) == Less {
                j -= 1;
            }

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
