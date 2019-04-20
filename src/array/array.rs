use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use shape::Shape;


/// Array structure
pub struct Array<T: Copy> {
    pub data: Rc<RefCell<Vec<T>>>,
    pub shape: Shape,
}

impl<T: Copy> Array<T> {
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

    /// Returns Shape instance
    #[inline]
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Returns vector representing array shape
    #[inline]
    pub fn get_shape(&self) -> &Vec<i32> {
        self.shape.get_shape()
    }

    /// Sets array shape
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing new array shape
    #[inline]
    pub fn reshape(&mut self, shape: Vec<i32>) -> &Array<T> {
        self.shape.set_shape(shape);
        return self;
    }

    /// Sets array shape
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing new array shape
    #[inline]
    pub fn set_shape(&mut self, shape: Vec<i32>) -> () {
        self.shape.set_shape(shape);
    }

    /// Validates indices have right dimension
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices
    fn check_indices_size(&self, indices: &Vec<usize>) -> () {
        let shape_len = self.shape.get_shape().len();

        if indices.len() / 2 > shape_len {
            panic!(
                "Invalid indices given: shape dimensions {}, indices dimensions {}",
                shape_len,
                indices.len() / 2
            );
        }
    }

    /// Set values on given indices to given value
    ///
    /// # Arguments
    ///
    /// * `indices` - vector of indices
    /// * `value` - value to fill it with
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use] extern crate numas;
    /// use numas::array::Array;
    ///
    /// let f_array = Array::new(vec![1,2,3,4,5,6,7,8,9], vec![3,3]);
    /// 
    /// // first row
    /// f_array.set(s![0], -1);
    /// assert_eq!(f_array.collect(), vec![-1,-1,-1,4,5,6,7,8,9]);
    ///
    /// let s_array = Array::new(vec![1,2,3,4,5,6,7,8,9], vec![3,3]);
    ///
    /// // fist two rows
    /// s_array.set(s![0 => 2], -1);
    /// assert_eq!(s_array.collect(), vec![-1,-1,-1,-1,-1,-1,7,8,9]);
    ///
    /// let t_array = Array::new(vec![1,2,3,4,5,6,7,8,9], vec![3,3]);
    ///
    /// // second row, second column
    /// t_array.set(s![1; 1], -1);
    /// assert_eq!(t_array.collect(), vec![1,2,3,4,-1,6,7,8,9]);
    ///
    /// let x_array = Array::new(vec![1,2,3,4,5,6,7,8,9], vec![3,3]);
    ///
    /// // last row, two last columns
    /// x_array.set(s![2; 1 => 3], -1);
    /// assert_eq!(x_array.collect(), vec![1,2,3,4,5,6,7,-1,-1]);
    /// ```
    pub fn set(&self, indices: Vec<usize>, value: T) -> () {
        self.check_indices_size(&indices);

        let shape = self.shape.indices_to_shape(indices);
        let mut data = self.data.borrow_mut();

        for i in shape.get_bounds() {
            data[i] = value;
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
            data[i] = value;
        }

        return self;
    }

    /// Return new Array from given indices
    ///
    /// # Arguments
    ///
    /// * `indices` - vector of indices
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use] extern crate numas;
    /// use numas::array::Array;
    ///
    /// let array = Array::new(vec![1,2,3,4,5,6,7,8,9], vec![3,3]);
    ///
    /// // first row
    /// assert_eq!(array.get(s![0]).collect(), vec![1,2,3]);
    ///
    /// // fist two rows
    /// assert_eq!(array.get(s![0 => 2]).collect(), vec![1,2,3,4,5,6]);
    ///
    /// // second row, second column
    /// assert_eq!(array.get(s![1; 1]).collect(), vec![5]);
    ///
    /// // last row, two last columns
    /// assert_eq!(array.get(s![2; 1 => 3]).collect(), vec![8,9]);
    /// ```
    pub fn get(&self, indices: Vec<usize>) -> Array<T> {
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
    #[inline]
    pub fn base_len(&self) -> usize {
        return self.data.borrow().len();
    }

    /// Creates view into array
    #[inline]
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
    #[inline]
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

impl<T: Copy> Clone for Array<T> {
    /// Clones array object
    fn clone(&self) -> Array<T> {
        let data = self.data.borrow().to_vec().clone();

        return Array {
            data: Rc::new(RefCell::new(data)),
            shape: self.shape.clone(),
        };
    }
}

impl <T: fmt::Debug + Copy> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.data.borrow();
        return write!(f, "data: \t{:?}\nshape: \t{:?}", &data[self.shape.get_bounds()], self.get_shape());
    }
}
