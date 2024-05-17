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
use AArch64_BlockDescSupported::*;
use common::*;
pub fn AArch64_DecodeDescriptorType<T: Tracer>(
    state: &mut State,
    tracer: &T,
    descriptor: Bits,
    d128: bool,
    ds: bool,
    tgx: u32,
    level: i128,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_17580: bool,
        return_value: u32,
        skl: u8,
        gs_17578: bool,
        effective_level: i128,
        ga_13128: u32,
        descriptor: Bits,
        d128: bool,
        ds: bool,
        tgx: u32,
        level: i128,
    }
    let fn_state = FunctionState {
        descriptor,
        d128,
        ds,
        tgx,
        level,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var descriptor:bv
        let s_0_1: Bits = fn_state.descriptor;
        // C s_0_2: const #1u : u64
        let s_0_2: u64 = 1;
        // D s_0_3: bit-extract s_0_1 s_0_0 s_0_2
        let s_0_3: Bits = (Bits::new(
            ((s_0_1) >> (s_0_0)).value(),
            u16::try_from(s_0_2).unwrap(),
        ));
        // D s_0_4: cast reint s_0_3 -> u8
        let s_0_4: bool = ((s_0_3.value()) != 0);
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // C s_0_6: const #0u : u64
        let s_0_6: u64 = 0;
        // D s_0_7: cast zx s_0_4 -> u64
        let s_0_7: u64 = (s_0_4 as u64);
        // C s_0_8: const #1u : u64
        let s_0_8: u64 = 1;
        // D s_0_9: and s_0_7 s_0_8
        let s_0_9: u64 = ((s_0_7) & (s_0_8));
        // D s_0_10: cmp-eq s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) == (s_0_8));
        // D s_0_11: lsl s_0_7 s_0_5
        let s_0_11: u64 = s_0_7 << s_0_5;
        // D s_0_12: or s_0_6 s_0_11
        let s_0_12: u64 = ((s_0_6) | (s_0_11));
        // D s_0_13: cmpl s_0_11
        let s_0_13: u64 = !s_0_11;
        // D s_0_14: and s_0_6 s_0_13
        let s_0_14: u64 = ((s_0_6) & (s_0_13));
        // D s_0_15: select s_0_10 s_0_12 s_0_14
        let s_0_15: u64 = if s_0_10 { s_0_12 } else { s_0_14 };
        // D s_0_16: cast trunc s_0_15 -> u8
        let s_0_16: bool = ((s_0_15) != 0);
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // C s_0_18: const #0u : u8
        let s_0_18: bool = false;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // D s_0_20: cmp-eq s_0_17 s_0_19
        let s_0_20: bool = ((s_0_17) == (s_0_19));
        // N s_0_21: branch s_0_20 b25 b1
        if s_0_20 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_1_0: read-var d128:u8
        let s_1_0: bool = fn_state.d128;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b12 b2
        if s_1_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var descriptor:bv
        let s_2_0: Bits = fn_state.descriptor;
        // D s_2_1: size-of s_2_0
        let s_2_1: u16 = s_2_0.length();
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // C s_2_3: const #1s : i
        let s_2_3: i128 = 1;
        // D s_2_4: cmp-lt s_2_3 s_2_2
        let s_2_4: bool = ((s_2_3) < (s_2_2));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // C s_2_6: const #1s : i
        let s_2_6: i128 = 1;
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
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 1u16);
        // C s_2_24: const #1u : u8
        let s_2_24: bool = true;
        // C s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 1u16);
        // D s_2_26: cmp-eq s_2_23 s_2_25
        let s_2_26: bool = ((s_2_23) == (s_2_25));
        // N s_2_27: branch s_2_26 b9 b3
        if s_2_26 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
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
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 1u16);
        // C s_3_18: const #0u : u8
        let s_3_18: bool = false;
        // C s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 1u16);
        // D s_3_20: cmp-eq s_3_17 s_3_19
        let s_3_20: bool = ((s_3_17) == (s_3_19));
        // N s_3_21: branch s_3_20 b6 b4
        if s_3_20 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var ga#13128:u32
        let s_4_0: u32 = fn_state.ga_13128;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var return_value:u32
        let s_5_0: u32 = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var d128:u8
        let s_6_0: bool = fn_state.d128;
        // D s_6_1: read-var ds:u8
        let s_6_1: bool = fn_state.ds;
        // D s_6_2: read-var tgx:u32
        let s_6_2: u32 = fn_state.tgx;
        // D s_6_3: read-var level:i
        let s_6_3: i128 = fn_state.level;
        // D s_6_4: call AArch64_BlockDescSupported(s_6_0, s_6_1, s_6_2, s_6_3)
        let s_6_4: bool = AArch64_BlockDescSupported(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
        );
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #2u : u32
        let s_7_0: u32 = 2;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #1u : u32
        let s_8_0: u32 = 1;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #800u : u32
        let s_9_0: u32 = 800;
        // D s_9_1: read-reg s_9_0:i64
        let s_9_1: i64 = {
            let value = state.read_register::<i64>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: read-var level:i
        let s_9_3: i128 = fn_state.level;
        // D s_9_4: cmp-eq s_9_3 s_9_2
        let s_9_4: bool = ((s_9_3) == (s_9_2));
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_10_0: const #0u : u32
        let s_10_0: u32 = 0;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #1u : u32
        let s_11_0: u32 = 1;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_12_0: read-var descriptor:bv
        let s_12_0: Bits = fn_state.descriptor;
        // D s_12_1: size-of s_12_0
        let s_12_1: u16 = s_12_0.length();
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #110s : i
        let s_12_3: i128 = 110;
        // D s_12_4: cmp-lt s_12_3 s_12_2
        let s_12_4: bool = ((s_12_3) < (s_12_2));
        // N s_12_5: assert s_12_4
        let s_12_5: () = assert!(s_12_4);
        // C s_12_6: const #109s : i
        let s_12_6: i128 = 109;
        // D s_12_7: read-var descriptor:bv
        let s_12_7: Bits = fn_state.descriptor;
        // C s_12_8: const #1s : i64
        let s_12_8: i64 = 1;
        // C s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // C s_12_10: const #1s : i
        let s_12_10: i128 = 1;
        // C s_12_11: add s_12_10 s_12_9
        let s_12_11: i128 = (s_12_10 + s_12_9);
        // D s_12_12: bit-extract s_12_7 s_12_6 s_12_11
        let s_12_12: Bits = (Bits::new(
            ((s_12_7) >> (s_12_6)).value(),
            u16::try_from(s_12_11).unwrap(),
        ));
        // D s_12_13: cast reint s_12_12 -> u8
        let s_12_13: u8 = (s_12_12.value() as u8);
        // D s_12_14: write-var skl <= s_12_13
        fn_state.skl = s_12_13;
        // D s_12_15: read-var tgx:u32
        let s_12_15: u32 = fn_state.tgx;
        // C s_12_16: const #1u : u32
        let s_12_16: u32 = 1;
        // D s_12_17: cmp-eq s_12_15 s_12_16
        let s_12_17: bool = ((s_12_15) == (s_12_16));
        // N s_12_18: branch s_12_17 b24 b13
        if s_12_17 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_13_0: read-var tgx:u32
        let s_13_0: u32 = fn_state.tgx;
        // C s_13_1: const #2u : u32
        let s_13_1: u32 = 2;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#17578 <= s_13_2
        fn_state.gs_17578 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_14_0: read-var gs#17578:u8
        let s_14_0: bool = fn_state.gs_17578;
        // N s_14_1: branch s_14_0 b23 b15
        if s_14_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#17580 <= s_15_0
        fn_state.gs_17580 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_16_0: read-var gs#17580:u8
        let s_16_0: bool = fn_state.gs_17580;
        // N s_16_1: branch s_16_0 b22 b17
        if s_16_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_17_0: read-var skl:u8
        let s_17_0: u8 = fn_state.skl;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: read-var level:i
        let s_17_5: i128 = fn_state.level;
        // D s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: write-var effective_level <= s_17_6
        fn_state.effective_level = s_17_6;
        // C s_17_8: const #800u : u32
        let s_17_8: u32 = 800;
        // D s_17_9: read-reg s_17_8:i64
        let s_17_9: i64 = {
            let value = state.read_register::<i64>(s_17_8 as isize);
            tracer.read_register(s_17_8 as isize, value);
            value
        };
        // D s_17_10: cast zx s_17_9 -> i
        let s_17_10: i128 = (i128::try_from(s_17_9).unwrap());
        // D s_17_11: read-var effective_level:i
        let s_17_11: i128 = fn_state.effective_level;
        // D s_17_12: cmp-gt s_17_11 s_17_10
        let s_17_12: bool = ((s_17_11) > (s_17_10));
        // N s_17_13: branch s_17_12 b21 b18
        if s_17_12 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #800u : u32
        let s_18_0: u32 = 800;
        // D s_18_1: read-reg s_18_0:i64
        let s_18_1: i64 = {
            let value = state.read_register::<i64>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: read-var effective_level:i
        let s_18_3: i128 = fn_state.effective_level;
        // D s_18_4: cmp-eq s_18_3 s_18_2
        let s_18_4: bool = ((s_18_3) == (s_18_2));
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #0u : u32
        let s_19_0: u32 = 0;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #1u : u32
        let s_20_0: u32 = 1;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_21_0: const #2u : u32
        let s_21_0: u32 = 2;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_22_0: const #2u : u32
        let s_22_0: u32 = 2;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_23_0: read-var skl:u8
        let s_23_0: u8 = fn_state.skl;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 2u16);
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #3s : i
        let s_23_4: i128 = 3;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-eq s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) == (s_23_4));
        // D s_23_7: write-var gs#17580 <= s_23_6
        fn_state.gs_17580 = s_23_6;
        // N s_23_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#17578 <= s_24_0
        fn_state.gs_17578 = s_24_0;
        // N s_24_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_25_0: const #2u : u32
        let s_25_0: u32 = 2;
        // D s_25_1: write-var return_value <= s_25_0
        fn_state.return_value = s_25_0;
        // N s_25_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
