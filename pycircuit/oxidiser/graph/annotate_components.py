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
from pycircuit.oxidiser.codegen.struct_il.typedefs.type_names import get_type_for_output
from pycircuit.oxidiser.graph.variable import (
    AlwaysValid,
    GraphValid,
    GraphVar,
    GraphVariable,
    PerCallValid,
    PerCallVar,
    StoredValid,
    StoredVar,
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


def is_invalid_by_default(output: ComponentOutput, circuit: CircuitData):
    match output:
        case ExternalOutput():
            return False
        case GraphOutput(parent, output_name):
            parent_component = circuit.components[parent]
            return parent_component.definition.output_specs[output_name].assume_invalid


def output_to_var(
    circuit_meta: CircuitMetadata, output: ComponentOutput
) -> GraphVariable:
    circuit = circuit_meta.circuit

    match output:
        case ExternalOutput():
            raise NotImplementedError("External variables WIP")
        case GraphOutput(parent, output_name):
            the_output = output.output()
            is_ephemeral = the_output not in circuit_meta.non_ephemeral_outputs

            val: Optional[GraphValid] = None
            var: GraphVar

            always_valid = is_always_valid(the_output, circuit)
            invalid_by_default = is_invalid_by_default(the_output, circuit)

            if always_valid:
                val = AlwaysValid(output=the_output)

            if invalid_by_default:
                val = val or PerCallValid(output=the_output, valid_by_default=False)

            if is_ephemeral:
                val = val or PerCallValid(output=the_output, valid_by_default=False)
                var_type = get_type_for_output(circuit, the_output)
                output_specs = circuit.components[parent].definition.output_specs.get(
                    output_name, None
                )
                if (
                    output_specs is not None
                    and output_specs.default_constructor is not None
                ):
                    var_constructor = output_specs.default_constructor
                else:
                    var_constructor = "Default::default()"
                var = PerCallVar(
                    output=the_output,
                    variable_type=var_type,
                    variable_constructor=var_constructor,
                )
            else:
                val = val or StoredValid(
                    output=the_output,
                )
                var = StoredVar(output=the_output)
            return GraphVariable(var=var, valid=val)


def outputs_to_var(
    circuit_meta: CircuitMetadata, outputs: List[ComponentOutput]
) -> Dict[ComponentOutput, GraphVariable]:
    return {output: output_to_var(circuit_meta, output) for output in outputs}
