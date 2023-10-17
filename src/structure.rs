

pub struct  StoneChain {
    pub previous_stone_hash: Vec<u8>,
    pub stone_hash: Vec<u8>,
    pub stonetree_hash: Vec<u8>,
    pub timestamp: Vec<u8>,
    pub transaction_list: Vec<u8>
}
pub struct StructRawStonePayload {
    pub sysinfo: String,
    pub command_input: String,
    pub command_output: String,
    pub stone_chain: String,
}
#[derive(Debug)]
pub struct StructStonePayload {
    pub sysinfo: Vec<u8>,
    pub command_input: Vec<u8>,
    pub command_output: Vec<u8>,
    pub stone_chain: Vec<u8>
    // pub stone_chain: StoneChain,
}

#[derive(Debug)]
pub struct StructStoneHeader {
    pub stone_status: Vec<u8>,
    pub stone_type: Vec<u8>,
    pub stone_size: Vec<u8>,
}
#[derive(Debug)]
pub struct StructStone {
    pub header: StructStoneHeader,
    pub payload: StructStonePayload,
    pub stone: Vec<u8>
}

pub trait Generator {
    fn stone_generator(self) -> StructStone;
}

impl Generator for StructRawStonePayload {

    fn stone_generator(self) -> StructStone{

        let ssp= StructRawStonePayload::to_vec( self);
        let ssh = StructStoneHeader::from(&ssp);
        let ss = StructStone::from(ssh, ssp);

        ss
    }
}


impl StructRawStonePayload {
    pub fn to_vec(srsp: StructRawStonePayload) -> StructStonePayload {
        let sysinfo        = srsp.sysinfo.as_bytes().to_vec();
        let command_input  = srsp.command_input.as_bytes().to_vec();
        let command_output = srsp.command_output.as_bytes().to_vec();

        StructStonePayload {
            sysinfo,
            command_input,
            command_output,
            stone_chain: vec![]
            // stone_chain: StoneChain {
            //     previous_stone_hash : vec![],
            //     stone_hash          : vec![],
            //     stonetree_hash      : vec![],
            //     timestamp           : vec![],
            //     transaction_list    : vec![]
            // }
        }
    }
}

impl StructStoneHeader {
    pub fn from(payload: &StructStonePayload) -> StructStoneHeader {
        let stone_type = if !payload.sysinfo.is_empty() && payload.command_output.is_empty() && payload.stone_chain.is_empty() {
            [1, 0, 0, 0].to_vec()
        } else if !payload.command_output.is_empty() {
            [2, 0, 0, 0].to_vec()
        } else {
            [3, 0, 0, 0].to_vec()
        };

        let stone_size = (payload.sysinfo.len() + payload.command_input.len() + payload.command_output.len()).to_le_bytes().to_vec();
        let stone_status = 0u32.to_le_bytes().to_vec();

        StructStoneHeader {
            stone_status,
            stone_type ,
            stone_size,
        }
    }
}


impl StructStone {
    pub fn from(header: StructStoneHeader, payload: StructStonePayload) -> StructStone {
        let mut stone: Vec<u8> = Vec::new();
        stone.extend(&header.stone_status);
        stone.extend(&header.stone_type);
        stone.extend(&header.stone_size);
        stone.extend(&payload.sysinfo);
        stone.extend(&payload.command_input);
        stone.extend(&payload.command_output);
        stone.extend(&payload.stone_chain);


        StructStone {
            header,
            payload,
            stone
        }
    }
}



