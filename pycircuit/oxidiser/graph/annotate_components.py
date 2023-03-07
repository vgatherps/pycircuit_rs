from dataclasses import dataclass
from typing import Dict, List
from pycircuit.circuit_builder.circuit import CircuitData

from pycircuit.circuit_builder.component import ArrayComponentInput, Component, ComponentOutput, ExternalOutput, GraphOutput
from pycircuit.common.frozen import FrozenDict
from pycircuit.oxidiser.codegen.call_il.variable import GraphVariable
from pycircuit.oxidiser.graph.graph_metadata import CircuitMetadata


@dataclass
class AnnotatedComponent:
    component: Component

    input_variables: FrozenDict[str, GraphVariable]
    output_variables: FrozenDict[str, GraphVariable]


def always_valid(output: ComponentOutput, circuit: CircuitData):
    match output:
        case ExternalOutput():
            return False
        case GraphOutput(parent, output_name):
            parent_component = circuit.components[parent]
            return parent_component.definition.output_specs[output_name].always_valid 

def annotate_components(circuit_meta: CircuitMetadata, components: List[Component]):
    """Annotate components with their input and output variables."""
    circuit = circuit_meta.circuit

    for component in components:

        input_variables = {}
        for (input_name, input) in component.inputs.items():
            if isinstance(input, ArrayComponentInput):
                raise NotImplementedError("Rust version does not support array inputs")
            is_ephemeral = input.output() not in circuit_meta.non_ephemeral_outputs

            always_valid = always_valid(input.output(), circuit)
            if is_ephemeral:
