// could create a struct/trait - prefer a trait
/*
new
add
pop
remove
remove_at
get_at
insert_at
append
prepend
*/

struct ArrayList<T, const N: usize>{
    data: Vec<T>
}

impl<T : Clone, const N: usize> ArrayList<T, N>  {
    //create a new array list with size N
    fn new() -> Self {
        ArrayList {
            data: Vec::with_capacity(N),
        }
    }
    fn add(&mut self, item: T) -> Result<(), &'static str>{
        if self.data.len()< N {
            self.data.push(item);
            return Ok(());
        }else{
            return Err("Array is full");
        }
    }
    fn pop(&mut self) -> Option<T>{
        self.data.pop()
    }
    fn remove_at(&mut self, index:usize) -> Option<T>{
        if index<self.data.len(){
            return Some(self.data.remove(index));
        }else{
            return None;
        }
    }
    fn get_at(&self, index:usize) -> Option<&T>{
        if index<self.data.len(){
            return Some(&self.data[index]);
        }else{
            return None;
        }
    }
    fn insert_at(&mut self, index:usize, item:T) -> Result<(), &'static str>{
        if index<self.data.len(){
            self.data.insert(index, item);
            return Ok(());
        }else{
            return Err("Index out of bounds");
        }
    }

    fn append(&mut self, other: &mut ArrayList<T, N>) -> Result<(), &'static str>{
        if self.data.len() + other.data.len() <= N {
            self.data.append(&mut other.data);
            return Ok(());
        }else{
            return Err("Appending would exceed capacity");
        }
    }

    fn prepend(&mut self, other: &mut ArrayList<T, N>) -> Result<(), &'static str>
    where
        T: Clone, // Add trait bound T: Clone
    {
        if self.data.len() + other.data.len() <= N {
            other.data.append(&mut self.data);
            self.data = other.data.clone();
            return Ok(());
        } else {
            return Err("Prepending would exceed capacity");
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let list: ArrayList<i32, 10> = ArrayList::<i32, 10>::new();
        assert_eq!(list.data.len(), 0);
    }

    #[test]
    fn test_add(){
        let mut list: ArrayList<i32, 2> = ArrayList::<i32, 2>::new();
        list.add(4).unwrap();
        list.add(5).unwrap();
        assert_eq!(list.data.len(), 2);
        assert_eq!(list.data[0], 4);
        assert_eq!(list.data[1], 5);
        match list.add(3) {
            Err(err_msg) => {
                assert_eq!(err_msg, "Array is full");
            }
            _ => {
                panic!("Expected error not returned");
            }
        }
    }


    #[test]
    fn test_pop(){
        let mut list: ArrayList<i32, 2> = ArrayList::<i32, 2>::new();
        list.add(4).unwrap();
        list.add(5).unwrap();
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_remove_at(){
        let mut list: ArrayList<i32, 2> = ArrayList::<i32, 2>::new();
        list.add(4).unwrap();
        list.add(5).unwrap();
        assert_eq!(list.remove_at(0), Some(4));
        assert_eq!(list.remove_at(0), Some(5));
        assert_eq!(list.remove_at(0), None);
    }

    #[test]
    fn test_get_at(){
        let mut list: ArrayList<i32, 2> = ArrayList::<i32, 2>::new();
        list.add(4).unwrap();
        list.add(5).unwrap();
        assert_eq!(list.get_at(0), Some(&4));
        assert_eq!(list.get_at(1), Some(&5));
        assert_eq!(list.get_at(2), None);
    }

    #[test]
    fn test_insert_at(){
        let mut list: ArrayList<i32, 2> = ArrayList::<i32, 2>::new();
        list.add(4).unwrap();
        list.add(5).unwrap();
        list.insert_at(1, 6).unwrap();
        assert_eq!(list.data.len(), 3);
        assert_eq!(list.data[0], 4);
        assert_eq!(list.data[1], 6);
        assert_eq!(list.data[2], 5);
        match list.insert_at(4, 3) {
            Err(err_msg) => {
                assert_eq!(err_msg, "Index out of bounds");
            }
            _ => {
                panic!("Expected error not returned");
            }
        }
    }

    #[test]
    fn test_append(){
        let mut list1: ArrayList<i32, 4> = ArrayList::<i32, 4>::new();
        list1.add(4).unwrap();
        list1.add(5).unwrap();
        let mut list2: ArrayList<i32, 4> = ArrayList::<i32, 4>::new();
        list2.add(6).unwrap();
        list2.add(7).unwrap();
        list1.append(&mut list2).unwrap();
        assert_eq!(list1.data.len(), 4);
        assert_eq!(list1.data[0], 4);
        assert_eq!(list1.data[1], 5);
        assert_eq!(list1.data[2], 6);
        assert_eq!(list1.data[3], 7);
        assert_eq!(list2.data.len(), 0);
        list2.add(9).unwrap();
        match list1.append(&mut list2) {
            Err(err_msg) => {
                assert_eq!(err_msg, "Appending would exceed capacity");
            }
            _ => {
                panic!("Expected error not returned");
            }
        }
    }
    
    #[test]
    fn test_prepend(){
        let mut list1: ArrayList<i32, 4> = ArrayList::<i32, 4>::new();
        list1.add(4).unwrap();
        list1.add(5).unwrap();
        let mut list2: ArrayList<i32, 4> = ArrayList::<i32, 4>::new();
        list2.add(6).unwrap();
        list2.add(7).unwrap();
        list1.prepend(&mut list2).unwrap();
        assert_eq!(list1.data.len(), 4);
        assert_eq!(list1.data[0], 6);
        assert_eq!(list1.data[1], 7);
        assert_eq!(list1.data[2], 4);
        assert_eq!(list1.data[3], 5);
    }


}


