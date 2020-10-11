pub fn arraycopy<T>(
    src: &Vec<T>,
    src_pos: usize,
    dest: &mut Vec<T>,
    dest_post: usize,
    length: usize,
) where
    T: Copy + Default,
{
    if length + dest_post > dest.len() {
        dest.resize(length + dest_post, T::default());
    }

    for i in 0..length {
        dest[i + dest_post] = src[i + src_pos];
    }
}
