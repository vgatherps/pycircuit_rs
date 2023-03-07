from dataclasses import dataclass
from typing import Optional

from pycircuit.oxidiser.codegen.generator import CodeLeaf
from pycircuit.oxidiser.codegen.call_il.input import INPUT_STRUCT_NAME
from pycircuit.oxidiser.codegen.call_il.output import (
    OUTPUT_STRUCT_NAME,
    OUTPUT_VALID_RETURN_NAME,
)


@dataclass(frozen=True, eq=True)
class CallDispatch(CodeLeaf):
    path: str
    call_name: str

    takes_inputs: bool
    takes_outputs: bool
    takes_metadata: bool

    valid_struct_name: Optional[str]

    def generate_code(self) -> str:
        call_args = []

        if self.takes_inputs:
            call_args.append(INPUT_STRUCT_NAME)

        if self.takes_outputs:
            call_args.append(OUTPUT_STRUCT_NAME)

        if self.takes_metadata:
            raise NotImplementedError("Metadata not yet implemented")

        return_ty = self.valid_struct_name or "()"

        call_args_str = ", ".join(call_args)

        return f"""\
let {OUTPUT_VALID_RETURN_NAME}: {return_ty} = {self.path}.{self.call_name}({call_args_str});\
"""
