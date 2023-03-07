from dataclasses import dataclass
from typing import List, Optional
from pycircuit.oxidiser.codegen.call_il.dispatch import CallDispatch

from pycircuit.oxidiser.codegen.call_il.input import CallInputSet
from pycircuit.oxidiser.codegen.call_il.output import CallOutputSet
from pycircuit.oxidiser.codegen.tree.tree_node import CodeTree, TreeNode


@dataclass(frozen=True, eq=True)
class FullCall(CodeTree):

    dispatch: CallDispatch

    inputs: Optional[CallInputSet]
    outputs: Optional[CallOutputSet]

    def get_tree_children(self) -> List[TreeNode]:

        children: List[TreeNode] = []
        validity = None

        if self.inputs is not None:
            children.append(self.inputs)

        if self.outputs is not None:
            children.append(self.outputs)
            validity = self.outputs.valid_set()

        children.append(self.dispatch)

        if validity is not None:
            children.append(validity)

        return children
