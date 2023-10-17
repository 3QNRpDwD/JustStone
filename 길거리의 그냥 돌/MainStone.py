from StructureStone import *
from NetStone import StoneTransferProtocol



def Run():

    STP = StoneTransferProtocol( '127.0.0.1', 6974, 0 )

    req = STP.ReceiveStone()

    print( f"받은거 : { req.header } \n받은거 : { req.payload }" )

    SSP = ConstructStonePayload.from_( StructRawStonePayload( "sysinfo..", "command_input..", "command_output..", "stone_chain.." ) )
    SSH = ConstructStoneHeader.from_( SSP )

    STP.SendStone( ConstructStone.from_( SSH, SSP ).stone )

Run()