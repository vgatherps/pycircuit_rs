from typing import Dict, List, Set
from pycircuit.circuit_builder.circuit import CircuitData
from pycircuit.circuit_builder.component import ComponentOutput
from pycircuit.oxidiser.graph.ephemeral import find_nonephemeral_outputs
from pycircuit.oxidiser.graph.find_children_of import (
    CalledComponent,
    find_all_children_of,
    find_all_children_of_from_outputs,
)


def find_timer_subgraphs(circuit: CircuitData) -> Dict[str, List[CalledComponent]]:
    timer_calls = {}
    for component in circuit.components.values():
        if component.definition.timer_callset is not None:
            timer_outputs = {
                component.output(out)
                for out in component.definition.timer_callset.outputs
            }
            timer_children = find_all_children_of_from_outputs(circuit, timer_outputs)
            called_component = CalledComponent(
                callsets=[component.definition.timer_callset], component=component
            )
            timer_calls[component.name] = [called_component] + timer_children
    return timer_calls


def find_all_subgraphs(circuit: CircuitData) -> List[List[CalledComponent]]:
    called = []

    for call_group in circuit.call_groups.values():
        children = find_all_children_of(call_group.inputs, circuit)
        called.append(children)

    called += list(find_timer_subgraphs(circuit).values())

    return called


def all_nonephemeral_outputs(circuit: CircuitData) -> Set[ComponentOutput]:
    subgraphs = find_all_subgraphs(circuit)

    all_non_ephemeral_component_outputs: Set[ComponentOutput] = set()

    for subgraph in subgraphs:
        all_non_ephemeral_component_outputs |= find_nonephemeral_outputs(subgraph)

    return all_non_ephemeral_component_outputs
