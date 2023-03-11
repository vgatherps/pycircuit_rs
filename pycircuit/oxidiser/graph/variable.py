from dataclasses import dataclass
from typing import List
from pycircuit.circuit_builder.component import ComponentOutput
from pycircuit.oxidiser.codegen import separated_names

from pycircuit.oxidiser.codegen.tree.tree_node import (
    CodeLeaf,
    TreeNode,
    CodeTree,
    GlobalInitLeaf,
)

_RAW_VAR_HEADER = "__raw_var_"

# HACK MOVE THIS VAR
_OUTPUT_NAME = "outputs"


def _generate_valid_init(valid_name: str, mut: str, ctor: bool) -> str:
    return f"let {mut} {valid_name}: bool = {ctor};"


def _valid_name(var_name: str) -> str:
    return f"{_RAW_VAR_HEADER}{var_name}_valid"


@dataclass(frozen=True, eq=True)
class OutputVar:
    output: ComponentOutput

    def variable_name(self) -> str:
        return f"{self.output.parent}_{self.output.output_name}"


@dataclass(eq=True, frozen=True)
class PerCallVar(GlobalInitLeaf, OutputVar):
    variable_type: str
    variable_constructor: str

    def generate_global_init_code(self) -> str:
        return f"let mut {self.var_path()}: {self.variable_type} = {self.variable_constructor};"

    def var_path(self) -> str:
        return f"{_RAW_VAR_HEADER}{self.variable_name()}"


@dataclass(eq=True, frozen=True)
class StoredVar(CodeLeaf, OutputVar):
    def generate_code(self) -> str:
        return ""

    def var_path(self) -> str:
        return f"self.{_OUTPUT_NAME}.{self.variable_name()}"


@dataclass(eq=True, frozen=True)
class PerCallValid(GlobalInitLeaf, OutputVar):
    valid_by_default: bool

    def generate_global_init_code(self) -> str:
        valid_name = self.valid_path()
        return _generate_valid_init(valid_name, "mut", self.valid_by_default)

    def valid_path(self) -> str:
        return _valid_name(self.variable_name())


@dataclass(eq=True, frozen=True)
class StoredValid(CodeLeaf, OutputVar):
    def valid_path(self) -> str:
        return f"self.{_OUTPUT_NAME}.{self.variable_name()}"

    def generate_code(self) -> str:
        raise NotImplementedError()


@dataclass(eq=True, frozen=True)
class AlwaysValid(GlobalInitLeaf, OutputVar):
    def generate_global_init_code(self) -> str:
        valid_name = self.valid_path()
        return _generate_valid_init(valid_name, "", True)

    def valid_path(self) -> str:
        return _valid_name(self.variable_name())


GraphVar = PerCallVar | StoredVar
GraphValid = PerCallValid | StoredValid | AlwaysValid


@dataclass(eq=True, frozen=True)
class GraphVariable(CodeTree):
    var: GraphVar
    valid: GraphValid

    def get_tree_children(self) -> List[TreeNode]:
        return [self.var, self.valid]
