from dataclasses import dataclass
from typing import List, Optional

from pycircuit.oxidiser.codegen.tree.tree_node import (
    CodeLeaf,
    CodeTree,
    GlobalInitLeaf,
    TreeNode,
)
from pycircuit.oxidiser.codegen.tree.line_literal import LineLiteral


@dataclass(frozen=True, eq=True)
class Scoped(CodeTree):
    inner: TreeNode
    prefix: Optional[str] = None
    suffix: Optional[str] = None

    def get_tree_children(self) -> List["TreeNode"]:
        prefix_str = self.prefix or ""
        suffix_str = self.suffix or ""
        open = LineLiteral(prefix_str + " {")
        close = LineLiteral("}" + suffix_str)
        match self.inner:
            case CodeTree():
                return [open] + self.inner.get_tree_children() + [close]
            case CodeLeaf():
                return [open, self.inner, close]
            case GlobalInitLeaf():
                return []
