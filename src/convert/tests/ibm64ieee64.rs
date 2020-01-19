use super::*;

struct Testcase(u64, u64);
impl Testcase {
    fn verify(&self) {
        let actual = ibm64ieee64(self.0);
        assert_eq!(
            actual, self.1,
            "ibm64ieee64(0x{:016x}): got 0x{:016x}, expected 0x{:016x}",
            self.0, actual, self.1
        );
    }
}

#[test]
fn round_ties_to_even() {
    for testcase in &[
        Testcase(0x01f6b0aeaeaeffff, 0x302ed615d5d5e000),
        Testcase(0x0073000000000873, 0x2fdcc0000000021d),
        Testcase(0xe39c9c9c9cffffff, 0xc8a3939393a00000),
        Testcase(0xf6f5ffbf33bff595, 0xcd6ebff7e677feb3),
    ] {
        testcase.verify();
    }
}

#[cfg(feature = "std")]
#[test]
fn python_test_cases() {
    for testcase in vec![
        Testcase(0x0000000000000001, 0x2c70000000000000),
        Testcase(0x8000000000000001, 0xac70000000000000),
        Testcase(0x0000000000000002, 0x2c80000000000000),
        Testcase(0x8000000000000002, 0xac80000000000000),
        Testcase(0x0000000000000003, 0x2c88000000000000),
        Testcase(0x8000000000000003, 0xac88000000000000),
        Testcase(0x400ffffffffffffe, 0x3faffffffffffffc),
        Testcase(0xc00ffffffffffffe, 0xbfaffffffffffffc),
        Testcase(0x400fffffffffffff, 0x3faffffffffffffe),
        Testcase(0xc00fffffffffffff, 0xbfaffffffffffffe),
        Testcase(0x4010000000000000, 0x3fb0000000000000),
        Testcase(0xc010000000000000, 0xbfb0000000000000),
        Testcase(0x4010000000000001, 0x3fb0000000000001),
        Testcase(0xc010000000000001, 0xbfb0000000000001),
        Testcase(0x4010000000000002, 0x3fb0000000000002),
        Testcase(0xc010000000000002, 0xbfb0000000000002),
        Testcase(0x401ffffffffffffe, 0x3fbffffffffffffe),
        Testcase(0xc01ffffffffffffe, 0xbfbffffffffffffe),
        Testcase(0x401fffffffffffff, 0x3fbfffffffffffff),
        Testcase(0xc01fffffffffffff, 0xbfbfffffffffffff),
        Testcase(0x4020000000000000, 0x3fc0000000000000),
        Testcase(0xc020000000000000, 0xbfc0000000000000),
        Testcase(0x4020000000000001, 0x3fc0000000000000),
        Testcase(0xc020000000000001, 0xbfc0000000000000),
        Testcase(0x4020000000000002, 0x3fc0000000000001),
        Testcase(0xc020000000000002, 0xbfc0000000000001),
        Testcase(0x4020000000000003, 0x3fc0000000000002),
        Testcase(0xc020000000000003, 0xbfc0000000000002),
        Testcase(0x403ffffffffffffd, 0x3fcffffffffffffe),
        Testcase(0xc03ffffffffffffd, 0xbfcffffffffffffe),
        Testcase(0x403ffffffffffffe, 0x3fcfffffffffffff),
        Testcase(0xc03ffffffffffffe, 0xbfcfffffffffffff),
        Testcase(0x403fffffffffffff, 0x3fd0000000000000),
        Testcase(0xc03fffffffffffff, 0xbfd0000000000000),
        Testcase(0x4040000000000002, 0x3fd0000000000000),
        Testcase(0xc040000000000002, 0xbfd0000000000000),
        Testcase(0x4040000000000003, 0x3fd0000000000001),
        Testcase(0xc040000000000003, 0xbfd0000000000001),
        Testcase(0x4040000000000005, 0x3fd0000000000001),
        Testcase(0xc040000000000005, 0xbfd0000000000001),
        Testcase(0x4040000000000006, 0x3fd0000000000002),
        Testcase(0xc040000000000006, 0xbfd0000000000002),
        Testcase(0x407ffffffffffffa, 0x3fdffffffffffffe),
        Testcase(0xc07ffffffffffffa, 0xbfdffffffffffffe),
        Testcase(0x407ffffffffffffb, 0x3fdfffffffffffff),
        Testcase(0xc07ffffffffffffb, 0xbfdfffffffffffff),
        Testcase(0x407ffffffffffffd, 0x3fdfffffffffffff),
        Testcase(0xc07ffffffffffffd, 0xbfdfffffffffffff),
        Testcase(0x407ffffffffffffe, 0x3fe0000000000000),
        Testcase(0xc07ffffffffffffe, 0xbfe0000000000000),
        Testcase(0x4080000000000004, 0x3fe0000000000000),
        Testcase(0xc080000000000004, 0xbfe0000000000000),
        Testcase(0x4080000000000005, 0x3fe0000000000001),
        Testcase(0xc080000000000005, 0xbfe0000000000001),
        Testcase(0x408000000000000b, 0x3fe0000000000001),
        Testcase(0xc08000000000000b, 0xbfe0000000000001),
        Testcase(0x408000000000000c, 0x3fe0000000000002),
        Testcase(0xc08000000000000c, 0xbfe0000000000002),
        Testcase(0x40fffffffffffff4, 0x3feffffffffffffe),
        Testcase(0xc0fffffffffffff4, 0xbfeffffffffffffe),
        Testcase(0x40fffffffffffff5, 0x3fefffffffffffff),
        Testcase(0xc0fffffffffffff5, 0xbfefffffffffffff),
        Testcase(0x40fffffffffffffb, 0x3fefffffffffffff),
        Testcase(0xc0fffffffffffffb, 0xbfefffffffffffff),
        Testcase(0x40fffffffffffffc, 0x3ff0000000000000),
        Testcase(0xc0fffffffffffffc, 0xbff0000000000000),
        Testcase(0x4110000000000000, 0x3ff0000000000000),
        Testcase(0xc110000000000000, 0xbff0000000000000),
        Testcase(0x4110000000000001, 0x3ff0000000000001),
        Testcase(0xc110000000000001, 0xbff0000000000001),
        Testcase(0x4110000000000002, 0x3ff0000000000002),
        Testcase(0xc110000000000002, 0xbff0000000000002),
        Testcase(0x411fffffffffffff, 0x3fffffffffffffff),
        Testcase(0xc11fffffffffffff, 0xbfffffffffffffff),
        Testcase(0x4120000000000000, 0x4000000000000000),
        Testcase(0xc120000000000000, 0xc000000000000000),
        Testcase(0x4120000000000001, 0x4000000000000000),
        Testcase(0xc120000000000001, 0xc000000000000000),
        Testcase(0x4120000000000002, 0x4000000000000001),
        Testcase(0xc120000000000002, 0xc000000000000001),
        Testcase(0x4800000000000001, 0x3e70000000000000),
        Testcase(0xc800000000000001, 0xbe70000000000000),
        Testcase(0x4800000000000002, 0x3e80000000000000),
        Testcase(0xc800000000000002, 0xbe80000000000000),
        Testcase(0x4800000000000004, 0x3e90000000000000),
        Testcase(0xc800000000000004, 0xbe90000000000000),
        Testcase(0x4800000000000008, 0x3ea0000000000000),
        Testcase(0xc800000000000008, 0xbea0000000000000),
        Testcase(0x4800000000000010, 0x3eb0000000000000),
        Testcase(0xc800000000000010, 0xbeb0000000000000),
        Testcase(0x4800000000000020, 0x3ec0000000000000),
        Testcase(0xc800000000000020, 0xbec0000000000000),
        Testcase(0x4800000000000040, 0x3ed0000000000000),
        Testcase(0xc800000000000040, 0xbed0000000000000),
        Testcase(0x4800000000000080, 0x3ee0000000000000),
        Testcase(0xc800000000000080, 0xbee0000000000000),
        Testcase(0x4800000000000100, 0x3ef0000000000000),
        Testcase(0xc800000000000100, 0xbef0000000000000),
        Testcase(0x4800000000000200, 0x3f00000000000000),
        Testcase(0xc800000000000200, 0xbf00000000000000),
        Testcase(0x4800000000000400, 0x3f10000000000000),
        Testcase(0xc800000000000400, 0xbf10000000000000),
        Testcase(0x4800000000000800, 0x3f20000000000000),
        Testcase(0xc800000000000800, 0xbf20000000000000),
        Testcase(0x4800000000001000, 0x3f30000000000000),
        Testcase(0xc800000000001000, 0xbf30000000000000),
        Testcase(0x4800000000002000, 0x3f40000000000000),
        Testcase(0xc800000000002000, 0xbf40000000000000),
        Testcase(0x4800000000004000, 0x3f50000000000000),
        Testcase(0xc800000000004000, 0xbf50000000000000),
        Testcase(0x4800000000008000, 0x3f60000000000000),
        Testcase(0xc800000000008000, 0xbf60000000000000),
        Testcase(0x4800000000010000, 0x3f70000000000000),
        Testcase(0xc800000000010000, 0xbf70000000000000),
        Testcase(0x4800000000020000, 0x3f80000000000000),
        Testcase(0xc800000000020000, 0xbf80000000000000),
        Testcase(0x4800000000040000, 0x3f90000000000000),
        Testcase(0xc800000000040000, 0xbf90000000000000),
        Testcase(0x4800000000080000, 0x3fa0000000000000),
        Testcase(0xc800000000080000, 0xbfa0000000000000),
        Testcase(0x4800000000100000, 0x3fb0000000000000),
        Testcase(0xc800000000100000, 0xbfb0000000000000),
        Testcase(0x4800000000200000, 0x3fc0000000000000),
        Testcase(0xc800000000200000, 0xbfc0000000000000),
        Testcase(0x4800000000400000, 0x3fd0000000000000),
        Testcase(0xc800000000400000, 0xbfd0000000000000),
        Testcase(0x4800000000800000, 0x3fe0000000000000),
        Testcase(0xc800000000800000, 0xbfe0000000000000),
        Testcase(0x4800000001000000, 0x3ff0000000000000),
        Testcase(0xc800000001000000, 0xbff0000000000000),
        Testcase(0x4800000002000000, 0x4000000000000000),
        Testcase(0xc800000002000000, 0xc000000000000000),
        Testcase(0x4800000004000000, 0x4010000000000000),
        Testcase(0xc800000004000000, 0xc010000000000000),
        Testcase(0x4800000008000000, 0x4020000000000000),
        Testcase(0xc800000008000000, 0xc020000000000000),
        Testcase(0x4800000010000000, 0x4030000000000000),
        Testcase(0xc800000010000000, 0xc030000000000000),
        Testcase(0x4800000020000000, 0x4040000000000000),
        Testcase(0xc800000020000000, 0xc040000000000000),
        Testcase(0x4800000040000000, 0x4050000000000000),
        Testcase(0xc800000040000000, 0xc050000000000000),
        Testcase(0x4800000080000000, 0x4060000000000000),
        Testcase(0xc800000080000000, 0xc060000000000000),
        Testcase(0x4800000100000000, 0x4070000000000000),
        Testcase(0xc800000100000000, 0xc070000000000000),
        Testcase(0x4800000200000000, 0x4080000000000000),
        Testcase(0xc800000200000000, 0xc080000000000000),
        Testcase(0x4800000400000000, 0x4090000000000000),
        Testcase(0xc800000400000000, 0xc090000000000000),
        Testcase(0x4800000800000000, 0x40a0000000000000),
        Testcase(0xc800000800000000, 0xc0a0000000000000),
        Testcase(0x4800001000000000, 0x40b0000000000000),
        Testcase(0xc800001000000000, 0xc0b0000000000000),
        Testcase(0x4800002000000000, 0x40c0000000000000),
        Testcase(0xc800002000000000, 0xc0c0000000000000),
        Testcase(0x4800004000000000, 0x40d0000000000000),
        Testcase(0xc800004000000000, 0xc0d0000000000000),
        Testcase(0x4800008000000000, 0x40e0000000000000),
        Testcase(0xc800008000000000, 0xc0e0000000000000),
        Testcase(0x4800010000000000, 0x40f0000000000000),
        Testcase(0xc800010000000000, 0xc0f0000000000000),
        Testcase(0x4800020000000000, 0x4100000000000000),
        Testcase(0xc800020000000000, 0xc100000000000000),
        Testcase(0x4800040000000000, 0x4110000000000000),
        Testcase(0xc800040000000000, 0xc110000000000000),
        Testcase(0x4800080000000000, 0x4120000000000000),
        Testcase(0xc800080000000000, 0xc120000000000000),
        Testcase(0x4800100000000000, 0x4130000000000000),
        Testcase(0xc800100000000000, 0xc130000000000000),
        Testcase(0x4800200000000000, 0x4140000000000000),
        Testcase(0xc800200000000000, 0xc140000000000000),
        Testcase(0x4800400000000000, 0x4150000000000000),
        Testcase(0xc800400000000000, 0xc150000000000000),
        Testcase(0x4800800000000000, 0x4160000000000000),
        Testcase(0xc800800000000000, 0xc160000000000000),
        Testcase(0x4801000000000000, 0x4170000000000000),
        Testcase(0xc801000000000000, 0xc170000000000000),
        Testcase(0x4802000000000000, 0x4180000000000000),
        Testcase(0xc802000000000000, 0xc180000000000000),
        Testcase(0x4804000000000000, 0x4190000000000000),
        Testcase(0xc804000000000000, 0xc190000000000000),
        Testcase(0x4808000000000000, 0x41a0000000000000),
        Testcase(0xc808000000000000, 0xc1a0000000000000),
        Testcase(0x4810000000000000, 0x41b0000000000000),
        Testcase(0xc810000000000000, 0xc1b0000000000000),
        Testcase(0x4820000000000000, 0x41c0000000000000),
        Testcase(0xc820000000000000, 0xc1c0000000000000),
        Testcase(0x4840000000000000, 0x41d0000000000000),
        Testcase(0xc840000000000000, 0xc1d0000000000000),
        Testcase(0x4880000000000000, 0x41e0000000000000),
        Testcase(0xc880000000000000, 0xc1e0000000000000),
        Testcase(0x567faef3ff3dc282, 0x455febbcffcf70a0),
        Testcase(0xd67faef3ff3dc282, 0xc55febbcffcf70a0),
        Testcase(0x7ffffffffffffff4, 0x4faffffffffffffe),
        Testcase(0xfffffffffffffff4, 0xcfaffffffffffffe),
        Testcase(0x7ffffffffffffff5, 0x4fafffffffffffff),
        Testcase(0xfffffffffffffff5, 0xcfafffffffffffff),
        Testcase(0x7ffffffffffffffb, 0x4fafffffffffffff),
        Testcase(0xfffffffffffffffb, 0xcfafffffffffffff),
        Testcase(0x7ffffffffffffffc, 0x4fb0000000000000),
        Testcase(0xfffffffffffffffc, 0xcfb0000000000000),
        Testcase(0x7ffffffffffffffd, 0x4fb0000000000000),
        Testcase(0xfffffffffffffffd, 0xcfb0000000000000),
        Testcase(0x7ffffffffffffffe, 0x4fb0000000000000),
        Testcase(0xfffffffffffffffe, 0xcfb0000000000000),
        Testcase(0x7fffffffffffffff, 0x4fb0000000000000),
        Testcase(0xffffffffffffffff, 0xcfb0000000000000),
        Testcase(0x0000000000000000, 0x0000000000000000),
        Testcase(0x8000000000000000, 0x8000000000000000),
        Testcase(0x0000000100000000, 0x2e70000000000000),
        Testcase(0x8000000100000000, 0xae70000000000000),
        Testcase(0x00ffffff00000000, 0x2fefffffe0000000),
        Testcase(0x80ffffff00000000, 0xafefffffe0000000),
        Testcase(0x4110000000000000, 0x3ff0000000000000),
        Testcase(0xc110000000000000, 0xbff0000000000000),
        Testcase(0x7fffffff00000000, 0x4fafffffe0000000),
        Testcase(0xffffffff00000000, 0xcfafffffe0000000),
    ] {
        testcase.verify();
    }
}
