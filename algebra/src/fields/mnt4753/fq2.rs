//! The base field of the quadratic twist, a quadratic extension of Fq.

use crate::field_new;
use crate::{
    biginteger::BigInteger768 as BigInteger,
    fields::{
        mnt4753::fq::Fq,
        fp2::{Fp2, Fp2Parameters},
    },
};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    /// NONRESIDUE = alpha = 13
    /// is a non-square in Fq
    const NONRESIDUE: Fq = field_new!(Fq, BigInteger([
        11881297496860141143,
        13588356353764843511,
        9969398190777826186,
        17325157081734070311,
        16341533986183788031,
        8322434028726676858,
        13631157743146294957,
        8365783422740577875,
        3010239015809771096,
        11776256826687733591,
        7214251687253691272,
        268626707558702,
    ]));

    /// QUADRATIC_NONRESIDUE  in Fq2 = (8,1)
    /// Montg. rep.
    const QUADRATIC_NONRESIDUE: (Fq, Fq) = (
        field_new!(Fq, BigInteger([
            587330122779359758,
            14352661462510473462,
            17802452401246596498,
            18018663494943049411,
            17948754733747257098,
            10253180574146027531,
            6683223122694781837,
            13573468617269213174,
            5059368039312883748,
            950479668716233863,
            9936591501985804621,
            88719447132658
        ])),

        field_new!(Fq, BigInteger([
            11000302312691101506,
            10506108233708684934,
            10935835699472258862,
            8743905913029047809,
            17088517996127229807,
            2166035204362411368,
            3606323059104122201,
            6452324570546309730,
            4644558993695221281,
            1127165286758606988,
            10756108507984535957,
            135547536859714
        ])),
    );

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP2_C1: [Fq; 2] = [

        // X^{q^0} = alpha^((q^0 - 1)/2)*X = 1*X
        field_new!(Fq, BigInteger([
            11000302312691101506,
            10506108233708684934,
            10935835699472258862,
            8743905913029047809,
            17088517996127229807,
            2166035204362411368,
            3606323059104122201,
            6452324570546309730,
            4644558993695221281,
            1127165286758606988,
            10756108507984535957,
            135547536859714
        ])),

        // alpha^((q^1 - 1)/2)
        field_new!(Fq, BigInteger([
            14260497802974073023,
            5895249896161266456,
            14682908860938702530,
            17222385991615618722,
            14621060510943733448,
            10594887362868996148,
            7477357615964975684,
            12570239403004322603,
            2180620924574446161,
            12129628062772479841,
            8853285699251153944,
            362282887012814
        ])),
    ];

}
