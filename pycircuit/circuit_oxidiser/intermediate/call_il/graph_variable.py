from dataclasses import dataclass

_RAW_VAR_HEADER = "__raw_var_noname_"


@dataclass(eq=True, frozen=True)
class PerCallValid:
    variable_name: str
    variable_type: str
    variable_constructor: str
    valid_by_default: bool

    def generate_init_code(self) -> str:
        valid_name = self.ref_to_valid()
        return f"let mut {valid_name}: bool = {self.valid_by_default};"

    def ref_to_valid(self) -> str:
        return f"{_RAW_VAR_HEADER}{self.variable_name}_valid"


@dataclass(eq=True, frozen=True)
class PerCallVar:
    variable_name: str
    variable_type: str
    variable_constructor: str

    def generate_init_code(self) -> str:
        var_name = self.ref_to_var()
        raw_var_name = f"{_RAW_VAR_HEADER}{var_name}"
        return f"""\
let mut {raw_var_name}: {self.variable_type} = {self.variable_constructor};\
let {var_name} = &mut {raw_var_name};\
"""

    def ref_to_var(self) -> str:
        return f"{self.variable_name}"


@dataclass(eq=True, frozen=True)
class StoredVar:
    variable_name: str
    outputs_name: str

    def ref_to_var(self) -> str:
        return f"&mut {self.outputs_name}.{self.variable_name}"


@dataclass(eq=True, frozen=True)
class StoredValid:
    variable_name: str
    outputs_name: str

    def ref_to_valid(self) -> str:
        return f"&mut {self.outputs_name}.{self.variable_name}"


@dataclass(eq=True, frozen=True)
class EphemeralVar:
    var: PerCallVar
    valid: PerCallValid

    def generate_init_code(self) -> str:
        return self.var.generate_init_code() + self.valid.generate_init_code()


@dataclass(eq=True, frozen=True)
class StoredValidVar:
    var: StoredVar
    valid: PerCallValid

    def generate_init_code(self) -> str:
        return self.valid.generate_init_code()


@dataclass(eq=True, frozen=True)
class FullyStoredVar:
    var: StoredVar
    valid: StoredValid

    def generate_init_code(self) -> str:
        return ""


GraphVariable = EphemeralVar | StoredValidVar | FullyStoredVar
