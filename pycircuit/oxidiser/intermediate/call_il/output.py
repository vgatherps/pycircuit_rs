from dataclasses import dataclass
from typing import List
from pycircuit.oxidiser.codegen.generator import CodeLeaf, CodeTree, CodeNode

from pycircuit.oxidiser.intermediate.graph.graph_variable import GraphVariable

_RAW_OUTPUT_HEADER = "__raw_output_"

OUTPUT_STRUCT_NAME = "__outputs"


@dataclass(frozen=True, eq=True)
class OutputRefLeaf(CodeLeaf):
    output: "OutputRef"

    def generate_code(self) -> str:
        return f"let {self.output.local_path()} = &mut {self.output.variable.var.var_path()};"


@dataclass(frozen=True, eq=True)
class OutputRef(CodeTree):
    variable: GraphVariable
    output_name: str

    def get_tree_children(self) -> List[CodeNode]:
        return [
            self.variable,
            OutputRefLeaf(self),
        ]

    def local_path(self) -> str:
        return _RAW_OUTPUT_HEADER + self.output_name


@dataclass(frozen=True, eq=True)
class OutputStructLeaf(CodeLeaf):
    output_set: "CallOutputSet"

    def generate_code(self) -> str:
        struct_lines = [
            f"{output.output_name}: {output.local_path()}"
            for output in self.output_set.outputs
        ]
        struct_innards = "\n".join(struct_lines)
        return f"""\
let {OUTPUT_STRUCT_NAME} = {self.output_set.output_struct_name} {{
{struct_innards}
}};\
"""


@dataclass(frozen=True, eq=True)
class CallOutputSet(CodeTree):
    outputs: List[OutputRef]
    output_struct_name: str

    def get_tree_children(self) -> List[CodeNode]:
        local_input_children = [
            o for output in self.outputs for o in output.get_tree_children()
        ]
        return local_input_children + [OutputStructLeaf(self)]
