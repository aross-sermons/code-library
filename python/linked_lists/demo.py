import singly_linked_list, doubly_linked_list

def generate_singly_linked_list_from_array(arr):
    head = singly_linked_list.Node(arr[0])
    index = 1
    while index < len(arr):
        temp_node = head.get_tail()
        temp_node.next = singly_linked_list.Node(arr[index])
        index += 1
    return head

def singly_linked_list_demo():
    my_arr = [3, 7, 6, 0, 4]
    single_list = generate_singly_linked_list_from_array(my_arr)

    print("Forward Traverse Sinlgy Linked List:")
    single_list.forward_traverse()

    print("\nBackward Traverse Recursive Singly Linked List:")
    single_list.backward_traverse()

    print("\nAppending '2' to the list.")
    single_list.get_tail().next = singly_linked_list.Node(2)

    print("\nForward Traverse Sinlgy Linked List:")
    single_list.forward_traverse()

    print(f'\nGet tail:\n{single_list.get_tail()}')

def generate_doubly_linked_list_from_array(arr):
    head = doubly_linked_list.Node(arr[0])
    index = 1
    while index < len(arr):
        temp_node = head.get_tail()
        temp_node.next = doubly_linked_list.Node(arr[index], temp_node)
        index += 1
    return head

def doubly_linked_list_demo():
    my_arr = [3, 7, 6, 0, 4]
    double_list = generate_doubly_linked_list_from_array(my_arr)

    print("Forward Traverse Doubly Linked List:")
    double_list.forward_traverse()

    print("\nBackward Traverse Doubly Linked List")
    double_list.get_tail().backward_traverse()

    print("\nAppending '2' to the tail and '9' to the head.")
    double_list.get_tail().append_node(doubly_linked_list.Node(2))
    double_list.append_node(doubly_linked_list.Node(9))

    print("\nForward Traverse Doubly Linked List:")
    double_list.forward_traverse()

    print(f'\nGet head:\n{double_list.get_head()}')

    print(f'\nGet tail:\n{double_list.get_tail()}')

doubly_linked_list_demo()