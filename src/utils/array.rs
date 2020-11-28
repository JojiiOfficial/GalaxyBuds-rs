pub fn arraycopy<T>(src: &[T], src_pos: usize, dest: &mut Vec<T>, dest_post: usize, length: usize)
where
    T: Copy + Default,
{
    if length + dest_post > dest.len() {
        dest.resize(length + dest_post, T::default());
    }

    dest[dest_post..(length + dest_post)].clone_from_slice(&src[src_pos..(length + src_pos)]);

    /* for i in 0..length { */
    /* dest[i + dest_post] = src[i + src_pos]; */
    /* } */
}
