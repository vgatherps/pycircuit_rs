from dataclasses import dataclass

from pycircuit.oxidiser.codegen.tree.tree_node import CodeLeaf


@dataclass(frozen=True, eq=True)
class LineLiteral(CodeLeaf):
    line: str

    def generate_code(self) -> str:
        return self.line
