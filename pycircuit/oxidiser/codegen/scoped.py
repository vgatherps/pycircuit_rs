from dataclasses import dataclass
from typing import List

from pycircuit.oxidiser.codegen.generator import (
    CodeLeaf,
    CodeTree,
    GlobalInitLeaf,
    TreeNode,
)
from pycircuit.oxidiser.codegen.line_literal import LineLiteral


@dataclass(frozen=True, eq=True)
class Scoped(CodeTree):
    inner: TreeNode

    def get_tree_children(self) -> List["TreeNode"]:
        open = LineLiteral("{")
        close = LineLiteral("}")
        match self.inner:
            case CodeTree():
                return [open] + self.inner.get_tree_children() + [close]
            case CodeLeaf():
                return [open, self.inner, close]
            case GlobalInitLeaf():
                return []
