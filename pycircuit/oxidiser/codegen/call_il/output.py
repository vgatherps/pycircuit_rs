from dataclasses import dataclass
from typing import List
from pycircuit.oxidiser.codegen.tree.tree_node import CodeLeaf, CodeTree, TreeNode

from pycircuit.oxidiser.codegen.call_il.graph_variable import (
    AlwaysValid,
    GraphValid,
    GraphVariable,
)

_RAW_OUTPUT_HEADER = "__raw_output_"

OUTPUT_STRUCT_NAME = "__outputs"

OUTPUT_VALID_RETURN_NAME = "__output_valid"


@dataclass(frozen=True, eq=True)
class OutputRefLeaf(CodeLeaf):
    output: "OutputRef"

    def generate_code(self) -> str:
        return f"let {self.output.local_path()} = &mut {self.output.variable.var.var_path()};"


@dataclass(frozen=True, eq=True)
class OutputRef(CodeTree):
    variable: GraphVariable
    output_name: str

    def get_tree_children(self) -> List[TreeNode]:
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

    def get_tree_children(self) -> List[TreeNode]:
        local_input_children = [
            o for output in self.outputs for o in output.get_tree_children()
        ]
        return local_input_children + [OutputStructLeaf(self)]

    def valid_set(self) -> "OutputValidSet":
        return OutputValidSet(
            outputs=[
                OutputValid(
                    valid=output.variable.valid,
                    output_name=output.output_name,
                )
                for output in self.outputs
            ]
        )


# This does not partake in the code tree since it's just a helper
# to condense information already in other trees
@dataclass(frozen=True, eq=True)
class OutputValid:
    valid: GraphValid
    output_name: str


@dataclass(frozen=True, eq=True)
class OutputValidSet(CodeLeaf):
    outputs: List[OutputValid]

    def generate_code(self) -> str:
        valid_lines = []
        for output in self.outputs:
            match output.valid:
                case AlwaysValid():
                    continue
                case valid_var:
                    path = valid_var.valid_path()
                    valid_lines.append(
                        f"{path} = {OUTPUT_VALID_RETURN_NAME}.{output.output_name}"
                    )

        return "\n".join(valid_lines)
