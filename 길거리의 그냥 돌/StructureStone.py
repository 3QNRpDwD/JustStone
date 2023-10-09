from dataclasses import dataclass, field
from datetime import datetime, timedelta
import struct


@dataclass
class StoneChain: 

    previous_stone_hash: bytes
    stone_hash: bytes
    stonetree_hash: bytes
    timestamp: bytes
    transaction_list: bytes

@dataclass
class StructRawStonePayload:

    sysinfo: str
    command_input: str
    command_output: str
    stone_chain: str

@dataclass
class StructStonePayload:

    sysinfo: bytes
    command_input: bytes
    command_output: bytes
    stone_chain: bytes

@dataclass
class StructStoneHeader:

    StoneStatus : bytes
    StoneType   : bytes
    StoneSize   : bytes

@dataclass
class StructStone:

    header: StructStoneHeader
    payload: StructStonePayload
    stone: bytes = field(init=False, default=None)

    def __post_init__(self):

        self.stone = self.header + self.payload


class ConstructStonePayload:

    def from_(SRSP: StructRawStonePayload ) -> StructStonePayload:

        sysinfo     = SRSP.sysinfo        .encode()
        cmd_input   = SRSP.command_input  .encode()
        cmd_output  = SRSP.command_output .encode()
        stone_chain = SRSP.stone_chain    .encode()
        
        return StructStonePayload(sysinfo, cmd_input, cmd_output, stone_chain)

class ConstructStoneHeader:
    
    def from_(SSP: StructStonePayload ) -> StructStoneHeader:
        
        StoneSize = len(SSP.sysinfo) + len(SSP.command_input) + len(SSP.command_output) + len(SSP.stone_chain)
        StoneStatus = struct.pack("I", 0)
        
        if ( SSP.sysinfo and not SSP.command_input and not SSP.command_output) :
            StoneType = struct.pack("I", 1)
        elif SSP.command_output:
            StoneType = struct.pack("I", 2)
        else:
            StoneType = struct.pack("I", 3)
        
        return StructStoneHeader(StoneStatus, StoneType, struct.pack("Q", StoneSize))

    def __init__(self, packed_data):
        self.packed_data = packed_data

class ConstructStone:
    
    def from_(SSH: StructStoneHeader, SSP: StructStonePayload) -> StructStone:
        header = SSH.StoneStatus + SSH.StoneType + SSH.StoneSize
        payload = SSP.sysinfo + SSP.command_input + SSP.command_output + SSP.stone_chain
        
        return  StructStone(header, payload)

class protocolRules:
    def __init__(self) -> None:
        self.Rules = [
                        { 0 : "sysinfo", 1 : "command_input", 2 : "command_output", 3 : "stone_chain" },
                        { 0 : "sysinfo", 1 : "command_input", 2 : "command_output", 3 : "stone_chain" }
                    ]

    def from_json():
        pass

    def to_json():
        pass