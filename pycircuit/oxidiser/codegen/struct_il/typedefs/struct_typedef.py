from pycircuit.circuit_builder.circuit import CircuitData
from pycircuit.circuit_builder.component import Component
from pycircuit.oxidiser.codegen.struct_il.typedefs.type_names import get_type_for_input

TypeSuffix = "Type"


def get_struct_type(circuit: CircuitData, component: Component):
    root_path = f"{component.definition.module}::{component.name}"
    gen_order = component.definition.generics_order

    if gen_order:

        sorted_generic_inputs = sorted(gen_order.keys(), key=lambda a: gen_order[a])
        associated_outputs = [
            component.inputs[input] for input in sorted_generic_inputs
        ]

        types = [
            get_type_for_input(circuit, input.outputs()[0])
            for input in associated_outputs
        ]

        generics = ", ".join(types)

        root_path = f"{root_path}::<{generics}>"

    return root_path


def get_component_type_name(component: Component):
    return f"{component.name}{TypeSuffix}"


def generate_component_typedefs(circuit: CircuitData) -> str:
    typedefs = []

    for component in circuit.components.values():
        typedefs.append(
            f"pub type {get_component_type_name(component)} = {get_struct_type(circuit, component)};"
        )

    return "\n".join(typedefs)
