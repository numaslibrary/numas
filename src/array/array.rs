use std::cmp;
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
        let bounds: Vec<(usize, usize, usize)> = vec![(0, 1, length)];

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
    pub fn reshape(&mut self, shape: Vec<i32>) -> () {
        self.shape.set_shape(shape);
    }

    /// Sets array shape
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing new array shape
    pub fn set_shape(&mut self, shape: Vec<i32>) -> () {
        self.shape.set_shape(shape);
    }

    /// Returns linear vector from given indices
    ///
    /// # Arguments
    ///
    /// * `indices` - vector of indices
    pub fn get_raw(&self, indices: Vec<Vec<usize>>) -> Vec<T> {
        let (start, count) = self.shape.get_index(indices);
        let data = self.data.borrow().clone();

        println!("{:?}", start);

        println!("{:?}", count);
        return data[start..(count + start)].to_vec();
    }

    pub fn set(&mut self, indices: Vec<Vec<usize>>, mut values: Vec<T>) -> () {
        let (start, count) = self.shape.get_index(indices);
        let range = 0..cmp::min(count, values.len());
        let mut data = self.data.borrow_mut();

        for (index, value) in values.drain(range).enumerate() {
            data[start + index] = value;
        }
    }

    pub fn get(&self, indices: Vec<Vec<usize>>) -> Array<T> {
        let mut use_view = true;

        for (i, value) in indices.iter().skip(1).enumerate() {
            if indices[i - 1].len() > indices[i].len() {
                use_view = false;
                break;
            }
        }

        if use_view {
            let (start, count) = self.shape.get_index(indices);
            return self.bounded_view(self.get_shape().clone(), start, start + count);
        } else {
            return self.clone();
        }
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
    pub fn bounded_view(&self, shape: Vec<i32>, start: usize, end: usize) -> Array<T> {
        return Array {
            data: self.data.clone(),
            shape: Shape::new(shape, start, end),
        }
    }

    pub fn set_t(&mut self, index: usize, value: T) -> () {
        let mut data = self.data.borrow_mut();
        data[index] = value;
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
        let shape = self.shape.get_shape();
        let count = shape.iter().last();

//        match count {
//            None => {
//                writeln!(f, "Invalid array instance");
//            },
//
//            Some(c) => {
//                for index in 0..shape.len() {
//                    writeln!(f, "{}[", "\t".repeat(index));
//                }
//
//                for (index, value) in shape.iter().enumerate() {
//                    writeln!(f, "{}[", "\t".repeat(index));
//                }
//
//                for index in (0..shape.len()).rev() {
//                    writeln!(f, "{}]", "\t".repeat(index));
//                }
//            },
//        }

        let data = self.data.borrow();
        return write!(f, "{:?}", &data[self.shape.get_bounds()]);
    }
}
