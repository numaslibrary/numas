use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use shape::Shape;


/// Array structure
pub struct Array<T: Clone> {
    pub data: Rc<RefCell<Vec<T>>>,
    pub shape: Shape,
}

impl<T: Clone> Array<T> {
    /// Creates new bounded array
    ///
    /// # Arguments
    ///
    /// * `data` - array elements
    /// * `shape` - vector representing array shape
    /// * `start` - start offset of array data
    /// * `end` - end offset of array data
    #[inline]
    pub fn new_bounded(data: Vec<T>, shape: Vec<i32>, start: usize, end: usize) -> Array<T> {
        return Array {
            data: Rc::new(RefCell::new(data)),
            shape: Shape::new(shape, start, end)
        };
    }

    /// Creates new array
    ///
    /// # Arguments
    ///
    /// * `data` - array elements
    /// * `shape` - vector representing array shape
    pub fn new(data: Vec<T>, shape: Vec<i32>) -> Array<T> {
        let length = data.len();

        return Array::new_bounded(data, shape, 0, length);
    }

    /// Returns vector representing array shape
    pub fn shape(&self) -> &Vec<i32> {
        self.shape.get_shape()
    }

    /// Returns vector representing array shape
    pub fn get_shape(&self) -> &Vec<i32> {
        self.shape.get_shape()
    }

    /// Sets array shape
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing new array shape
    pub fn reshape(&mut self, shape: Vec<i32>) -> &Array<T> {
        self.shape.set_shape(shape);
        return self;
    }

    /// Sets array shape
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing new array shape
    pub fn set_shape(&mut self, shape: Vec<i32>) -> () {
        self.shape.set_shape(shape);
    }

    /// Validates indices have right dimension
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices
    fn check_indices_size(&self, indices: &Vec<Vec<usize>>) -> () {
        let shape_len = self.shape.get_shape().len();

        if indices.len() > shape_len {
            panic!(
                "Invalid indices given: shape dimensions {}, indices dimensions {}",
                shape_len,
                indices.len()
            );
        }
    }

    /// Set values on given indices to given value
    ///
    /// # Arguments
    ///
    /// * `indices` - vector of indices
    /// * `value` - value to fill it with
    pub fn set(&self, indices: Vec<Vec<usize>>, value: T) -> () {
        self.check_indices_size(&indices);

        let shape = self.shape.indices_to_shape(indices);
        let mut data = self.data.borrow_mut();

        for i in shape.get_bounds() {
            data[i] = value.clone();
        }
    }

    /// Fills array with given value
    ///
    /// # Arguments
    ///
    /// * `value` - fill value
    pub fn fill(&self, value: T) -> &Array<T> {
        let mut data = self.data.borrow_mut();

        for i in self.shape.get_bounds() {
            data[i] = value.clone();
        }

        return self;
    }

    /// Return new Array from given indices
    ///
    /// # Arguments
    ///
    /// * `indices` - vector of indices
    pub fn get(&self, indices: Vec<Vec<usize>>) -> Array<T> {
        // Handle invalid indices length
        self.check_indices_size(&indices);

        let new_shape = self.shape.indices_to_shape(indices);

        return Array {
            shape: new_shape,
            data: self.data.clone(),
        };
    }

    /// Returns length of array
    pub fn len(&self) -> usize {
        return Shape::total_len(&self.shape) as usize;
    }

    /// Returns base length of array
    pub fn base_len(&self) -> usize {
        return self.data.borrow().len();
    }

    /// Creates view into array
    pub fn view(&self) -> Array<T> {
        return Array {
            data: self.data.clone(),
            shape: self.shape.clone(),
        }
    }

    /// Creates bounded view into array
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array shape
    /// * `start` - start offset of array data
    /// * `end` - end offset of array data
    pub fn bounded_view(&self, shape: &Vec<i32>, start: usize, end: usize) -> Array<T> {
        return Array {
            data: self.data.clone(),
            shape: Shape::new(shape.clone(), start, end),
        }
    }

    /// Collects elements of array into vector
    pub fn collect(&self) -> Vec<T> {
        let data = self.data.borrow();
        return data[self.shape.get_bounds()].to_vec();
    }
}

impl<T: Clone> Clone for Array<T> {
    /// Clones array object
    fn clone(&self) -> Array<T> {
        let data = self.data.borrow().to_vec().clone();

        return Array {
            data: Rc::new(RefCell::new(data)),
            shape: self.shape.clone(),
        };
    }
}

impl <T: fmt::Debug + Clone> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.data.borrow();
        return write!(f, "data: \t{:?}\nshape: \t{:?}", &data[self.shape.get_bounds()], self.get_shape());
    }
}
