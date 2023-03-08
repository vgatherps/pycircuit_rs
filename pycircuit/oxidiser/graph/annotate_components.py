from dataclasses import dataclass
from typing import Dict, List, Optional
from pycircuit.circuit_builder.circuit import CircuitData

from pycircuit.circuit_builder.component import (
    ArrayComponentInput,
    Component,
    ComponentOutput,
    ExternalOutput,
    GraphOutput,
)
from pycircuit.common.frozen import FrozenDict
from pycircuit.oxidiser.codegen.call_il.variable import (
    AlwaysValid,
    GraphValid,
    GraphVariable,
    PerCallValid,
    PerCallVar,
    StoredValid,
)
from pycircuit.oxidiser.graph.graph_metadata import CircuitMetadata


@dataclass
class AnnotatedComponent:
    component: Component

    input_variables: FrozenDict[str, GraphVariable]
    output_variables: FrozenDict[str, GraphVariable]


def is_always_valid(output: ComponentOutput, circuit: CircuitData):
    match output:
        case ExternalOutput():
            return False
        case GraphOutput(parent, output_name):
            parent_component = circuit.components[parent]
            return parent_component.definition.output_specs[output_name].always_valid


def invalid_by_default(output: ComponentOutput, circuit: CircuitData):
    match output:
        case ExternalOutput():
            return False
        case GraphOutput(parent, output_name):
            parent_component = circuit.components[parent]
            return parent_component.definition.output_specs[output_name].assume_invalid


def annotate_components(circuit_meta: CircuitMetadata, components: List[Component]):
    """Annotate components with their input and output variables."""
    circuit = circuit_meta.circuit

    for component in components:

        input_variables = {}

        for (input_name, input) in component.inputs.items():
            if isinstance(input, ArrayComponentInput):
                raise NotImplementedError("Rust version does not support array inputs")
            the_output = input.output()
            is_ephemeral = input.output() not in circuit_meta.non_ephemeral_outputs

            val: Optional[GraphValid] = None
            always_valid = is_always_valid(input.output(), circuit)
            invalid_by_default = invalid_by_default(the_output, circuit)

            if always_valid:
                val = AlwaysValid(output=the_output)

            if is_ephemeral or invalid_by_default:
                val = val or PerCallValid(output=the_output, valid_by_default=False)
            else:
                val = val or StoredValid(
                    output=the_output,
                )

            raise NotImplementedError(
                "Have not implemented type naming so can't create variables"
            )
