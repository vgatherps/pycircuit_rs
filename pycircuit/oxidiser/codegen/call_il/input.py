from dataclasses import dataclass
from typing import List
from pycircuit.oxidiser.codegen.tree.tree_node import CodeLeaf, CodeTree, TreeNode

from pycircuit.oxidiser.codegen.call_il.variable import GraphVariable

_RAW_INPUT_HEADER = "__raw_input_"

INPUT_STRUCT_NAME: str = "__inputs"


@dataclass(frozen=True, eq=True)
class SingleInputLeaf(CodeLeaf):
    input: "SingleInput"

    # TODO support always valid inputs
    def generate_code(self) -> str:
        return f"""\
let {self.input.local_path()}: Option<&_> = if {self.input.variable.valid.valid_path()} {{
    Some(& {self.input.variable.var.var_path()})
}} else {{
    None
}};\
"""


@dataclass(frozen=True, eq=True)
class SingleInput(CodeTree):
    variable: GraphVariable
    input_name: str

    def get_tree_children(self) -> List[TreeNode]:
        return [
            self.variable,
            SingleInputLeaf(self),
        ]

    def local_path(self) -> str:
        return _RAW_INPUT_HEADER + self.input_name


@dataclass(frozen=True, eq=True)
class InputStructLeaf(CodeLeaf):
    input_set: "CallInputSet"

    def generate_code(self) -> str:
        struct_lines = [
            f"{input.input_name}: {input.local_path()}"
            for input in self.input_set.inputs
        ]
        struct_innards = "\n".join(struct_lines)
        return f"""\
let {INPUT_STRUCT_NAME} = {self.input_set.input_struct_name} {{
{struct_innards}
}};\
"""


@dataclass(frozen=True, eq=True)
class CallInputSet(CodeTree):
    inputs: List[SingleInput]
    input_struct_name: str

    def get_tree_children(self) -> List[TreeNode]:
        local_input_children = [
            i for input in self.inputs for i in input.get_tree_children()
        ]
        return local_input_children + [InputStructLeaf(self)]
