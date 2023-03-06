from typing import Protocol


class CodeGenerator(Protocol):
    def generate_code(self) -> str:
        ...
