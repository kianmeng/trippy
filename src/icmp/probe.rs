use crate::icmp::tracer::{Index, Round, TimeToLive};
use crate::icmp::util::RemModU16Max;
use std::net::IpAddr;
use std::time::SystemTime;

/// The state of an ICMP echo request/response
#[derive(Debug, Clone, Copy, Default)]
pub struct Probe {
    /// The unique index of the probe.
    pub index: Index,
    /// The TTL of the probe.
    pub ttl: TimeToLive,
    /// Which round the probe belongs to.
    pub round: Round,
    /// Timestamp when the probe was sent.
    pub sent: Option<SystemTime>,
    /// The status of the probe.
    pub status: ProbeStatus,
    /// The host which responded to the probe.
    pub host: Option<IpAddr>,
    /// Timestamp when the response to the probe was received.
    pub received: Option<SystemTime>,
    /// The type of ICMP response packet received for the probe.
    pub icmp_packet_type: Option<IcmpPacketType>,
}

impl Probe {
    #[must_use]
    pub const fn new(index: Index, ttl: TimeToLive, round: Round, sent: SystemTime) -> Self {
        Self {
            index,
            ttl,
            round,
            sent: Some(sent),
            status: ProbeStatus::Awaited,
            host: None,
            received: None,
            icmp_packet_type: None,
        }
    }

    /// The sequence number for the probe.
    ///
    /// The sequence number is always the `index` modulo `u16::MAX`.
    #[must_use]
    pub fn sequence(&self) -> u16 {
        self.index.0.rem_u16max()
    }

    #[must_use]
    pub const fn with_status(self, status: ProbeStatus) -> Self {
        Self { status, ..self }
    }

    #[must_use]
    pub const fn with_icmp_packet_type(self, icmp_packet_type: IcmpPacketType) -> Self {
        Self {
            icmp_packet_type: Some(icmp_packet_type),
            ..self
        }
    }

    #[must_use]
    pub const fn with_host(self, host: IpAddr) -> Self {
        Self {
            host: Some(host),
            ..self
        }
    }

    #[must_use]
    pub const fn with_received(self, received: SystemTime) -> Self {
        Self {
            received: Some(received),
            ..self
        }
    }
}

/// The status of a `Echo` for a single TTL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeStatus {
    /// The probe has not been sent.
    NotSent,
    /// The probe has been sent and we are awaiting the response.
    Awaited,
    /// The probe has been sent and a response (`EchoReply`, `DestinationUnreachable` or `TimeExceeded`) has
    /// been received.
    Complete,
}

impl Default for ProbeStatus {
    fn default() -> Self {
        Self::NotSent
    }
}

/// The type of ICMP packet received.
#[derive(Debug, Clone, Copy)]
pub enum IcmpPacketType {
    TimeExceeded,
    EchoReply,
    Unreachable,
}
