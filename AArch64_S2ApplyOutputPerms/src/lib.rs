#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use HaveExtendedExecuteNeverExt::*;
use Bit::*;
use u_get_VTCR_EL2_Type_S2POE::*;
use HaveS2POExt::*;
use common::*;
pub fn AArch64_S2ApplyOutputPerms<T: Tracer>(
    state: &mut State,
    tracer: &T,
    descriptor: Bits,
    walkparams: ProductTypeb05ce25a107f0c5e,
) -> ProductTypebf05c51f33174538 {
    #[derive(Default)]
    struct FunctionState {
        gs_19145: bool,
        gs_19179: bool,
        ga_14336: ProductType5c790c8ef59cc8b2,
        gs_19144: bool,
        s2pi_index: u8,
        permissions: ProductTypebf05c51f33174538,
        gs_19180: bool,
        desc_dbm: bool,
        descriptor: Bits,
        walkparams: ProductTypeb05ce25a107f0c5e,
    }
    let fn_state = FunctionState {
        descriptor,
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_0_0: read-var walkparams.17:struct
        let s_0_0: bool = fn_state.walkparams._17;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b34 b1
        if s_0_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_1_0: read-var descriptor:bv
        let s_1_0: Bits = fn_state.descriptor;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // C s_1_3: const #7s : i
        let s_1_3: i128 = 7;
        // D s_1_4: cmp-lt s_1_3 s_1_2
        let s_1_4: bool = ((s_1_3) < (s_1_2));
        // N s_1_5: assert s_1_4
        let s_1_5: () = assert!(s_1_4);
        // C s_1_6: const #6s : i
        let s_1_6: i128 = 6;
        // D s_1_7: read-var descriptor:bv
        let s_1_7: Bits = fn_state.descriptor;
        // C s_1_8: const #1s : i64
        let s_1_8: i64 = 1;
        // C s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_10: const #1s : i
        let s_1_10: i128 = 1;
        // C s_1_11: add s_1_10 s_1_9
        let s_1_11: i128 = (s_1_10 + s_1_9);
        // D s_1_12: bit-extract s_1_7 s_1_6 s_1_11
        let s_1_12: Bits = (Bits::new(
            ((s_1_7) >> (s_1_6)).value(),
            u16::try_from(s_1_11).unwrap(),
        ));
        // D s_1_13: cast reint s_1_12 -> u8
        let s_1_13: u8 = (s_1_12.value() as u8);
        // D s_1_14: write-var permissions.7 <= s_1_13
        fn_state.permissions._7 = s_1_13;
        // D s_1_15: read-var walkparams.2:struct
        let s_1_15: bool = fn_state.walkparams._2;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 1u16);
        // C s_1_17: const #1u : u8
        let s_1_17: bool = true;
        // C s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // D s_1_19: cmp-eq s_1_16 s_1_18
        let s_1_19: bool = ((s_1_16) == (s_1_18));
        // N s_1_20: branch s_1_19 b33 b2
        if s_1_19 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_2_0: read-var descriptor:bv
        let s_2_0: Bits = fn_state.descriptor;
        // D s_2_1: size-of s_2_0
        let s_2_1: u16 = s_2_0.length();
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // C s_2_3: const #54s : i
        let s_2_3: i128 = 54;
        // D s_2_4: cmp-lt s_2_3 s_2_2
        let s_2_4: bool = ((s_2_3) < (s_2_2));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // C s_2_6: const #54s : i
        let s_2_6: i128 = 54;
        // D s_2_7: read-var descriptor:bv
        let s_2_7: Bits = fn_state.descriptor;
        // C s_2_8: const #1u : u64
        let s_2_8: u64 = 1;
        // D s_2_9: bit-extract s_2_7 s_2_6 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_7) >> (s_2_6)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: bool = ((s_2_9.value()) != 0);
        // C s_2_11: const #0s : i
        let s_2_11: i128 = 0;
        // C s_2_12: const #0u : u64
        let s_2_12: u64 = 0;
        // D s_2_13: cast zx s_2_10 -> u64
        let s_2_13: u64 = (s_2_10 as u64);
        // C s_2_14: const #1u : u64
        let s_2_14: u64 = 1;
        // D s_2_15: and s_2_13 s_2_14
        let s_2_15: u64 = ((s_2_13) & (s_2_14));
        // D s_2_16: cmp-eq s_2_15 s_2_14
        let s_2_16: bool = ((s_2_15) == (s_2_14));
        // D s_2_17: lsl s_2_13 s_2_11
        let s_2_17: u64 = s_2_13 << s_2_11;
        // D s_2_18: or s_2_12 s_2_17
        let s_2_18: u64 = ((s_2_12) | (s_2_17));
        // D s_2_19: cmpl s_2_17
        let s_2_19: u64 = !s_2_17;
        // D s_2_20: and s_2_12 s_2_19
        let s_2_20: u64 = ((s_2_12) & (s_2_19));
        // D s_2_21: select s_2_16 s_2_18 s_2_20
        let s_2_21: u64 = if s_2_16 { s_2_18 } else { s_2_20 };
        // D s_2_22: cast trunc s_2_21 -> u8
        let s_2_22: bool = ((s_2_21) != 0);
        // D s_2_23: write-var permissions.12 <= s_2_22
        fn_state.permissions._12 = s_2_22;
        // N s_2_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveExtendedExecuteNeverExt(s_3_0)
        let s_3_1: bool = HaveExtendedExecuteNeverExt(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b30 b4
        if s_3_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var permissions.13 <= s_4_0
        fn_state.permissions._13 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_5_0: read-var walkparams.2:struct
        let s_5_0: bool = fn_state.walkparams._2;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b29 b6
        if s_5_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_6_0: read-var descriptor:bv
        let s_6_0: Bits = fn_state.descriptor;
        // D s_6_1: size-of s_6_0
        let s_6_1: u16 = s_6_0.length();
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_3: const #51s : i
        let s_6_3: i128 = 51;
        // D s_6_4: cmp-lt s_6_3 s_6_2
        let s_6_4: bool = ((s_6_3) < (s_6_2));
        // N s_6_5: assert s_6_4
        let s_6_5: () = assert!(s_6_4);
        // C s_6_6: const #51s : i
        let s_6_6: i128 = 51;
        // D s_6_7: read-var descriptor:bv
        let s_6_7: Bits = fn_state.descriptor;
        // C s_6_8: const #1u : u64
        let s_6_8: u64 = 1;
        // D s_6_9: bit-extract s_6_7 s_6_6 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_7) >> (s_6_6)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: bool = ((s_6_9.value()) != 0);
        // C s_6_11: const #0s : i
        let s_6_11: i128 = 0;
        // C s_6_12: const #0u : u64
        let s_6_12: u64 = 0;
        // D s_6_13: cast zx s_6_10 -> u64
        let s_6_13: u64 = (s_6_10 as u64);
        // C s_6_14: const #1u : u64
        let s_6_14: u64 = 1;
        // D s_6_15: and s_6_13 s_6_14
        let s_6_15: u64 = ((s_6_13) & (s_6_14));
        // D s_6_16: cmp-eq s_6_15 s_6_14
        let s_6_16: bool = ((s_6_15) == (s_6_14));
        // D s_6_17: lsl s_6_13 s_6_11
        let s_6_17: u64 = s_6_13 << s_6_11;
        // D s_6_18: or s_6_12 s_6_17
        let s_6_18: u64 = ((s_6_12) | (s_6_17));
        // D s_6_19: cmpl s_6_17
        let s_6_19: u64 = !s_6_17;
        // D s_6_20: and s_6_12 s_6_19
        let s_6_20: u64 = ((s_6_12) & (s_6_19));
        // D s_6_21: select s_6_16 s_6_18 s_6_20
        let s_6_21: u64 = if s_6_16 { s_6_18 } else { s_6_20 };
        // D s_6_22: cast trunc s_6_21 -> u8
        let s_6_22: bool = ((s_6_21) != 0);
        // D s_6_23: write-var desc_dbm <= s_6_22
        fn_state.desc_dbm = s_6_22;
        // N s_6_24: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_7_0: read-var walkparams.7:struct
        let s_7_0: bool = fn_state.walkparams._7;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b28 b8
        if s_7_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#19179 <= s_8_0
        fn_state.gs_19179 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_9_0: read-var gs#19179:u8
        let s_9_0: bool = fn_state.gs_19179;
        // N s_9_1: branch s_9_0 b27 b10
        if s_9_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#19180 <= s_10_0
        fn_state.gs_19180 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_11_0: read-var gs#19180:u8
        let s_11_0: bool = fn_state.gs_19180;
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_14_0: read-var walkparams.17:struct
        let s_14_0: bool = fn_state.walkparams._17;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // N s_14_5: branch s_14_4 b25 b15
        if s_14_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#19144 <= s_15_0
        fn_state.gs_19144 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_16_0: read-var gs#19144:u8
        let s_16_0: bool = fn_state.gs_19144;
        // N s_16_1: branch s_16_0 b24 b17
        if s_16_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#19145 <= s_17_0
        fn_state.gs_19145 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_18_0: read-var gs#19145:u8
        let s_18_0: bool = fn_state.gs_19145;
        // N s_18_1: branch s_18_0 b21 b19
        if s_18_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_20_0: read-var permissions:struct
        let s_20_0: ProductTypebf05c51f33174538 = fn_state.permissions;
        // N s_20_1: return s_20_0
        return s_20_0;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_21_0: read-var walkparams.2:struct
        let s_21_0: bool = fn_state.walkparams._2;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // N s_21_5: branch s_21_4 b23 b22
        if s_21_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_22_0: read-var descriptor:bv
        let s_22_0: Bits = fn_state.descriptor;
        // D s_22_1: size-of s_22_0
        let s_22_1: u16 = s_22_0.length();
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // C s_22_3: const #62s : i
        let s_22_3: i128 = 62;
        // D s_22_4: cmp-lt s_22_3 s_22_2
        let s_22_4: bool = ((s_22_3) < (s_22_2));
        // N s_22_5: assert s_22_4
        let s_22_5: () = assert!(s_22_4);
        // C s_22_6: const #59s : i
        let s_22_6: i128 = 59;
        // D s_22_7: read-var descriptor:bv
        let s_22_7: Bits = fn_state.descriptor;
        // C s_22_8: const #1s : i64
        let s_22_8: i64 = 1;
        // C s_22_9: cast zx s_22_8 -> i
        let s_22_9: i128 = (i128::try_from(s_22_8).unwrap());
        // C s_22_10: const #3s : i
        let s_22_10: i128 = 3;
        // C s_22_11: add s_22_10 s_22_9
        let s_22_11: i128 = (s_22_10 + s_22_9);
        // D s_22_12: bit-extract s_22_7 s_22_6 s_22_11
        let s_22_12: Bits = (Bits::new(
            ((s_22_7) >> (s_22_6)).value(),
            u16::try_from(s_22_11).unwrap(),
        ));
        // D s_22_13: cast reint s_22_12 -> u8
        let s_22_13: u8 = (s_22_12.value() as u8);
        // D s_22_14: write-var permissions.10 <= s_22_13
        fn_state.permissions._10 = s_22_13;
        // N s_22_15: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_23_0: read-var descriptor:bv
        let s_23_0: Bits = fn_state.descriptor;
        // D s_23_1: size-of s_23_0
        let s_23_1: u16 = s_23_0.length();
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // C s_23_3: const #124s : i
        let s_23_3: i128 = 124;
        // D s_23_4: cmp-lt s_23_3 s_23_2
        let s_23_4: bool = ((s_23_3) < (s_23_2));
        // N s_23_5: assert s_23_4
        let s_23_5: () = assert!(s_23_4);
        // C s_23_6: const #121s : i
        let s_23_6: i128 = 121;
        // D s_23_7: read-var descriptor:bv
        let s_23_7: Bits = fn_state.descriptor;
        // C s_23_8: const #1s : i64
        let s_23_8: i64 = 1;
        // C s_23_9: cast zx s_23_8 -> i
        let s_23_9: i128 = (i128::try_from(s_23_8).unwrap());
        // C s_23_10: const #3s : i
        let s_23_10: i128 = 3;
        // C s_23_11: add s_23_10 s_23_9
        let s_23_11: i128 = (s_23_10 + s_23_9);
        // D s_23_12: bit-extract s_23_7 s_23_6 s_23_11
        let s_23_12: Bits = (Bits::new(
            ((s_23_7) >> (s_23_6)).value(),
            u16::try_from(s_23_11).unwrap(),
        ));
        // D s_23_13: cast reint s_23_12 -> u8
        let s_23_13: u8 = (s_23_12.value() as u8);
        // D s_23_14: write-var permissions.10 <= s_23_13
        fn_state.permissions._10 = s_23_13;
        // N s_23_15: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_24_0: const #15328u : u32
        let s_24_0: u32 = 15328;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_VTCR_EL2_Type_S2POE(s_24_1)
        let s_24_2: bool = u_get_VTCR_EL2_Type_S2POE(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#19145 <= s_24_6
        fn_state.gs_19145 = s_24_6;
        // N s_24_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveS2POExt(s_25_0)
        let s_25_1: bool = HaveS2POExt(state, tracer, s_25_0);
        // D s_25_2: write-var gs#19144 <= s_25_1
        fn_state.gs_19144 = s_25_1;
        // N s_25_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // S s_26_1: call Bit(s_26_0)
        let s_26_1: bool = Bit(state, tracer, s_26_0);
        // D s_26_2: read-var permissions:struct
        let s_26_2: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_26_3: write-var permissions <= s_26_2
        fn_state.permissions = s_26_2;
        // N s_26_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_27_0: read-var desc_dbm:u8
        let s_27_0: bool = fn_state.desc_dbm;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#19180 <= s_27_4
        fn_state.gs_19180 = s_27_4;
        // N s_27_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_28_0: read-var walkparams.9:struct
        let s_28_0: bool = fn_state.walkparams._9;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#19179 <= s_28_4
        fn_state.gs_19179 = s_28_4;
        // N s_28_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_29_0: read-var descriptor:bv
        let s_29_0: Bits = fn_state.descriptor;
        // D s_29_1: size-of s_29_0
        let s_29_1: u16 = s_29_0.length();
        // D s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (i128::try_from(s_29_1).unwrap());
        // C s_29_3: const #115s : i
        let s_29_3: i128 = 115;
        // D s_29_4: cmp-lt s_29_3 s_29_2
        let s_29_4: bool = ((s_29_3) < (s_29_2));
        // N s_29_5: assert s_29_4
        let s_29_5: () = assert!(s_29_4);
        // C s_29_6: const #115s : i
        let s_29_6: i128 = 115;
        // D s_29_7: read-var descriptor:bv
        let s_29_7: Bits = fn_state.descriptor;
        // C s_29_8: const #1u : u64
        let s_29_8: u64 = 1;
        // D s_29_9: bit-extract s_29_7 s_29_6 s_29_8
        let s_29_9: Bits = (Bits::new(
            ((s_29_7) >> (s_29_6)).value(),
            u16::try_from(s_29_8).unwrap(),
        ));
        // D s_29_10: cast reint s_29_9 -> u8
        let s_29_10: bool = ((s_29_9.value()) != 0);
        // C s_29_11: const #0s : i
        let s_29_11: i128 = 0;
        // C s_29_12: const #0u : u64
        let s_29_12: u64 = 0;
        // D s_29_13: cast zx s_29_10 -> u64
        let s_29_13: u64 = (s_29_10 as u64);
        // C s_29_14: const #1u : u64
        let s_29_14: u64 = 1;
        // D s_29_15: and s_29_13 s_29_14
        let s_29_15: u64 = ((s_29_13) & (s_29_14));
        // D s_29_16: cmp-eq s_29_15 s_29_14
        let s_29_16: bool = ((s_29_15) == (s_29_14));
        // D s_29_17: lsl s_29_13 s_29_11
        let s_29_17: u64 = s_29_13 << s_29_11;
        // D s_29_18: or s_29_12 s_29_17
        let s_29_18: u64 = ((s_29_12) | (s_29_17));
        // D s_29_19: cmpl s_29_17
        let s_29_19: u64 = !s_29_17;
        // D s_29_20: and s_29_12 s_29_19
        let s_29_20: u64 = ((s_29_12) & (s_29_19));
        // D s_29_21: select s_29_16 s_29_18 s_29_20
        let s_29_21: u64 = if s_29_16 { s_29_18 } else { s_29_20 };
        // D s_29_22: cast trunc s_29_21 -> u8
        let s_29_22: bool = ((s_29_21) != 0);
        // D s_29_23: write-var desc_dbm <= s_29_22
        fn_state.desc_dbm = s_29_22;
        // N s_29_24: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_30_0: read-var walkparams.2:struct
        let s_30_0: bool = fn_state.walkparams._2;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // N s_30_5: branch s_30_4 b32 b31
        if s_30_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_31_0: read-var descriptor:bv
        let s_31_0: Bits = fn_state.descriptor;
        // D s_31_1: size-of s_31_0
        let s_31_1: u16 = s_31_0.length();
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // C s_31_3: const #53s : i
        let s_31_3: i128 = 53;
        // D s_31_4: cmp-lt s_31_3 s_31_2
        let s_31_4: bool = ((s_31_3) < (s_31_2));
        // N s_31_5: assert s_31_4
        let s_31_5: () = assert!(s_31_4);
        // C s_31_6: const #53s : i
        let s_31_6: i128 = 53;
        // D s_31_7: read-var descriptor:bv
        let s_31_7: Bits = fn_state.descriptor;
        // C s_31_8: const #1u : u64
        let s_31_8: u64 = 1;
        // D s_31_9: bit-extract s_31_7 s_31_6 s_31_8
        let s_31_9: Bits = (Bits::new(
            ((s_31_7) >> (s_31_6)).value(),
            u16::try_from(s_31_8).unwrap(),
        ));
        // D s_31_10: cast reint s_31_9 -> u8
        let s_31_10: bool = ((s_31_9.value()) != 0);
        // C s_31_11: const #0s : i
        let s_31_11: i128 = 0;
        // C s_31_12: const #0u : u64
        let s_31_12: u64 = 0;
        // D s_31_13: cast zx s_31_10 -> u64
        let s_31_13: u64 = (s_31_10 as u64);
        // C s_31_14: const #1u : u64
        let s_31_14: u64 = 1;
        // D s_31_15: and s_31_13 s_31_14
        let s_31_15: u64 = ((s_31_13) & (s_31_14));
        // D s_31_16: cmp-eq s_31_15 s_31_14
        let s_31_16: bool = ((s_31_15) == (s_31_14));
        // D s_31_17: lsl s_31_13 s_31_11
        let s_31_17: u64 = s_31_13 << s_31_11;
        // D s_31_18: or s_31_12 s_31_17
        let s_31_18: u64 = ((s_31_12) | (s_31_17));
        // D s_31_19: cmpl s_31_17
        let s_31_19: u64 = !s_31_17;
        // D s_31_20: and s_31_12 s_31_19
        let s_31_20: u64 = ((s_31_12) & (s_31_19));
        // D s_31_21: select s_31_16 s_31_18 s_31_20
        let s_31_21: u64 = if s_31_16 { s_31_18 } else { s_31_20 };
        // D s_31_22: cast trunc s_31_21 -> u8
        let s_31_22: bool = ((s_31_21) != 0);
        // D s_31_23: write-var permissions.13 <= s_31_22
        fn_state.permissions._13 = s_31_22;
        // N s_31_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_32_0: read-var descriptor:bv
        let s_32_0: Bits = fn_state.descriptor;
        // D s_32_1: size-of s_32_0
        let s_32_1: u16 = s_32_0.length();
        // D s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (i128::try_from(s_32_1).unwrap());
        // C s_32_3: const #117s : i
        let s_32_3: i128 = 117;
        // D s_32_4: cmp-lt s_32_3 s_32_2
        let s_32_4: bool = ((s_32_3) < (s_32_2));
        // N s_32_5: assert s_32_4
        let s_32_5: () = assert!(s_32_4);
        // C s_32_6: const #117s : i
        let s_32_6: i128 = 117;
        // D s_32_7: read-var descriptor:bv
        let s_32_7: Bits = fn_state.descriptor;
        // C s_32_8: const #1u : u64
        let s_32_8: u64 = 1;
        // D s_32_9: bit-extract s_32_7 s_32_6 s_32_8
        let s_32_9: Bits = (Bits::new(
            ((s_32_7) >> (s_32_6)).value(),
            u16::try_from(s_32_8).unwrap(),
        ));
        // D s_32_10: cast reint s_32_9 -> u8
        let s_32_10: bool = ((s_32_9.value()) != 0);
        // C s_32_11: const #0s : i
        let s_32_11: i128 = 0;
        // C s_32_12: const #0u : u64
        let s_32_12: u64 = 0;
        // D s_32_13: cast zx s_32_10 -> u64
        let s_32_13: u64 = (s_32_10 as u64);
        // C s_32_14: const #1u : u64
        let s_32_14: u64 = 1;
        // D s_32_15: and s_32_13 s_32_14
        let s_32_15: u64 = ((s_32_13) & (s_32_14));
        // D s_32_16: cmp-eq s_32_15 s_32_14
        let s_32_16: bool = ((s_32_15) == (s_32_14));
        // D s_32_17: lsl s_32_13 s_32_11
        let s_32_17: u64 = s_32_13 << s_32_11;
        // D s_32_18: or s_32_12 s_32_17
        let s_32_18: u64 = ((s_32_12) | (s_32_17));
        // D s_32_19: cmpl s_32_17
        let s_32_19: u64 = !s_32_17;
        // D s_32_20: and s_32_12 s_32_19
        let s_32_20: u64 = ((s_32_12) & (s_32_19));
        // D s_32_21: select s_32_16 s_32_18 s_32_20
        let s_32_21: u64 = if s_32_16 { s_32_18 } else { s_32_20 };
        // D s_32_22: cast trunc s_32_21 -> u8
        let s_32_22: bool = ((s_32_21) != 0);
        // D s_32_23: write-var permissions.13 <= s_32_22
        fn_state.permissions._13 = s_32_22;
        // N s_32_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_33_0: read-var descriptor:bv
        let s_33_0: Bits = fn_state.descriptor;
        // D s_33_1: size-of s_33_0
        let s_33_1: u16 = s_33_0.length();
        // D s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (i128::try_from(s_33_1).unwrap());
        // C s_33_3: const #118s : i
        let s_33_3: i128 = 118;
        // D s_33_4: cmp-lt s_33_3 s_33_2
        let s_33_4: bool = ((s_33_3) < (s_33_2));
        // N s_33_5: assert s_33_4
        let s_33_5: () = assert!(s_33_4);
        // C s_33_6: const #118s : i
        let s_33_6: i128 = 118;
        // D s_33_7: read-var descriptor:bv
        let s_33_7: Bits = fn_state.descriptor;
        // C s_33_8: const #1u : u64
        let s_33_8: u64 = 1;
        // D s_33_9: bit-extract s_33_7 s_33_6 s_33_8
        let s_33_9: Bits = (Bits::new(
            ((s_33_7) >> (s_33_6)).value(),
            u16::try_from(s_33_8).unwrap(),
        ));
        // D s_33_10: cast reint s_33_9 -> u8
        let s_33_10: bool = ((s_33_9.value()) != 0);
        // C s_33_11: const #0s : i
        let s_33_11: i128 = 0;
        // C s_33_12: const #0u : u64
        let s_33_12: u64 = 0;
        // D s_33_13: cast zx s_33_10 -> u64
        let s_33_13: u64 = (s_33_10 as u64);
        // C s_33_14: const #1u : u64
        let s_33_14: u64 = 1;
        // D s_33_15: and s_33_13 s_33_14
        let s_33_15: u64 = ((s_33_13) & (s_33_14));
        // D s_33_16: cmp-eq s_33_15 s_33_14
        let s_33_16: bool = ((s_33_15) == (s_33_14));
        // D s_33_17: lsl s_33_13 s_33_11
        let s_33_17: u64 = s_33_13 << s_33_11;
        // D s_33_18: or s_33_12 s_33_17
        let s_33_18: u64 = ((s_33_12) | (s_33_17));
        // D s_33_19: cmpl s_33_17
        let s_33_19: u64 = !s_33_17;
        // D s_33_20: and s_33_12 s_33_19
        let s_33_20: u64 = ((s_33_12) & (s_33_19));
        // D s_33_21: select s_33_16 s_33_18 s_33_20
        let s_33_21: u64 = if s_33_16 { s_33_18 } else { s_33_20 };
        // D s_33_22: cast trunc s_33_21 -> u8
        let s_33_22: bool = ((s_33_21) != 0);
        // D s_33_23: write-var permissions.12 <= s_33_22
        fn_state.permissions._12 = s_33_22;
        // N s_33_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_34_0: read-var walkparams.2:struct
        let s_34_0: bool = fn_state.walkparams._2;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // N s_34_5: branch s_34_4 b37 b35
        if s_34_4 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_35_0: read-var descriptor:bv
        let s_35_0: Bits = fn_state.descriptor;
        // D s_35_1: size-of s_35_0
        let s_35_1: u16 = s_35_0.length();
        // D s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (i128::try_from(s_35_1).unwrap());
        // C s_35_3: const #54s : i
        let s_35_3: i128 = 54;
        // D s_35_4: cmp-lt s_35_3 s_35_2
        let s_35_4: bool = ((s_35_3) < (s_35_2));
        // N s_35_5: assert s_35_4
        let s_35_5: () = assert!(s_35_4);
        // C s_35_6: const #53s : i
        let s_35_6: i128 = 53;
        // D s_35_7: read-var descriptor:bv
        let s_35_7: Bits = fn_state.descriptor;
        // C s_35_8: const #1s : i64
        let s_35_8: i64 = 1;
        // C s_35_9: cast zx s_35_8 -> i
        let s_35_9: i128 = (i128::try_from(s_35_8).unwrap());
        // C s_35_10: const #1s : i
        let s_35_10: i128 = 1;
        // C s_35_11: add s_35_10 s_35_9
        let s_35_11: i128 = (s_35_10 + s_35_9);
        // D s_35_12: bit-extract s_35_7 s_35_6 s_35_11
        let s_35_12: Bits = (Bits::new(
            ((s_35_7) >> (s_35_6)).value(),
            u16::try_from(s_35_11).unwrap(),
        ));
        // D s_35_13: cast reint s_35_12 -> u8
        let s_35_13: u8 = (s_35_12.value() as u8);
        // C s_35_14: const #51s : i
        let s_35_14: i128 = 51;
        // D s_35_15: read-var descriptor:bv
        let s_35_15: Bits = fn_state.descriptor;
        // C s_35_16: const #1u : u64
        let s_35_16: u64 = 1;
        // D s_35_17: bit-extract s_35_15 s_35_14 s_35_16
        let s_35_17: Bits = (Bits::new(
            ((s_35_15) >> (s_35_14)).value(),
            u16::try_from(s_35_16).unwrap(),
        ));
        // D s_35_18: cast reint s_35_17 -> u8
        let s_35_18: bool = ((s_35_17.value()) != 0);
        // C s_35_19: const #0s : i
        let s_35_19: i128 = 0;
        // C s_35_20: const #0u : u64
        let s_35_20: u64 = 0;
        // D s_35_21: cast zx s_35_18 -> u64
        let s_35_21: u64 = (s_35_18 as u64);
        // C s_35_22: const #1u : u64
        let s_35_22: u64 = 1;
        // D s_35_23: and s_35_21 s_35_22
        let s_35_23: u64 = ((s_35_21) & (s_35_22));
        // D s_35_24: cmp-eq s_35_23 s_35_22
        let s_35_24: bool = ((s_35_23) == (s_35_22));
        // D s_35_25: lsl s_35_21 s_35_19
        let s_35_25: u64 = s_35_21 << s_35_19;
        // D s_35_26: or s_35_20 s_35_25
        let s_35_26: u64 = ((s_35_20) | (s_35_25));
        // D s_35_27: cmpl s_35_25
        let s_35_27: u64 = !s_35_25;
        // D s_35_28: and s_35_20 s_35_27
        let s_35_28: u64 = ((s_35_20) & (s_35_27));
        // D s_35_29: select s_35_24 s_35_26 s_35_28
        let s_35_29: u64 = if s_35_24 { s_35_26 } else { s_35_28 };
        // D s_35_30: cast trunc s_35_29 -> u8
        let s_35_30: bool = ((s_35_29) != 0);
        // C s_35_31: const #6s : i
        let s_35_31: i128 = 6;
        // D s_35_32: read-var descriptor:bv
        let s_35_32: Bits = fn_state.descriptor;
        // C s_35_33: const #1u : u64
        let s_35_33: u64 = 1;
        // D s_35_34: bit-extract s_35_32 s_35_31 s_35_33
        let s_35_34: Bits = (Bits::new(
            ((s_35_32) >> (s_35_31)).value(),
            u16::try_from(s_35_33).unwrap(),
        ));
        // D s_35_35: cast reint s_35_34 -> u8
        let s_35_35: bool = ((s_35_34.value()) != 0);
        // C s_35_36: const #0s : i
        let s_35_36: i128 = 0;
        // C s_35_37: const #0u : u64
        let s_35_37: u64 = 0;
        // D s_35_38: cast zx s_35_35 -> u64
        let s_35_38: u64 = (s_35_35 as u64);
        // C s_35_39: const #1u : u64
        let s_35_39: u64 = 1;
        // D s_35_40: and s_35_38 s_35_39
        let s_35_40: u64 = ((s_35_38) & (s_35_39));
        // D s_35_41: cmp-eq s_35_40 s_35_39
        let s_35_41: bool = ((s_35_40) == (s_35_39));
        // D s_35_42: lsl s_35_38 s_35_36
        let s_35_42: u64 = s_35_38 << s_35_36;
        // D s_35_43: or s_35_37 s_35_42
        let s_35_43: u64 = ((s_35_37) | (s_35_42));
        // D s_35_44: cmpl s_35_42
        let s_35_44: u64 = !s_35_42;
        // D s_35_45: and s_35_37 s_35_44
        let s_35_45: u64 = ((s_35_37) & (s_35_44));
        // D s_35_46: select s_35_41 s_35_43 s_35_45
        let s_35_46: u64 = if s_35_41 { s_35_43 } else { s_35_45 };
        // D s_35_47: cast trunc s_35_46 -> u8
        let s_35_47: bool = ((s_35_46) != 0);
        // D s_35_48: cast zx s_35_30 -> bv
        let s_35_48: Bits = Bits::new(s_35_30 as u128, 1u16);
        // D s_35_49: cast zx s_35_47 -> bv
        let s_35_49: Bits = Bits::new(s_35_47 as u128, 1u16);
        // D s_35_50: cast reint s_35_48 -> u128
        let s_35_50: u128 = (s_35_48.value() as u128);
        // D s_35_51: size-of s_35_48
        let s_35_51: u16 = s_35_48.length();
        // D s_35_52: cast reint s_35_49 -> u128
        let s_35_52: u128 = (s_35_49.value() as u128);
        // D s_35_53: size-of s_35_49
        let s_35_53: u16 = s_35_49.length();
        // D s_35_54: lsl s_35_50 s_35_53
        let s_35_54: u128 = s_35_50 << s_35_53;
        // D s_35_55: or s_35_54 s_35_52
        let s_35_55: u128 = ((s_35_54) | (s_35_52));
        // D s_35_56: add s_35_51 s_35_53
        let s_35_56: u16 = (s_35_51 + s_35_53);
        // D s_35_57: create-bits s_35_55 s_35_56
        let s_35_57: Bits = Bits::new(s_35_55, s_35_56);
        // D s_35_58: cast reint s_35_57 -> u8
        let s_35_58: u8 = (s_35_57.value() as u8);
        // D s_35_59: cast zx s_35_13 -> bv
        let s_35_59: Bits = Bits::new(s_35_13 as u128, 2u16);
        // D s_35_60: cast zx s_35_58 -> bv
        let s_35_60: Bits = Bits::new(s_35_58 as u128, 2u16);
        // D s_35_61: cast reint s_35_59 -> u128
        let s_35_61: u128 = (s_35_59.value() as u128);
        // D s_35_62: size-of s_35_59
        let s_35_62: u16 = s_35_59.length();
        // D s_35_63: cast reint s_35_60 -> u128
        let s_35_63: u128 = (s_35_60.value() as u128);
        // D s_35_64: size-of s_35_60
        let s_35_64: u16 = s_35_60.length();
        // D s_35_65: lsl s_35_61 s_35_64
        let s_35_65: u128 = s_35_61 << s_35_64;
        // D s_35_66: or s_35_65 s_35_63
        let s_35_66: u128 = ((s_35_65) | (s_35_63));
        // D s_35_67: add s_35_62 s_35_64
        let s_35_67: u16 = (s_35_62 + s_35_64);
        // D s_35_68: create-bits s_35_66 s_35_67
        let s_35_68: Bits = Bits::new(s_35_66, s_35_67);
        // D s_35_69: cast reint s_35_68 -> u8
        let s_35_69: u8 = (s_35_68.value() as u8);
        // D s_35_70: write-var s2pi_index <= s_35_69
        fn_state.s2pi_index = s_35_69;
        // N s_35_71: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_36_0: read-var s2pi_index:u8
        let s_36_0: u8 = fn_state.s2pi_index;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 4u16);
        // D s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (s_36_1.value() as i128);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #4s : i
        let s_36_4: i128 = 4;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: mul s_36_4 s_36_5
        let s_36_6: i128 = ((s_36_4) * (s_36_5));
        // D s_36_7: cast reint s_36_6 -> i64
        let s_36_7: i64 = (s_36_6 as i64);
        // D s_36_8: read-var walkparams.18:struct
        let s_36_8: ProductType5c790c8ef59cc8b2 = fn_state.walkparams._18;
        // D s_36_9: write-var ga#14336 <= s_36_8
        fn_state.ga_14336 = s_36_8;
        // D s_36_10: read-var ga#14336.0:struct
        let s_36_10: u64 = fn_state.ga_14336._0;
        // C s_36_11: const #4s : i
        let s_36_11: i128 = 4;
        // D s_36_12: cast zx s_36_10 -> bv
        let s_36_12: Bits = Bits::new(s_36_10 as u128, 64u16);
        // D s_36_13: cast zx s_36_7 -> i
        let s_36_13: i128 = (i128::try_from(s_36_7).unwrap());
        // D s_36_14: bit-extract s_36_12 s_36_13 s_36_11
        let s_36_14: Bits = (Bits::new(
            ((s_36_12) >> (s_36_13)).value(),
            u16::try_from(s_36_11).unwrap(),
        ));
        // D s_36_15: cast reint s_36_14 -> u8
        let s_36_15: u8 = (s_36_14.value() as u8);
        // D s_36_16: write-var permissions.9 <= s_36_15
        fn_state.permissions._9 = s_36_15;
        // D s_36_17: read-var descriptor:bv
        let s_36_17: Bits = fn_state.descriptor;
        // D s_36_18: size-of s_36_17
        let s_36_18: u16 = s_36_17.length();
        // D s_36_19: cast zx s_36_18 -> i
        let s_36_19: i128 = (i128::try_from(s_36_18).unwrap());
        // C s_36_20: const #7s : i
        let s_36_20: i128 = 7;
        // D s_36_21: cmp-lt s_36_20 s_36_19
        let s_36_21: bool = ((s_36_20) < (s_36_19));
        // N s_36_22: assert s_36_21
        let s_36_22: () = assert!(s_36_21);
        // C s_36_23: const #7s : i
        let s_36_23: i128 = 7;
        // D s_36_24: read-var descriptor:bv
        let s_36_24: Bits = fn_state.descriptor;
        // C s_36_25: const #1u : u64
        let s_36_25: u64 = 1;
        // D s_36_26: bit-extract s_36_24 s_36_23 s_36_25
        let s_36_26: Bits = (Bits::new(
            ((s_36_24) >> (s_36_23)).value(),
            u16::try_from(s_36_25).unwrap(),
        ));
        // D s_36_27: cast reint s_36_26 -> u8
        let s_36_27: bool = ((s_36_26.value()) != 0);
        // C s_36_28: const #0s : i
        let s_36_28: i128 = 0;
        // C s_36_29: const #0u : u64
        let s_36_29: u64 = 0;
        // D s_36_30: cast zx s_36_27 -> u64
        let s_36_30: u64 = (s_36_27 as u64);
        // C s_36_31: const #1u : u64
        let s_36_31: u64 = 1;
        // D s_36_32: and s_36_30 s_36_31
        let s_36_32: u64 = ((s_36_30) & (s_36_31));
        // D s_36_33: cmp-eq s_36_32 s_36_31
        let s_36_33: bool = ((s_36_32) == (s_36_31));
        // D s_36_34: lsl s_36_30 s_36_28
        let s_36_34: u64 = s_36_30 << s_36_28;
        // D s_36_35: or s_36_29 s_36_34
        let s_36_35: u64 = ((s_36_29) | (s_36_34));
        // D s_36_36: cmpl s_36_34
        let s_36_36: u64 = !s_36_34;
        // D s_36_37: and s_36_29 s_36_36
        let s_36_37: u64 = ((s_36_29) & (s_36_36));
        // D s_36_38: select s_36_33 s_36_35 s_36_37
        let s_36_38: u64 = if s_36_33 { s_36_35 } else { s_36_37 };
        // D s_36_39: cast trunc s_36_38 -> u8
        let s_36_39: bool = ((s_36_38) != 0);
        // D s_36_40: write-var permissions.8 <= s_36_39
        fn_state.permissions._8 = s_36_39;
        // N s_36_41: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_37_0: read-var descriptor:bv
        let s_37_0: Bits = fn_state.descriptor;
        // D s_37_1: size-of s_37_0
        let s_37_1: u16 = s_37_0.length();
        // D s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (i128::try_from(s_37_1).unwrap());
        // C s_37_3: const #118s : i
        let s_37_3: i128 = 118;
        // D s_37_4: cmp-lt s_37_3 s_37_2
        let s_37_4: bool = ((s_37_3) < (s_37_2));
        // N s_37_5: assert s_37_4
        let s_37_5: () = assert!(s_37_4);
        // C s_37_6: const #115s : i
        let s_37_6: i128 = 115;
        // D s_37_7: read-var descriptor:bv
        let s_37_7: Bits = fn_state.descriptor;
        // C s_37_8: const #1s : i64
        let s_37_8: i64 = 1;
        // C s_37_9: cast zx s_37_8 -> i
        let s_37_9: i128 = (i128::try_from(s_37_8).unwrap());
        // C s_37_10: const #3s : i
        let s_37_10: i128 = 3;
        // C s_37_11: add s_37_10 s_37_9
        let s_37_11: i128 = (s_37_10 + s_37_9);
        // D s_37_12: bit-extract s_37_7 s_37_6 s_37_11
        let s_37_12: Bits = (Bits::new(
            ((s_37_7) >> (s_37_6)).value(),
            u16::try_from(s_37_11).unwrap(),
        ));
        // D s_37_13: cast reint s_37_12 -> u8
        let s_37_13: u8 = (s_37_12.value() as u8);
        // D s_37_14: write-var s2pi_index <= s_37_13
        fn_state.s2pi_index = s_37_13;
        // N s_37_15: jump b36
        return block_36(state, tracer, fn_state);
    }
}
