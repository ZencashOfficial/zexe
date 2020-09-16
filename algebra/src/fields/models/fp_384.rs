use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Read, Result as IoResult, Write, Error as IoError, ErrorKind},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    biginteger::{arithmetic as fa, BigInteger as _BigInteger, BigInteger384 as BigInteger},
    bytes::{FromBytes, ToBytes},
    fields::{Field, FpParameters, LegendreSymbol, PrimeField, SquareRootField},
    MulShort,
};

use unroll::unroll_for_loops;

impl_Fp!(Fp384, Fp384Parameters, 6);