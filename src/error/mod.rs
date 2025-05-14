#[derive(Debug)]
/// Enum defining errors specific to split/recombine operations
pub enum SplitterError {
    /// Error indicating provided path does not resolve to file
    NotFileError,    
    
    /// Error indicating invalid number of ROs provided
    InvalidRoError,
}

impl std::error::Error for SplitterError {}

impl std::fmt::Display for SplitterError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SplitterError::NotFileError => {
                write!(f, "Provided path does not resolve to a file")
            },
            SplitterError::InvalidRoError => {
                write!(f, "Number of ROs must be greater than zero")
            },
		}
	}
}
