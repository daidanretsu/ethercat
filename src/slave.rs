use crate::register::datalink::PortPhysics;
use core::cell::RefCell;
use heapless::Deque;

#[derive(Debug, Clone)]
pub enum SlaveError {
    PDINotOperational,
    UnexpectedALState,
    SMSettingsAreNotCorrect,
    WatchdogTimeout,
    PDOStateError,
    PDOControlError,
    PDOToggleError,
    EarlySMEvnet,
    SMEvnetJitterTooMuch,
    SMEventNotRecieved,
    OutputCalcAndCopyNotFinished,
    Sync0NotRecieved,
    Sync1NotRecieved,
    SyncEventNotDetected,
}

#[derive(Debug, Clone, Default)]
pub struct SlaveID {
    pub(crate) vender_id: u16,
    pub(crate) product_code: u16,
    pub(crate) revision_number: u16,
}

#[derive(Debug, Default)]
pub struct Slave {
    pub info: SlaveInfo,
    pub error: Option<SlaveError>,
    pub al_state: AlState,
    pub mailbox_count: u8,
    pub rx_pdo_mapping: Option<&'static mut [PDOMapping]>,
    pub tx_pdo_mapping: Option<&'static mut [PDOMapping]>,

    pub linked_ports: [bool; 4],

    // for DC init
    pub(crate) dc_context: RefCell<DCContext>,
}

#[derive(Debug, Default)]
pub struct DCContext {
    pub parent_port: Option<(u16, u8)>,
    pub current_port: u8,
    pub recieved_port_time: [u32; 4],
    pub delay: u32,
    pub recieved_time: u64,
    pub offset: u64,
}

#[derive(Debug, Default, Clone)]
pub struct SlaveInfo {
    pub id: SlaveID,
    //pub(crate) configured_address: u16,
    pub ports: [Option<PortPhysics>; 4],
    pub ram_size_kb: u8,

    pub number_of_fmmu: u8,
    pub number_of_sm: u8,

    pub pdo_start_address: Option<u16>,
    pub pdo_ram_size: u16,

    pub sm0: Option<SyncManager>, //sm0
    pub sm1: Option<SyncManager>, //sm1
    pub sm2: Option<SyncManager>, //sm1
    pub sm3: Option<SyncManager>, //sm1

    pub support_dc: bool,
    pub is_dc_range_64bits: bool,
    pub support_fmmu_bit_operation: bool,
    pub support_lrw: bool,
    pub support_rw: bool,

    pub support_coe: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub enum AlState {
    Init = 0x1,
    PreOperational = 0x2,
    Bootstrap = 0x3,
    SafeOperational = 0x4,
    Operational = 0x8,
    Invalid,
}

impl From<u8> for AlState {
    fn from(v: u8) -> Self {
        if v == AlState::Init as u8 {
            AlState::Init
        } else if v == AlState::PreOperational as u8 {
            AlState::PreOperational
        } else if v == AlState::Bootstrap as u8 {
            AlState::Bootstrap
        } else if v == AlState::SafeOperational as u8 {
            AlState::SafeOperational
        } else if v == AlState::PreOperational as u8 {
            AlState::PreOperational
        } else if v == AlState::Operational as u8 {
            AlState::Operational
        } else {
            AlState::Invalid
        }
    }
}

impl Default for AlState {
    fn default() -> Self {
        AlState::Invalid
    }
}

#[derive(Debug, Clone)]
pub enum SyncManager {
    MailboxRx(MailboxSyncManager),
    MailboxTx(MailboxSyncManager),
    ProcessDataRx,
    ProcessDataTx,
}

#[derive(Debug, Clone)]
pub struct MailboxSyncManager {
    pub size: u16,
    pub start_address: u16,
}

#[derive(Debug, Clone)]
pub enum OperationMode {
    FreeRun,
    Sync0Event,
    Sync1Event,
    SyncManagerEvent,
}

impl Default for OperationMode {
    fn default() -> Self {
        OperationMode::FreeRun
    }
}

#[derive(Debug)]
pub struct PDOMapping {
    //pub(crate) index: u16,
    pub(crate) entries: &'static mut [PDOEntry],
}

#[derive(Debug)]
pub struct PDOEntry {
    pub(crate) index: u16,
    pub(crate) sub_index: u8,
    pub(crate) byte_length: u8, // NOTE: not bit length
    pub(crate) data: &'static mut [u8],
}
