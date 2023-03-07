from typing import Hashable, List
from abc import ABC, abstractmethod


class CodeLeaf:
    def generate_code(self) -> str:
        return ""


class GlobalInitLeaf(CodeLeaf, Hashable):
    @abstractmethod
    def generate_global_init_code(self) -> str:
        pass


class CodeTree:
    def get_tree_children(self) -> List["CodeNode"]:
        return []


CodeNode = CodeTree | CodeLeaf
