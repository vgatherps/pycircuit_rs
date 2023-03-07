from frozenlist import FrozenList
from pycircuit.common.frozen import FrozenDict
from pycircuit.circuit_builder.component import (
    Component,
    ComponentInput,
    ExternalOutput,
    GraphOutput,
)
from pycircuit.circuit_builder.definition import (
    CallSpec,
    Definition,
    BasicInput,
    OutputSpec,
    InitSpec,
)
from pycircuit.circuit_builder.component import SingleComponentInput
from pycircuit.circuit_builder.component import ComponentOutput
from pycircuit.circuit_builder.definition import CallsetGroup

OUT_B_VALID_INDEX = 2
COMPONENT_NAME = "test"
COMPONENT_CLASS = "TestComponent"
OUT_A = "out_a"
OUT_A_CLASS = "OutA"
OUT_B = "out_b"
OUT_B_CLASS = "OutB"
OUT_C = "out_c"
OUT_C_CLASS = "OutC"

A_INPUT = SingleComponentInput(
    input=ExternalOutput(external_name="val_a"), input_name="a"
)
B_INPUT = SingleComponentInput(
    GraphOutput(parent="fake", output_name="fake_out"), input_name="b"
)
C_INPUT = SingleComponentInput(
    GraphOutput(parent="fake", output_name="fake_out_c"), input_name="c"
)
D_INPUT = SingleComponentInput(
    GraphOutput(parent="fake", output_name="fake_out_d"), input_name="d"
)
E_INPUT = SingleComponentInput(
    GraphOutput(parent="fake", output_name="fake_out_e"), input_name="e"
)

AB_CALLSET = CallSpec(
    written_set=frozenset({"a", "b"}),
    observes=frozenset(),
    callback="call_out_a",
    outputs=frozenset({OUT_A}),
    name="AB",
)

BC_CALLSET = CallSpec(
    written_set=frozenset({"b", "c"}),
    observes=frozenset(),
    callback="call_out_b",
    outputs=frozenset({OUT_B}),
    name="BC",
)

CD_CALLSET = CallSpec(
    written_set=frozenset({"c", "d"}),
    observes=frozenset(),
    callback="call_e1",
    outputs=frozenset({OUT_B}),
    name="CD",
)

CDE_CALLSET = CallSpec(
    written_set=frozenset({"c", "d", "e"}),
    observes=frozenset({"a", "b"}),
    callback="call_e2",
    outputs=frozenset({OUT_B}),
)


GENERIC_CALLSET = CallSpec(
    written_set=frozenset({"a", "b", "c"}),
    observes=frozenset(),
    callback="call",
    outputs=frozenset({OUT_A, OUT_B}),
)


def freeze(l):
    l = FrozenList(l)
    l.freeze()
    return l


def basic_definition(generic_callset=GENERIC_CALLSET) -> Definition:
    defin = Definition(
        inputs=FrozenDict(
            {
                "a": BasicInput(),
                "b": BasicInput(),
                "c": BasicInput(),
                "d": BasicInput(),
                "e": BasicInput(),
            }
        ),
        output_specs=FrozenDict(
            {
                OUT_A: OutputSpec(ephemeral=True, type_path=OUT_A_CLASS),
                OUT_B: OutputSpec(ephemeral=False, type_path=OUT_B_CLASS),
                OUT_C: OutputSpec(
                    ephemeral=True, type_path=OUT_C_CLASS, always_valid=True
                ),
            }
        ),
        class_name=COMPONENT_CLASS,
        init_spec=InitSpec(init_call="dummy"),
        header="test.hh",
        generic_callset=generic_callset,
        callsets=frozenset(
            {
                AB_CALLSET,
                BC_CALLSET,
                CD_CALLSET,
                CDE_CALLSET,
            }
        ),
        callset_groups=frozenset({CallsetGroup(callsets=freeze(["BC", "AB"]))}),
    )
    defin.validate()
    return defin


def basic_component() -> Component:
    comp = Component(
        inputs=FrozenDict(
            {"a": A_INPUT, "b": B_INPUT, "c": C_INPUT, "d": D_INPUT, "e": E_INPUT}
        ),
        output_options={},
        definition=basic_definition(),
        name="test",
        class_generics={},
        params=FrozenDict(),
    )
    return comp
