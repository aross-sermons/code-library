class Node:
    """
    A class representing a node in a linked list.

    Attributes:
    -----------
    data : any
        The data stored in the node.
    next : Node, optional
        The next node in the linked list (default is None).
    """

    def __init__(self, data, next:'Node'=None):
        """
        Initializes a Node with given data and an optional next node.

        Parameters:
        -----------
        data : any
            The data to be stored in the node.
        next : Node, optional
            The next node in the linked list (default is None).
        """
        self.data = data
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
            return f'Node(data={self.data}, next={self.next})'
        return f'Node(data={self.data}, next={self.next.data})'
    
    def __repr__(self) -> str:
        """
        Returns an unambiguous string representation of the Node.

        Returns:
        --------
        str
            A string representation of the node suitable for debugging.
        """
        if self.next == None:
            return f'Node(data={self.data!r}, next={self.next!r})'
        return f'Node(data={self.data!r}, next={self.next.data!r})'
    
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

    def forward_traverse(self):
        """
        Prints each node in the linked list starting from this node.

        Parameters:
        -----------
        self : Node
            The starting node of the linked list.

        Prints:
        -------
        Prints data of each node in the list.
        """
        current_node = self
        while True:
            print(current_node)
            if current_node.next == None:
                break
            current_node = current_node.next

    def backward_traverse(self, previous_last:'Node'=None):
        """
        Prints each node in the linked list in reverse starting from the tail of this list.

        Parameters:
        -----------
        self : Node
            The starting node of the linket list.
        previous_last : Node
            The last node printed.

        Prints:
        -------
        Prints data of each node in the list.
        """
        current_node = self
        if previous_last == None:
            previous_last = self.get_tail()
            self.backward_traverse(previous_last)
        elif previous_last == self:
            print(self)
            return
        else:
            print(previous_last)
            while True:
                if current_node != None:
                    if current_node.next == previous_last:
                        self.backward_traverse(current_node)
                    current_node = current_node.next
                else:
                    break