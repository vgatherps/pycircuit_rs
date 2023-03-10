from pycircuit.circuit_builder.circuit import Component
from pycircuit.oxidiser.codegen.call_il.variable import GraphVariable

TYPES_MODULE = "_types"
INPUTS_MODULE = "_inputs"
OUTPUTS_MODULE = "_outputs"
COMPONENT_TYPE = "_ComponentType"


def get_alias_for(component: Component) -> str:
    return f"{TYPES_MODULE}::{component.name}::{COMPONENT_TYPE}"


def get_type_path_for_input(component: Component, input_name: str):
    assert input_name in component.inputs
    assert input_name in component.definition.inputs
    return f"{TYPES_MODULE}::{component.name}::{INPUTS_MODULE}::{input_name}"


def generate_output_type_alias_path(component_name: str, output: str) -> str:
    return f"{TYPES_MODULE}::{component_name}::{OUTPUTS_MODULE}::{output}"
