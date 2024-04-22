use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Debug)]
pub enum Status {
    Ready,
    Error(
        String, // error message(s)
        usize,  // number of errors
    ),
}

impl Add<Status> for Status {
    type Output = Status;
    fn add(self, other: Status) -> Status {
        // combine two errors
        match (&self, &other) {
            // ready + ready = ready
            (Self::Ready, Self::Ready) | // self
            // ready + error = error + ready = error
            (Self::Error(_, _), Self::Ready) => self,
            (Self::Ready, Self::Error(_, _)) => other,
            // error + error = combined errors
            (Self::Error(err_lhs, size1), Self::Error(err_rhs, size2)) => Self::Error(format!("{}\n{}", err_lhs, err_rhs), size1 + size2),
        }
    }
}

impl AddAssign for Status {
    fn add_assign(&mut self, rhs: Self) {
        match (&self, &rhs) {
            // ready + ready = ready
            (Self::Ready, Self::Ready) | // self
            // ready + error = error + ready = error
            (Self::Error(_, _), Self::Ready) => { /* *self = self is redundant and also doesn't work */ }
            (Self::Ready, Self::Error(_, _)) => { *self = rhs; }
            // error + error = combined errors
            (Self::Error(err_lhs, size1), Self::Error(err_rhs, size2)) => {
                *self = Self::Error(format!("{}\n{}", err_lhs, err_rhs), size1 + size2)
            }
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ready => {
                write!(f, "Ready")
            }
            Status::Error(e, size) => {
                let plural = if *size == 1 { "" } else { "s" };
                write!(f, "{size} Error{plural}:\n{}", e)
            }
        }
    }
}
