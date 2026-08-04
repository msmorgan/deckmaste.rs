mod __spike_golden {
    #[derive(Debug, PartialEq, Eq)]
    pub struct SpikeError;
    pub struct SpikeCoordination {
        members: u16,
    }
    impl SpikeCoordination {
        pub fn try_new(members: u16) -> Result<Self, SpikeError> {
            if members == 0 {
                return Err(SpikeError);
            }
            Ok(Self { members })
        }
        pub fn members(&self) -> u16 {
            self.members
        }
    }
}
pub use __spike_golden::SpikeCoordination;
pub use __spike_golden::SpikeError;
