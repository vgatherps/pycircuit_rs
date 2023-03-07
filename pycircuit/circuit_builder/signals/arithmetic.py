from pycircuit.circuit_builder.definition import CallSpec, Definition, OutputSpec
from pycircuit.circuit_builder.definition import BasicInput
from pycircuit.common.frozen import FrozenDict


def generate_binary_definition(diff_name: str, operator_name: str) -> Definition:
    return Definition(
        class_name=operator_name,
        output_specs=FrozenDict(out=OutputSpec(ephemeral=True, type_path="Output")),
        inputs=FrozenDict({"a": BasicInput(), "b": BasicInput()}),
        header="signals/basic_arithmetic.hh",
        generic_callset=CallSpec(
            observes=frozenset(),
            written_set=frozenset(["a", "b"]),
            callback="call",
            outputs=frozenset(["out"]),
            input_struct_path="Input",
        ),
        generics_order=FrozenDict(a=0, b=1),
        differentiable_operator_name=diff_name,
        metadata=FrozenDict({"include_param_names": False}),
    ).validate()
