from typing import Generic, Sequence, Tuple, TypeVar
from frozendict.core import frozendict

K = TypeVar("K")
V = TypeVar("V")


# mypy friendly frozendict interface


class FrozenDict(frozendict, Generic[K, V]):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def __contains__(self, __o: object) -> bool:
        return super().__contains__(__o)

    def __getitem__(self, __key: K) -> V:
        return super().__getitem__(__key)

    def __setitem__(self, __key: K, __value: V) -> None:  # type: ignore
        raise TypeError("frozen is immutable")

    def items(self) -> Sequence[Tuple[K, V]]:  # type: ignore
        return super().items()  # type: ignore

    def keys(self) -> Sequence[K]:  # type: ignore
        return super().keys()  # type: ignore

    def values(self) -> Sequence[V]:  # type: ignore
        return super().values()  # type: ignore
