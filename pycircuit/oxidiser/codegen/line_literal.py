from dataclasses import dataclass

from pycircuit.oxidiser.codegen.generator import CodeLeaf


@dataclass(frozen=True, eq=True)
class LineLiteral(CodeLeaf):
    line: str

    def generate_code(self) -> str:
        return self.line
