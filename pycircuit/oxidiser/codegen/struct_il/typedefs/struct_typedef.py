from dataclasses import dataclass
from pycircuit.circuit_builder.circuit import CircuitData
from pycircuit.circuit_builder.component import Component
from pycircuit.oxidiser.codegen.struct_il.typedefs.type_names import (
    get_component_type_name,
    get_component_valid_name,
    get_type_for_output,
)
from pycircuit.oxidiser.codegen.tree.tree_node import CodeLeaf


def get_struct_type(circuit: CircuitData, component: Component):
    root_path = f"{component.definition.module}::{component.name}"
    gen_order = component.definition.generics_order

    if gen_order:

        sorted_generic_inputs = sorted(gen_order.keys(), key=lambda a: gen_order[a])
        associated_outputs = [
            component.inputs[input] for input in sorted_generic_inputs
        ]

        types = [
            get_type_for_output(circuit, input.outputs()[0])
            for input in associated_outputs
        ]

        generics = ", ".join(types)

        root_path = f"{root_path}::<{generics}>"

    return root_path


def generate_component_typedefs(circuit: CircuitData) -> str:
    typedefs = []

    for component in circuit.components.values():
        typedefs.append(
            f"pub type {get_component_type_name(component)} = {get_struct_type(circuit, component)};"
        )

    return "\n".join(typedefs)


@dataclass
class ComponentTypedefLeaf(CodeLeaf):
    circuit: CircuitData
    component: Component

    def generate(self) -> str:
        type_alias_name = get_component_type_name(self.component)
        type_name = get_struct_type(self.circuit, self.component)
        return f"pub type {type_alias_name} = {type_name};"


@dataclass
class ComponentFieldLeaf(CodeLeaf):
    component: Component

    def generate(self) -> str:
        type_alias_name = get_component_type_name(self.component)
        return f"pub {self.component.name}: {type_alias_name},"


@dataclass
class ComponentValidLeaf(CodeLeaf):
    component: Component

    def generate(self) -> str:
        valid_var_name = get_component_valid_name(self.component)
        return f"pub {valid_var_name}: bool,"
