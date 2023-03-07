from pycircuit.circuit_builder.definition import CallSpec, Definition, OutputSpec
from pycircuit.circuit_builder.definition import BasicInput
from pycircuit.common.frozen import FrozenDict


def generate_static_index_definition(offset: int) -> Definition:
    return Definition(
        class_name=f"StaticIndex",
        output_specs=FrozenDict(out=OutputSpec(ephemeral=True, type_path="Output")),
        inputs=FrozenDict({"a": BasicInput()}),
        header="signals/basic_operators.hh",
        generic_callset=CallSpec(
            observes=frozenset(),
            written_set=frozenset(["a"]),
            callback="call",
            outputs=frozenset(["out"]),
        ),
        # TODO once we support arrays can definitely have
        # a differentiable operator
        metadata=FrozenDict({"include_param_names": False}),
        generics_order=FrozenDict({"a": 0}),
        class_generics=FrozenDict({"N": 0}),
    ).validate()
