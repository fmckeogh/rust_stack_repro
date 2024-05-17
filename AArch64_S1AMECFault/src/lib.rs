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
use common::*;
pub fn AArch64_S1AMECFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeef284266e139aee2,
    paspace: u32,
    regime: u32,
    descriptor: Bits,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_17670: bool,
        gs_17680: bool,
        gs_17681: bool,
        gs_17678: bool,
        gs_17679: bool,
        descriptor_amec: bool,
        walkparams: ProductTypeef284266e139aee2,
        paspace: u32,
        regime: u32,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        walkparams,
        paspace,
        regime,
        descriptor,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var descriptor:bv
        let s_0_0: Bits = fn_state.descriptor;
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #64s : i
        let s_0_4: i128 = 64;
        // D s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_6: cmp-eq s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) == (s_0_4));
        // N s_0_7: branch s_0_6 b18 b1
        if s_0_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var descriptor:bv
        let s_1_0: Bits = fn_state.descriptor;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #128s : i
        let s_1_4: i128 = 128;
        // D s_1_5: cast zx s_1_3 -> i
        let s_1_5: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_6: cmp-eq s_1_5 s_1_4
        let s_1_6: bool = ((s_1_5) == (s_1_4));
        // D s_1_7: write-var gs#17670 <= s_1_6
        fn_state.gs_17670 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#17670:u8
        let s_2_0: bool = fn_state.gs_17670;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var walkparams.3:struct
        let s_2_2: bool = fn_state.walkparams._3;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b17 b3
        if s_2_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #63s : i
        let s_3_0: i128 = 63;
        // D s_3_1: read-var descriptor:bv
        let s_3_1: Bits = fn_state.descriptor;
        // C s_3_2: const #1u : u64
        let s_3_2: u64 = 1;
        // D s_3_3: bit-extract s_3_1 s_3_0 s_3_2
        let s_3_3: Bits = (Bits::new(
            ((s_3_1) >> (s_3_0)).value(),
            u16::try_from(s_3_2).unwrap(),
        ));
        // D s_3_4: cast reint s_3_3 -> u8
        let s_3_4: bool = ((s_3_3.value()) != 0);
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // C s_3_6: const #0u : u64
        let s_3_6: u64 = 0;
        // D s_3_7: cast zx s_3_4 -> u64
        let s_3_7: u64 = (s_3_4 as u64);
        // C s_3_8: const #1u : u64
        let s_3_8: u64 = 1;
        // D s_3_9: and s_3_7 s_3_8
        let s_3_9: u64 = ((s_3_7) & (s_3_8));
        // D s_3_10: cmp-eq s_3_9 s_3_8
        let s_3_10: bool = ((s_3_9) == (s_3_8));
        // D s_3_11: lsl s_3_7 s_3_5
        let s_3_11: u64 = s_3_7 << s_3_5;
        // D s_3_12: or s_3_6 s_3_11
        let s_3_12: u64 = ((s_3_6) | (s_3_11));
        // D s_3_13: cmpl s_3_11
        let s_3_13: u64 = !s_3_11;
        // D s_3_14: and s_3_6 s_3_13
        let s_3_14: u64 = ((s_3_6) & (s_3_13));
        // D s_3_15: select s_3_10 s_3_12 s_3_14
        let s_3_15: u64 = if s_3_10 { s_3_12 } else { s_3_14 };
        // D s_3_16: cast trunc s_3_15 -> u8
        let s_3_16: bool = ((s_3_15) != 0);
        // D s_3_17: write-var descriptor_amec <= s_3_16
        fn_state.descriptor_amec = s_3_16;
        // N s_3_18: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var walkparams.10:struct
        let s_4_0: bool = fn_state.walkparams._10;
        // D s_4_1: read-var walkparams.1:struct
        let s_4_1: bool = fn_state.walkparams._1;
        // D s_4_2: cast zx s_4_0 -> bv
        let s_4_2: Bits = Bits::new(s_4_0 as u128, 1u16);
        // D s_4_3: cast zx s_4_1 -> bv
        let s_4_3: Bits = Bits::new(s_4_1 as u128, 1u16);
        // D s_4_4: cast reint s_4_2 -> u128
        let s_4_4: u128 = (s_4_2.value() as u128);
        // D s_4_5: size-of s_4_2
        let s_4_5: u16 = s_4_2.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u8
        let s_4_12: u8 = (s_4_11.value() as u8);
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 2u16);
        // C s_4_14: const #2u : u8
        let s_4_14: u8 = 2;
        // C s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 2u16);
        // D s_4_16: cmp-eq s_4_13 s_4_15
        let s_4_16: bool = ((s_4_13) == (s_4_15));
        // N s_4_17: branch s_4_16 b13 b5
        if s_4_16 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#17679 <= s_5_0
        fn_state.gs_17679 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#17679:u8
        let s_6_0: bool = fn_state.gs_17679;
        // N s_6_1: branch s_6_0 b12 b7
        if s_6_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#17680 <= s_7_0
        fn_state.gs_17680 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#17680:u8
        let s_8_0: bool = fn_state.gs_17680;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#17681 <= s_9_0
        fn_state.gs_17681 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#17681:u8
        let s_10_0: bool = fn_state.gs_17681;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var descriptor_amec:u8
        let s_11_0: bool = fn_state.descriptor_amec;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: write-var gs#17681 <= s_11_4
        fn_state.gs_17681 = s_11_4;
        // N s_11_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var paspace:u32
        let s_12_0: u32 = fn_state.paspace;
        // C s_12_1: const #3u : u32
        let s_12_1: u32 = 3;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: write-var gs#17680 <= s_12_2
        fn_state.gs_17680 = s_12_2;
        // N s_12_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var regime:u32
        let s_13_0: u32 = fn_state.regime;
        // C s_13_1: const #2u : u32
        let s_13_1: u32 = 2;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var regime:u32
        let s_14_0: u32 = fn_state.regime;
        // C s_14_1: const #3u : u32
        let s_14_1: u32 = 3;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: write-var gs#17678 <= s_14_2
        fn_state.gs_17678 = s_14_2;
        // N s_14_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#17678:u8
        let s_15_0: bool = fn_state.gs_17678;
        // D s_15_1: write-var gs#17679 <= s_15_0
        fn_state.gs_17679 = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#17678 <= s_16_0
        fn_state.gs_17678 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var descriptor:bv
        let s_17_0: Bits = fn_state.descriptor;
        // D s_17_1: size-of s_17_0
        let s_17_1: u16 = s_17_0.length();
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: const #103s : i
        let s_17_4: i128 = 103;
        // D s_17_5: cast zx s_17_3 -> i
        let s_17_5: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_6: cmp-lt s_17_4 s_17_5
        let s_17_6: bool = ((s_17_4) < (s_17_5));
        // N s_17_7: assert s_17_6
        let s_17_7: () = assert!(s_17_6);
        // C s_17_8: const #103s : i
        let s_17_8: i128 = 103;
        // D s_17_9: read-var descriptor:bv
        let s_17_9: Bits = fn_state.descriptor;
        // C s_17_10: const #1u : u64
        let s_17_10: u64 = 1;
        // D s_17_11: bit-extract s_17_9 s_17_8 s_17_10
        let s_17_11: Bits = (Bits::new(
            ((s_17_9) >> (s_17_8)).value(),
            u16::try_from(s_17_10).unwrap(),
        ));
        // D s_17_12: cast reint s_17_11 -> u8
        let s_17_12: bool = ((s_17_11.value()) != 0);
        // C s_17_13: const #0s : i
        let s_17_13: i128 = 0;
        // C s_17_14: const #0u : u64
        let s_17_14: u64 = 0;
        // D s_17_15: cast zx s_17_12 -> u64
        let s_17_15: u64 = (s_17_12 as u64);
        // C s_17_16: const #1u : u64
        let s_17_16: u64 = 1;
        // D s_17_17: and s_17_15 s_17_16
        let s_17_17: u64 = ((s_17_15) & (s_17_16));
        // D s_17_18: cmp-eq s_17_17 s_17_16
        let s_17_18: bool = ((s_17_17) == (s_17_16));
        // D s_17_19: lsl s_17_15 s_17_13
        let s_17_19: u64 = s_17_15 << s_17_13;
        // D s_17_20: or s_17_14 s_17_19
        let s_17_20: u64 = ((s_17_14) | (s_17_19));
        // D s_17_21: cmpl s_17_19
        let s_17_21: u64 = !s_17_19;
        // D s_17_22: and s_17_14 s_17_21
        let s_17_22: u64 = ((s_17_14) & (s_17_21));
        // D s_17_23: select s_17_18 s_17_20 s_17_22
        let s_17_23: u64 = if s_17_18 { s_17_20 } else { s_17_22 };
        // D s_17_24: cast trunc s_17_23 -> u8
        let s_17_24: bool = ((s_17_23) != 0);
        // D s_17_25: write-var descriptor_amec <= s_17_24
        fn_state.descriptor_amec = s_17_24;
        // N s_17_26: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#17670 <= s_18_0
        fn_state.gs_17670 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
