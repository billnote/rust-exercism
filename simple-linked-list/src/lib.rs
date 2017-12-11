pub struct SimpleLinkedList<T>
where
    T: Clone,
{
    head: Option<Box<Node<T>>>,
    size: usize,
}

#[derive(Clone)]
struct Node<T>
where
    T: Clone,
{
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        let node = Node {
            data: element,
            next: match self.head {
                None => None,
                Some(ref h) => Some(h.clone()),
            },
        };
        self.head = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut new_head: Option<Box<Node<T>>> = None;
        let mut result: Option<T> = None;

        if let Some(ref h) = self.head {
            if let Some(ref n) = h.next {
                new_head = Some(n.clone());
            }
            self.size -= 1;
            result = Some(h.data.clone());
        }

        self.head = new_head;
        result
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(ref h) = self.head {
            Some(&h.data)
        } else {
            None
        }
    }

    pub fn clone_data_to(&self, clone_data: &mut Vec<T>) {
        fn clone_node_data_to<T>(node: &Option<Box<Node<T>>>, clone_node_data: &mut Vec<T>)
        where
            T: Clone,
        {
            if let Some(ref node) = *node {
                clone_node_data.push(node.data.clone());
                clone_node_data_to(&node.next, clone_node_data);
            }
        }

        clone_node_data_to(&self.head, clone_data);
    }
}



impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();
        let mut nodes = Vec::<T>::new();

        self.clone_data_to(&mut nodes);
        nodes.iter().for_each(|data| new_list.push(data.clone()));

        new_list
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        item.iter().for_each(|data| list.push(data.clone()));

        list
    }
}

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut nodes = Vec::<T>::new();
        self.clone_data_to(&mut nodes);
        nodes.reverse();

        nodes
    }
}
