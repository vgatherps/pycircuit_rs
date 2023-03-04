# automatically generated by the FlatBuffers compiler, do not modify

# namespace:

import flatbuffers
from flatbuffers.compat import import_numpy

np = import_numpy()


class MdMessage(object):
    __slots__ = ["_tab"]

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = MdMessage()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsMdMessage(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)

    # MdMessage
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # MdMessage
    def LocalTimeUs(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MdMessage
    def MessageType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint8Flags, o + self._tab.Pos)
        return 0

    # MdMessage
    def Message(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            from flatbuffers.table import Table

            obj = Table(bytearray(), 0)
            self._tab.Union(obj, o)
            return obj
        return None


def MdMessageStart(builder):
    builder.StartObject(3)


def Start(builder):
    return MdMessageStart(builder)


def MdMessageAddLocalTimeUs(builder, localTimeUs):
    builder.PrependInt64Slot(0, localTimeUs, 0)


def AddLocalTimeUs(builder, localTimeUs):
    return MdMessageAddLocalTimeUs(builder, localTimeUs)


def MdMessageAddMessageType(builder, messageType):
    builder.PrependUint8Slot(1, messageType, 0)


def AddMessageType(builder, messageType):
    return MdMessageAddMessageType(builder, messageType)


def MdMessageAddMessage(builder, message):
    builder.PrependUOffsetTRelativeSlot(
        2, flatbuffers.number_types.UOffsetTFlags.py_type(message), 0
    )


def AddMessage(builder, message):
    return MdMessageAddMessage(builder, message)


def MdMessageEnd(builder):
    return builder.EndObject()


def End(builder):
    return MdMessageEnd(builder)
