
/// Status of the cell representation:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellStatus {

    /// Name of software bundle:
    services: Option<Vec<String>>,

    /// Current CPU usage:
    cpu: Option<usize>,

    /// Current RSS usage:
    mem: Option<usize>,

    /// Current CPUTIME usage:
    time: Option<usize>,

    /// Current WRITES usage:
    io_writes: Option<usize>,

    /// Current READS usage:
    io_reads: Option<usize>,

}
