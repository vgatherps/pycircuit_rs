from typing import Hashable, List
from abc import ABC, abstractmethod


class CodeLeaf:
    @abstractmethod
    def generate_code(self) -> str:
        ...


class GlobalInitLeaf(Hashable):
    @abstractmethod
    def generate_global_init_code(self) -> str:
        ...


class CodeTree:
    @abstractmethod
    def get_tree_children(self) -> List["TreeNode"]:
        ...


LeafNode = CodeLeaf | GlobalInitLeaf
TreeNode = CodeTree | LeafNode
