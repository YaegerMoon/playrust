type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn append(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let last_link = self.last_link_mut();
            *last_link = Some(new_node);
        }
    }

    fn last_link_mut(&mut self) -> &mut Link<T> {
        let mut current = &mut self.head;

        // 다음 노드가 있는 동안 계속 이동
        while let Some(node) = current {
            if node.next.is_none() {
                // 다음이 없으면 현재 노드의 next가 우리가 찾는 '마지막 링크'
                return &mut node.next;
            }
            current = &mut node.next;
        }

        current
    }

    pub fn last(&self) -> Option<&T> {
        let mut cur = self.head.as_ref();
        let mut last_data = None;

        while let Some(node) = cur {
            last_data = Some(&node.data);
            cur = node.next.as_ref();
        }

        last_data
    }

    pub fn last_mut(&mut self) -> Option<&T> {
        let mut cur = self.head.as_mut();
        let mut last_data = None;

        while let Some(node) = cur {
            last_data = Some(&node.data);
            cur = node.next.as_mut();
        }
        last_data
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }
}

impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        // head를 꺼내오고 그 자리를 None으로 바꿈 (소유권 획득)
        let mut cur_link = self.head.take();

        // 루프를 돌며 노드를 하나씩 꺼내서 수동으로 해제
        while let Some(mut boxed_node) = cur_link {
            // 현재 노드가 가리키는 다음 노드의 소유권을 가져옴
            // 여기서 이전 노드(boxed_node)는 루프가 반복될 때 스코프를 벗어나며 안전하게 해제됨
            cur_link = boxed_node.next.take();
        }
    }
}
