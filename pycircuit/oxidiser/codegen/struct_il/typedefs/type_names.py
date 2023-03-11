from pycircuit.circuit_builder.circuit import CircuitData, Component
from pycircuit.circuit_builder.component import (
    ComponentIndex,
    ComponentInput,
    ComponentOutput,
    ExternalInput,
    ExternalOutput,
    GraphOutput,
    SingleComponentInput,
)
from pycircuit.oxidiser.codegen.call_il.variable import GraphVariable

from camel_converter import to_pascal


TYPES_MODULE = "_types"
INPUTS_MODULE = "_inputs"
COMPONENT_TYPE = "_ComponentType"
EXPORT_TAIL = "OutputExport"


TypeSuffix = "Type"
ValidSuffix = "Valid"


def get_alias_for(component: Component) -> str:
    return f"{TYPES_MODULE}::{component.name}::{COMPONENT_TYPE}"


def output_name_to_export_trait(name: str):
    return to_pascal(name)


def get_output_alias(component: Component, output: str):
    component_path = get_alias_for(component)
    output_type = output_name_to_export_trait(output)
    module_path = component.definition.module
    export_trait = f"{module_path}::{component.name}{EXPORT_TAIL}"
    return f"<{component_path} as {export_trait}>::{output_type}"


def get_type_for_input(circuit: CircuitData, output: ComponentOutput) -> str:
    match output:
        case ExternalOutput(type):
            return circuit.external_inputs[type].type
        case GraphOutput(parent, output_name):
            return get_output_alias(circuit.components[parent], output_name)


def get_component_type_name(component: Component):
    return f"{component.name}{TypeSuffix}"


def get_component_valid_name(component: Component):
    return f"{component.name}{ValidSuffix}"
