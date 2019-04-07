use shape::Shape;

impl PartialEq for Shape {
    /// Returns true if two shapes have equal dimensions
    ///
    /// # Arguments
    ///
    /// * `other` - shape to compare shape with
    fn eq(&self, other: &Shape) -> bool {
        let f_shape = self.get_shape();
        let s_shape = other.get_shape();

        if f_shape.len() == s_shape.len() {
            return f_shape
                .iter()
                .zip(s_shape.iter())
                .all(|(f, s)| f == s);
        }

        return false;
    }
}

impl Eq for Shape {}