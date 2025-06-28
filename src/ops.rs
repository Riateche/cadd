macro_rules! declare_binary_trait {
    ($trait_:ident, $trait_fn:ident, $doc:literal) => {
        #[doc = $doc]
        pub trait $trait_<Other = Self>: Sized {
            type Error;
            type Output;
            fn $trait_fn(self, b: Other) -> Result<Self::Output, Self::Error>;
        }

        #[doc = $doc]
        pub fn $trait_fn<T1, T2>(a: T1, b: T2) -> Result<T1::Output, T1::Error>
        where
            T1: $trait_<T2>,
        {
            a.$trait_fn(b)
        }
    };
}

macro_rules! declare_unary_trait {
    ($trait_:ident, $trait_fn:ident, $doc:literal) => {
        #[doc = $doc]
        pub trait $trait_: Sized {
            type Error;
            type Output;
            fn $trait_fn(self) -> Result<Self::Output, Self::Error>;
        }

        #[doc = $doc]
        pub fn $trait_fn<T1>(value: T1) -> Result<T1::Output, T1::Error>
        where
            T1: $trait_,
        {
            value.$trait_fn()
        }
    };
}

declare_binary_trait!(
    Cadd,
    cadd,
    "Addition (`a + b`) that returns an error on overflow."
);
declare_binary_trait!(
    Csub,
    csub,
    "Subtraction (`a - b`) that returns an error on overflow."
);
declare_unary_trait!(
    Cneg,
    cneg,
    "Negation (`-a`) that return an error on overflow."
);
declare_binary_trait!(
    Cmul,
    cmul,
    "Multiplication (`a * b`) that returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    Cdiv,
    cdiv,
    "Division (`a / b`) that returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    CdivEuclid,
    cdiv_euclid,
    "Euclidian division that returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    Crem,
    crem,
    "Remainder (`a % b`) that returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    CremEuclid,
    crem_euclid,
    "Euclidian reminder that returns an error on overflow or if the divisor is zero."
);

declare_binary_trait!(
    CILog,
    cilog,
    "Logarithm (<code>log<sub>b</sub> a</code>) that return an error if the number is negative or zero, or if the base is less than 2."
);
declare_unary_trait!(
    CILog2,
    cilog2,
    "Base 2 logarithm (`ln a`) that return an error if the number is negative or zero."
);
declare_unary_trait!(
    CILog10,
    cilog10,
    "Base 10 logarithm (<code>log<sub>10</sub> a</code>) that return an error if the number is negative or zero."
);
declare_binary_trait!(
    Cshl,
    cshl,
    "Shift left (`a << b`) that return an error if `b` is greater or equal to the number of bits in the type."
);
declare_binary_trait!(
    Cshr,
    cshr,
    "Shift right (`a >> b`) that return an error if `b` is greater or equal to the number of bits in the type."
);
declare_binary_trait!(
    Cpow,
    cpow,
    "Exponentiation (<code>a<sup>b</sup></code>) that return an error on overflow."
);
declare_unary_trait!(
    Cabs,
    cabs,
    "Checked absolute value (`|a|`) for signed types that return an error if `a == MIN`."
);
declare_unary_trait!(
    Cisqrt,
    cisqrt,
    "Checked square root (`√a`) for signed types that return an error if `a` is negative."
);
declare_binary_trait!(
    CnextMultipleOf,
    cnext_multiple_of,
    "Next multiple of `b` that returns an error on overflow or if `b` is zero."
);
declare_unary_trait!(
    CnextPowerOfTwo,
    cnext_power_of_two,
    "Next power of 2 that returns an error on overflow."
);
