from typing import List, Set
from pycircuit.circuit_builder.component import Component, ComponentOutput, GraphOutput
from pycircuit.oxidiser.graph.find_children_of import CalledComponent


def is_ephemeral(
    component: Component, output: str, non_ephemeral_outputs: Set[ComponentOutput]
):
    potential_options = component.output_options.get(output)
    output_data = component.definition.d_output_specs[output]
    component_output = GraphOutput(parent=component.name, output_name=output)

    if potential_options is not None:
        must_store = potential_options.force_stored
    else:
        must_store = False

    needs_write = output_data.assume_invalid or output_data.assume_default

    return (
        (component_output not in non_ephemeral_outputs or needs_write)
        and not must_store
        and output_data.ephemeral
    )


def find_nonephemeral_outputs(
    called_components: List[CalledComponent],
) -> Set[ComponentOutput]:
    own_component_names = {
        called_component.component.name for called_component in called_components
    }

    # Filter out always-invalid outputs. They're ephemeral

    # TODO this is a hack, this filtering here is depended upon in a spaghetti-like fashion
    # throughout the codebase to ensure that invalidity always works.

    non_ephemeral_outputs = set()
    for called_component in called_components:

        component = called_component.component

        for input in component.inputs.values():
            for output in input.outputs():
                if output.parent not in own_component_names:
                    non_ephemeral_outputs.add(output)

    return non_ephemeral_outputs
