import socket
from StructureStone import *

class StoneTransferProtocol:
    def __init__( self , addr  : str,  port :int, listen : int ):
        self.s = socket.socket()
        self.soket = self.SetupConnection( addr, port, listen )
        self.client = self.soket[0]

    def SetupConnection( self, addr : str, port :int, listen : int ):

        self.s.bind( ( addr, port ) )
        self.s.listen( listen )

        return self.s.accept()

    def identifyPacketType( self, Packet: StructStoneHeader ) -> str:
        pass


    def ParsingPacket( self, Packet: [ StructStoneHeader, StructStonePayload ] ) -> StructStonePayload:
        Packet = Packet[1].decode().split("..")
        return StructStonePayload(*Packet)

    def SendStone( self, Stone ):

        try:
            print( Stone, "보낸거" )
            self.client.send( Stone )
            return self.ReceiveStone()
        
        except Exception as e:

            return f'failed... Reason: { e }'
        
        finally:

            self.s.close()

    def ReceiveStone( self, buffer_size: int = 16 ) -> [ StructStoneHeader, StructStonePayload ]:

        Stone = self.client.recv( buffer_size )

        if len(Stone) == 16:
            packat = StructStoneHeader( Stone[0:4], Stone[4:8], Stone[8:16] )

            if packat.StoneSize:
                return packat, self.ParsingPacket( self.ReceiveStone( struct.unpack('Q', packat.StoneSize )[0] ) )
            
            return packat, None
        
        return None, Stone
            