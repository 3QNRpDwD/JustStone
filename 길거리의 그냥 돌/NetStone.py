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


    def ParsingPacket( self, Packet: StructStone ) -> StructRawStonePayload:
        Packet = Packet.payload.decode().split("..")
        return StructRawStonePayload(*Packet)

    def SendStone( self, Stone ):

        try:
            print( f"보낸거 : { Stone }")
            self.client.send( Stone )
            return self.ReceiveStone()
        
        except Exception as e:

            return f'failed... Reason: { e }'
        
        finally:

            self.s.close()

    def ReceiveStone( self, buffer_size: int = 16 ) -> StructStone:

        Packet = self.client.recv( buffer_size )

        if len(Packet) != 16:
            return StructStone( None, Packet )
        
        Header = StructStoneHeader( Packet[0:4], Packet[4:8], Packet[8:16] )
        Payload = self.ParsingPacket( self.ReceiveStone( struct.unpack('Q', Header.StoneSize )[0] ) )

        if Header.StoneSize:
            return StructStone( Header, Payload )
        
        return StructStone( Header, None )
        
        
            