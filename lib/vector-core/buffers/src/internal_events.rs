use core_common::internal_event::InternalEvent;
use metrics::counter;

pub struct BufferEventsReceived {
    pub count: u64,
    pub byte_size: usize,
}

impl InternalEvent for BufferEventsReceived {
    fn emit_metrics(&self) {
        counter!("buffer_received_events_total", self.count);
        counter!("buffer_received_bytes_total", self.byte_size as u64);
    }
}

pub struct BufferEventsSent {
    pub count: u64,
    pub byte_size: usize,
}

impl InternalEvent for BufferEventsSent {
    fn emit_metrics(&self) {
        counter!("buffer_sent_events_total", self.count);
        counter!("buffer_sent_bytes_total", self.byte_size as u64);
    }
}

pub struct EventsDropped {
    pub count: u64,
}

impl InternalEvent for EventsDropped {
    fn emit_metrics(&self) {
        counter!("buffer_discarded_events_total", self.count);
    }
}
