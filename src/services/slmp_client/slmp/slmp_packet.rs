

fn create_read_request(deviceCode: SlmpDeviceCode, offset: usize, wordCount: usize) {
    // Creates and returns packet ready to send.
    // Packet contains device read command from D(word) register of specific device number.
    // :return: packet
    let slmpHeadDevice = int(offset).to_bytes(3, byteorder = 'little')
    let slmpWordCount = int(wordCount).to_bytes(2, byteorder = 'little')
    let pucData = b''.join([slmpHeadDevice, deviceCode.value, slmpWordCount])
    let slmpPacket = SLMPPacket(
        ulFrameType=FrameType.SLMP_FTYPE_BIN_REQ_ST.value, 
        usNetNumber=0, 
        usNodeNumber=0xFF,
        usProcNumber=ProcessorNumber.SLMP_CPU_DEFAULT.value,
        usTimer=TimerValue.SLMP_TIMER_WAIT_FOREVER.value,
        usCommand=SLMPCommand.SLMP_COMMAND_DEVICE_READ.value,
        usSubCommand=SLMPSubCommand.SUB_word0.value, pucData=pucData
    )
    packet = slmpPacket.create_stream()
    return packet
}



///
/// SLMP Info structure in c-type.
/// Used in SLMP_MakePacketStream as argument.
/// It needs to be prepared like that because of C language and pointers.
/// Frame type     -> ASCII/BIN + REQUEST/RESPONSE + ST/MT
/// Serial number  -> used only with MT(multiple transmission) frame types, marks request and response with this number
/// Net number     -> network number(1 to 239), can be fined in parameters of target device (e.g. SLMPNWNO)
/// Node number    -> station number (1 to 120), can be fined in parameters of target device (e.g. SLMPNDID)
/// Proc number    -> destination unit I/O number
/// Data length    -> length of data from usTimer to end of pucData, use function get_data_length
/// Timer          -> monitoring timer
/// Command        -> SLMP command (e.g. SLMP_COMMAND_DEVICE_READ, SLMP_COMMAND_SELF_TEST)
/// SubCommand     -> 0x0001/0x0003 = per bit, 0x0000/0x0002 = per word, 0x0080/0x0082 = per word on CPU
/// EndCode        -> used mostly in response
/// Data           -> data which obtains device number, number of devices, its like arguments of used command
struct SLMPInfoC {
    _fields_ = [
        ("ulFrameType", ctypes.c_ulong),
        ("usSerialNumber", ctypes.c_ushort),
        ("usNetNumber", ctypes.c_ushort),
        ("usNodeNumber", ctypes.c_ushort),
        ("usProcNumber", ctypes.c_ushort),
        ("usDataLength", ctypes.c_ushort),
        ("usTimer", ctypes.c_ushort),
        ("usCommand", ctypes.c_ushort),
        ("usSubCommand", ctypes.c_ushort),
        ("usEndCode", ctypes.c_ushort),
        ("pucData", ctypes.c_char_p)
    ]
}
///
///        Initialize and create SLMPInfoC structure.
///        Load C functions from .so file.
///        From SLMPInfoC structure is created packet with create_stream method.
///
///        :param ulFrameType: ASCII/BIN + REQUEST/RESPONSE + ST/MT
///        :param usSerialNumber: used only with MT(multiple transmission) frame types,
///                               marks request and response with this number
///        :param usNetNumber: network number(1 to 239), can be fined in parameters of target device (e.g. SLMPNWNO)
///        :param usNodeNumber: station number (1 to 120), can be fined in parameters of target device (e.g. SLMPNDID)
///        :param usProcNumber: destination unit I/O number
///        :param usDataLength: length of data from usTimer to end of pucData, use function get_data_length
///        :param usTimer: monitoring timer
///        :param usCommand: SLMP command (e.g. SLMP_COMMAND_DEVICE_READ, SLMP_COMMAND_SELF_TEST)
///        :param usSubCommand: Sub command -> 0x0001/0x0003 = per bit,
///                                            0x0000/0x0002 = per word,
///                                            0x0080/0x0082 = per word on CPU
///        :param usEndCode: used mostly in response
///        :param pucData: data which obtains device number, number of devices, its like arguments of used command
struct SLMPPacket {
    ulFrameType: usize,
    usSerialNumber: usize,
    usNetNumber: usize,
    usNodeNumber: usize,
    usProcNumber: usize,
    usDataLength: usize,
    usTimer: usize,
    usCommand: usize,
    usSubCommand: usize,
    usEndCode: usize,
    pucData: usize,
    slmp_info_c: SLMPInfoC,
}
//
//
impl SLMPPacket {
    ///
    /// 
    pub fn new(
        ulFrameType: usize,
        usSerialNumber:    Option<usize>,  // =0x0000,
        usNetNumber:       Option<usize>,  // =0,
        usNodeNumber:      Option<usize>,  // =0x00,
        usProcNumber:      Option<usize>,  // =0x0000,
        usDataLength:      Option<usize>,  // =0,
        usTimer:           Option<usize>,  // =0x0000,
        usCommand:         Option<usize>,  // =0x0000,
        usSubCommand:      Option<usize>,  // =0x0000,
        usEndCode:         Option<usize>,  // =0x0000,
        pucData:           &str,           // =b""
    ) {
        c_lib = ctypes.CDLL(so_file)
        try:        # Loading SLMP.h, SLMP.c
            self.SLMP_MakePacketStream = c_lib.SLMP_MakePacketStream
            self.SLMP_MakePacketStream.argtypes = [ctypes.c_ulong,
                                                   ctypes.POINTER(SLMPInfoC),
                                                   ctypes.POINTER(ctypes.c_ubyte * MAX_FRAME_SIZE)]
            self.SLMP_MakePacketStream.restype = ctypes.c_int
        except RuntimeError:
            self.logger.critical("ERR: cannot use SLMP.h/SLMP.c")
            raise ClibImportErr

        # Creating SLMPInfoC structure
        let slmp_info_c = SLMPInfoC::new(
            ulFrameType, usSerialNumber, usNetNumber, usNodeNumber,
            usProcNumber, usDataLength, usTimer, usCommand,
            usSubCommand, usEndCode, pucData,
        );
        let instance = Self {
            ulFrameType: ulFrameType
            usSerialNumber: usSerialNumber
            usNetNumber: usNetNumber
            usNodeNumber: usNodeNumber
            usProcNumber: usProcNumber
            usTimer: usTimer
            usCommand: usCommand
            usSubCommand: usSubCommand
            usEndCode: usEndCode
            pucData: pucData
            usDataLength: Self::get_data_length()
            slmp_info_c
        }
    }
    ///
    /// 
    fn create_stream(self) {
        """
        Creates packet, ready to send.
        This packet is created with c function SLMP_MakePacketStream, which is from official library.
        :return: packet
        """
        data = [0] * MAX_FRAME_SIZE     # Maximal length in bytes of your request packet
        puc_stream = (ctypes.c_ubyte * MAX_FRAME_SIZE)(*data)

        # Calling C function, check src/clib/SLMP.*
        try:
            res = self.SLMP_MakePacketStream(self.ulFrameType, self.slmp_info_c, puc_stream)
            self.logger.info("packet stream created")
        except:
            self.logger.critical("ERR: cannot create SLMP packet")
            raise ClibPacketErr

        # Return only part of packet with data and remove empty spaces
        if self.ulFrameType == FrameType.SLMP_FTYPE_BIN_REQ_ST.value or FrameType.SLMP_FTYPE_BIN_RES_ST.value:
            puc_stream = bytearray(puc_stream)[:(9 + self.usDataLength)]
        elif self.ulFrameType == FrameType.SLMP_FTYPE_BIN_REQ_MT.value or FrameType.SLMP_FTYPE_BIN_RES_MT.value:
            puc_stream = bytearray(puc_stream)[:(13 + self.usDataLength)]
        elif self.ulFrameType == FrameType.SLMP_FTYPE_ASCII_REQ_ST.value or FrameType.SLMP_FTYPE_ASCII_RES_ST.value:
            puc_stream = bytearray(puc_stream)[:(18 + self.usDataLength)]
        elif self.ulFrameType == FrameType.SLMP_FTYPE_ASCII_REQ_MT.value or FrameType.SLMP_FTYPE_ASCII_RES_MT.value:
            puc_stream = bytearray(puc_stream)[:(26 + self.usDataLength)]
        return puc_stream
    }

    /// 
    /// Gets usDataLength for PacketCreator.
    /// TODO is there difference with length of serial number ? Check and add +2 to BIN and +4 to ASCII
    /// :return: data size
    fn get_data_length(self) {
        length = len(self.pucData)
        if self.ulFrameType == FrameType.SLMP_FTYPE_BIN_REQ_ST.value:
            length += 6
        elif self.ulFrameType == FrameType.SLMP_FTYPE_BIN_REQ_MT.value:
            length += 6
        elif self.ulFrameType == FrameType.SLMP_FTYPE_BIN_RES_ST.value:
            length += 2
        elif self.ulFrameType == FrameType.SLMP_FTYPE_BIN_RES_MT.value:
            length += 2
        elif self.ulFrameType == FrameType.SLMP_FTYPE_ASCII_REQ_ST.value:
            length += 12
        elif self.ulFrameType == FrameType.SLMP_FTYPE_ASCII_REQ_MT.value:
            length += 12
        elif self.ulFrameType == FrameType.SLMP_FTYPE_ASCII_RES_ST.value:
            length += 4
        elif self.ulFrameType == FrameType.SLMP_FTYPE_ASCII_RES_MT.value:
            length += 4
        else:
            length = 0

        return length
    }
}