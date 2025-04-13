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
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    // 向链表尾部添加一个节点（注意：此处直接使用 Box::into_raw，故节点为堆分配，后续需手动释放，非本题重点）
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    // 返回索引 i 处的节点值的引用（使用递归实现）
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

    // 合并两个有序链表。注意：要求 T: Ord 用于比较
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let mut merged = LinkedList::new();
        // 新链表的总长度等于两个链表长度之和
        merged.length = list_a.length + list_b.length;
        
        // 用 current_a 和 current_b 遍历两个链表的节点指针
        let mut current_a = list_a.start;
        let mut current_b = list_b.start;
        let mut merged_start: Option<NonNull<Node<T>>> = None;
        let mut merged_end: Option<NonNull<Node<T>>> = None;
        
        // 定义一个闭包用于将节点追加到 merged 链表
        let mut push_node = |node: NonNull<Node<T>>| {
            // 将该节点的 next 设为 None（因为它即将成为新的尾节点）
            unsafe { (*node.as_ptr()).next = None; }
            if let Some(end) = merged_end {
                unsafe {
                    (*end.as_ptr()).next = Some(node);
                }
                merged_end = Some(node);
            } else {
                merged_start = Some(node);
                merged_end = Some(node);
            }
        };
        
        unsafe {
            // 当两个链表都非空时，比较头节点的值，将较小者追加到新链表中
            while let (Some(a_ptr), Some(b_ptr)) = (current_a, current_b) {
                if (*a_ptr.as_ptr()).val <= (*b_ptr.as_ptr()).val {
                    current_a = (*a_ptr.as_ptr()).next;
                    push_node(a_ptr);
                } else {
                    current_b = (*b_ptr.as_ptr()).next;
                    push_node(b_ptr);
                }
            }
            // 将剩余的节点追加到新链表中
            while let Some(a_ptr) = current_a {
                current_a = (*a_ptr.as_ptr()).next;
                push_node(a_ptr);
            }
            while let Some(b_ptr) = current_b {
                current_b = (*b_ptr.as_ptr()).next;
                push_node(b_ptr);
            }
        }
        
        merged.start = merged_start;
        merged.end = merged_end;
        merged
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
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
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        for &item in &vec_a {
            list_a.add(item);
        }
        for &item in &vec_b {
            list_b.add(item);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];
    
        for &item in &vec_a {
            list_a.add(item);
        }
        for &item in &vec_b {
            list_b.add(item);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
