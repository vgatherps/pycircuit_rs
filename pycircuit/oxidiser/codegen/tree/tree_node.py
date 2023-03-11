from typing import Hashable, List, Set
from abc import ABC, abstractmethod


class CodeLeaf(ABC):
    @abstractmethod
    def generate_code(self) -> str:
        ...


# Inheriting hashable doesn't seem to work
class GlobalInitLeaf(ABC):
    @abstractmethod
    def generate_global_init_code(self) -> str:
        ...

    def key(self) -> Hashable:
        try:
            self.__hash__()
            return self
        except:
            raise NotImplementedError("Did not provide key and was not hashable")


class CodeTree(ABC):
    @abstractmethod
    def get_tree_children(self) -> List["TreeNode"]:
        ...


LeafNode = CodeLeaf | GlobalInitLeaf
TreeNode = CodeTree | LeafNode


class _GlobalInitTracker:

    # mypy wants this to have a pointless typing annotation?
    def __init__(self: "_GlobalInitTracker"):
        self.seen_globals: Set[GlobalInitLeaf] = set()
        self.ordered_globals: List[GlobalInitLeaf] = []

    def maybe_add_global(self, leaf: GlobalInitLeaf):
        if leaf not in self.seen_globals:
            self.seen_globals.add(leaf)
            self.ordered_globals.append(leaf)


def _generate_lines_from_node(
    node: TreeNode, global_init: _GlobalInitTracker
) -> List[str]:
    match node:
        case CodeLeaf():
            return [node.generate_code()]
        case CodeTree():
            return [
                l
                for child in node.get_tree_children()
                for l in _generate_lines_from_node(child, global_init)
            ]
        case GlobalInitLeaf():
            global_init.maybe_add_global(node)
            return []


def generate_code_from_tree(root: TreeNode) -> str:
    global_init = _GlobalInitTracker()
    lines = _generate_lines_from_node(root, global_init)
    global_lines = [
        leaf.generate_global_init_code() for leaf in global_init.ordered_globals
    ]
    return "\n".join(global_lines + lines)
