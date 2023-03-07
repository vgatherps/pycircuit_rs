from dataclasses import dataclass
from typing import Set

from pycircuit.circuit_builder.circuit import CircuitData
from pycircuit.circuit_builder.component import ComponentOutput


@dataclass
class CircuitMetadata:
    circuit: CircuitData
    non_ephemeral_outputs: Set[ComponentOutput]
