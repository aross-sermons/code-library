class Node:
    """
    A class representing a node in a linked list.

    Attributes:
    -----------
    data : any
        The data stored in the node.
    previous : Node, optional
        The previous node in the linked list (default is None).
    next : Node, optional
        The next node in the linked list (default is None).
    """

    def __init__(self, data, previous:'Node'=None, next:'Node'=None):
        """
        Initializes a Node with given data and an optional next node.

        Parameters:
        -----------
        data : any
            The data to be stored in the node.
        previous : Node, optional
            The previous node in the linked list (default is None).
        next : Node, optional
            The next node in the linked list (default is None).
        """
        self.data = data
        self.previous = previous
        self.next = next

    def __str__(self) -> str:
        """
        Returns a user-friendly string representation of the Node.

        Returns:
        --------
        str
            A string representation of the node.
        """
        if self.next == None:
            return f'Node(data={self.data}, previous = {self.previous.data}, next={self.next})'
        if self.previous == None:
            return f'Node(data={self.data}, previous = {self.previous}, next={self.next.data})'
        return f'Node(data={self.data}, previous = {self.previous.data}, next={self.next.data})'
    
    def __repr__(self) -> str:
        """
        Returns an unambiguous string representation of the Node.

        Returns:
        --------
        str
            A string representation of the node suitable for debugging.
        """
        if self.next == None:
            return f'Node(data={self.data!r}, previous = {self.previous.data!r}, next={self.next!r})'
        if self.previous == None:
            return f'Node(data={self.data!r}, previous = {self.previous!r}, next={self.next!r})'
        return f'Node(data={self.data!r}, previous = {self.previous.data!r}, next={self.next.data!r})'
    
    def get_tail(self):
        """
        Returns the last node in the linked list starting from this node.

        Parameters:
        -----------
        self : Node
            This node.

        Returns:
        --------
        current_node : Node
            The last node in the linked list.
        """
        current_node = self
        while True:
            if current_node.next == None:
                return current_node
            current_node = current_node.next

    def get_head(self):
        """
        Returns the first node in the linked list starting from this node.

        Parameters:
        -----------
        self : Node
            This node.

        Returns:
        --------
        current_node : Node
            The first node in the linked list.
        """
        current_node = self
        while True:
            if current_node.previous == None:
                return current_node
            current_node = current_node.previous

    def append_node(self, new_node:'Node'):
        """
        Appends a new node onto the end of this node, then returns this node.

        Parameters:
        -----------
        self : Node
            This node.
        new_node : Node
            The node to add.

        Returns:
        --------
        self : Node
            This node.
        """
        if self.next == None:
            new_node.previous = self
            self.next = new_node
            return self
        last_node = self.next
        last_node.previous = new_node
        new_node.previous = self
        new_node.next = last_node
        self.next = new_node
        return self

    def forward_traverse(self):
        current_node = self
        while True:
            print(current_node)
            if current_node.next == None:
                break
            current_node = current_node.next

    def backward_traverse(self):
        current_node = self
        while True:
            print(current_node)
            if current_node.previous == None:
                break
            current_node = current_node.previous
        