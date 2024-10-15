/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
    length: u32,
}

impl<T: Ord + Clone> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            start: None,
            end: None,
            length: 0,
        }
    }

    fn push(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.start;
        let new_node = Some(Box::leak(new_node).into());
        self.start = new_node;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut list_c = LinkedList::new();
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        while ptr_a.is_some() && ptr_b.is_some() {
            let val_a = unsafe { ptr_a.unwrap().as_ref().val.clone() };
            let val_b = unsafe { ptr_b.unwrap().as_ref().val.clone() };
            if val_a < val_b {
                list_c.add(val_a);
                ptr_a = unsafe { ptr_a.unwrap().as_ref().next };
            } else {
                list_c.add(val_b);
                ptr_b = unsafe { ptr_b.unwrap().as_ref().next };
            }
        }

        while let Some(ptr) = ptr_a {
            let val = unsafe { ptr.as_ref().val.clone() };
            list_c.add(val);
            ptr_a = unsafe { ptr.as_ref().next };
        }

        while let Some(ptr) = ptr_b {
            let val = unsafe { ptr.as_ref().val.clone() };
            list_c.add(val);
            ptr_b = unsafe { ptr.as_ref().next };
        }

        list_c
    }

    fn add(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = None;

        let new_node = Some(Box::leak(new_node).into());

        match self.end {
            None => {
                self.start = new_node;
                self.end = new_node;
            }
            Some(end) => unsafe {
                (*end.as_ptr()).next = new_node;
                self.end = new_node;
            },
        }

        self.length += 1;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        while let Some(node) = current {
            unsafe {
                write!(f, "{} -> ", node.as_ref().val)?;
                current = node.as_ref().next;
            }
        }
        write!(f, "None")
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}