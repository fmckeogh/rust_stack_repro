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
use Have56BitPAExt::*;
use Have52BitPAExt::*;
use Zeros::*;
use common::*;
pub fn AArch64_NextTableBase<T: Tracer>(
    state: &mut State,
    tracer: &T,
    descriptor: Bits,
    d128: bool,
    ds: bool,
    tgx: u32,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        tablebase: u64,
        return_value: u64,
        gs_18096: bool,
        gs_18097: bool,
        descriptor: Bits,
        d128: bool,
        ds: bool,
        tgx: u32,
    }
    let fn_state = FunctionState {
        descriptor,
        d128,
        ds,
        tgx,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #56s : i
        let s_0_0: i128 = 56;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u56
        let s_0_2: u64 = (s_0_1.value() as u64);
        // D s_0_3: write-var tablebase <= s_0_2
        fn_state.tablebase = s_0_2;
        // C s_0_4: const #0u : u32
        let s_0_4: u32 = 0;
        // D s_0_5: read-var tgx:u32
        let s_0_5: u32 = fn_state.tgx;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // D s_0_7: not s_0_6
        let s_0_7: bool = !s_0_6;
        // N s_0_8: branch s_0_7 b16 b1
        if s_0_7 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_1_0: read-var descriptor:bv
        let s_1_0: Bits = fn_state.descriptor;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // C s_1_3: const #47s : i
        let s_1_3: i128 = 47;
        // D s_1_4: cmp-lt s_1_3 s_1_2
        let s_1_4: bool = ((s_1_3) < (s_1_2));
        // N s_1_5: assert s_1_4
        let s_1_5: () = assert!(s_1_4);
        // C s_1_6: const #12s : i
        let s_1_6: i128 = 12;
        // D s_1_7: read-var descriptor:bv
        let s_1_7: Bits = fn_state.descriptor;
        // C s_1_8: const #1s : i64
        let s_1_8: i64 = 1;
        // C s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_10: const #35s : i
        let s_1_10: i128 = 35;
        // C s_1_11: add s_1_10 s_1_9
        let s_1_11: i128 = (s_1_10 + s_1_9);
        // D s_1_12: bit-extract s_1_7 s_1_6 s_1_11
        let s_1_12: Bits = (Bits::new(
            ((s_1_7) >> (s_1_6)).value(),
            u16::try_from(s_1_11).unwrap(),
        ));
        // D s_1_13: cast reint s_1_12 -> u36
        let s_1_13: u64 = (s_1_12.value() as u64);
        // C s_1_14: const #12s : i
        let s_1_14: i128 = 12;
        // D s_1_15: read-var tablebase:u56
        let s_1_15: u64 = fn_state.tablebase;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 56u16);
        // D s_1_17: cast zx s_1_13 -> bv
        let s_1_17: Bits = Bits::new(s_1_13 as u128, 36u16);
        // C s_1_18: const #35s : i
        let s_1_18: i128 = 35;
        // C s_1_19: const #1u : u64
        let s_1_19: u64 = 1;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 64u16);
        // C s_1_21: lsl s_1_20 s_1_18
        let s_1_21: Bits = s_1_20 << s_1_18;
        // C s_1_22: sub s_1_21 s_1_20
        let s_1_22: Bits = ((s_1_21) - (s_1_20));
        // D s_1_23: and s_1_17 s_1_22
        let s_1_23: Bits = ((s_1_17) & (s_1_22));
        // D s_1_24: lsl s_1_23 s_1_14
        let s_1_24: Bits = s_1_23 << s_1_14;
        // C s_1_25: lsl s_1_22 s_1_14
        let s_1_25: Bits = s_1_22 << s_1_14;
        // C s_1_26: cmpl s_1_25
        let s_1_26: Bits = !s_1_25;
        // D s_1_27: and s_1_16 s_1_26
        let s_1_27: Bits = ((s_1_16) & (s_1_26));
        // D s_1_28: or s_1_27 s_1_24
        let s_1_28: Bits = ((s_1_27) | (s_1_24));
        // D s_1_29: cast reint s_1_28 -> u56
        let s_1_29: u64 = (s_1_28.value() as u64);
        // D s_1_30: write-var tablebase <= s_1_29
        fn_state.tablebase = s_1_29;
        // N s_1_31: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call Have56BitPAExt(s_2_0)
        let s_2_1: bool = Have56BitPAExt(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b15 b3
        if s_2_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#18096 <= s_3_0
        fn_state.gs_18096 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var gs#18096:u8
        let s_4_0: bool = fn_state.gs_18096;
        // N s_4_1: branch s_4_0 b14 b5
        if s_4_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call Have52BitPAExt(s_5_0)
        let s_5_1: bool = Have52BitPAExt(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b13 b6
        if s_5_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#18097 <= s_6_0
        fn_state.gs_18097 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#18097:u8
        let s_7_0: bool = fn_state.gs_18097;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_8_0: read-var ds:u8
        let s_8_0: bool = fn_state.ds;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b11 b9
        if s_8_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var tablebase:u56
        let s_9_0: u64 = fn_state.tablebase;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_10_0: read-var return_value:u56
        let s_10_0: u64 = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var descriptor:bv
        let s_11_0: Bits = fn_state.descriptor;
        // D s_11_1: size-of s_11_0
        let s_11_1: u16 = s_11_0.length();
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // C s_11_3: const #9s : i
        let s_11_3: i128 = 9;
        // D s_11_4: cmp-lt s_11_3 s_11_2
        let s_11_4: bool = ((s_11_3) < (s_11_2));
        // N s_11_5: assert s_11_4
        let s_11_5: () = assert!(s_11_4);
        // D s_11_6: read-var descriptor:bv
        let s_11_6: Bits = fn_state.descriptor;
        // D s_11_7: size-of s_11_6
        let s_11_7: u16 = s_11_6.length();
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // C s_11_9: const #49s : i
        let s_11_9: i128 = 49;
        // D s_11_10: cmp-lt s_11_9 s_11_8
        let s_11_10: bool = ((s_11_9) < (s_11_8));
        // N s_11_11: assert s_11_10
        let s_11_11: () = assert!(s_11_10);
        // C s_11_12: const #8s : i
        let s_11_12: i128 = 8;
        // D s_11_13: read-var descriptor:bv
        let s_11_13: Bits = fn_state.descriptor;
        // C s_11_14: const #1s : i64
        let s_11_14: i64 = 1;
        // C s_11_15: cast zx s_11_14 -> i
        let s_11_15: i128 = (i128::try_from(s_11_14).unwrap());
        // C s_11_16: const #1s : i
        let s_11_16: i128 = 1;
        // C s_11_17: add s_11_16 s_11_15
        let s_11_17: i128 = (s_11_16 + s_11_15);
        // D s_11_18: bit-extract s_11_13 s_11_12 s_11_17
        let s_11_18: Bits = (Bits::new(
            ((s_11_13) >> (s_11_12)).value(),
            u16::try_from(s_11_17).unwrap(),
        ));
        // D s_11_19: cast reint s_11_18 -> u8
        let s_11_19: u8 = (s_11_18.value() as u8);
        // C s_11_20: const #48s : i
        let s_11_20: i128 = 48;
        // D s_11_21: read-var descriptor:bv
        let s_11_21: Bits = fn_state.descriptor;
        // C s_11_22: const #1s : i64
        let s_11_22: i64 = 1;
        // C s_11_23: cast zx s_11_22 -> i
        let s_11_23: i128 = (i128::try_from(s_11_22).unwrap());
        // C s_11_24: const #1s : i
        let s_11_24: i128 = 1;
        // C s_11_25: add s_11_24 s_11_23
        let s_11_25: i128 = (s_11_24 + s_11_23);
        // D s_11_26: bit-extract s_11_21 s_11_20 s_11_25
        let s_11_26: Bits = (Bits::new(
            ((s_11_21) >> (s_11_20)).value(),
            u16::try_from(s_11_25).unwrap(),
        ));
        // D s_11_27: cast reint s_11_26 -> u8
        let s_11_27: u8 = (s_11_26.value() as u8);
        // D s_11_28: cast zx s_11_19 -> bv
        let s_11_28: Bits = Bits::new(s_11_19 as u128, 2u16);
        // D s_11_29: cast zx s_11_27 -> bv
        let s_11_29: Bits = Bits::new(s_11_27 as u128, 2u16);
        // D s_11_30: cast reint s_11_28 -> u128
        let s_11_30: u128 = (s_11_28.value() as u128);
        // D s_11_31: size-of s_11_28
        let s_11_31: u16 = s_11_28.length();
        // D s_11_32: cast reint s_11_29 -> u128
        let s_11_32: u128 = (s_11_29.value() as u128);
        // D s_11_33: size-of s_11_29
        let s_11_33: u16 = s_11_29.length();
        // D s_11_34: lsl s_11_30 s_11_33
        let s_11_34: u128 = s_11_30 << s_11_33;
        // D s_11_35: or s_11_34 s_11_32
        let s_11_35: u128 = ((s_11_34) | (s_11_32));
        // D s_11_36: add s_11_31 s_11_33
        let s_11_36: u16 = (s_11_31 + s_11_33);
        // D s_11_37: create-bits s_11_35 s_11_36
        let s_11_37: Bits = Bits::new(s_11_35, s_11_36);
        // D s_11_38: cast reint s_11_37 -> u8
        let s_11_38: u8 = (s_11_37.value() as u8);
        // C s_11_39: const #48s : i
        let s_11_39: i128 = 48;
        // D s_11_40: read-var tablebase:u56
        let s_11_40: u64 = fn_state.tablebase;
        // D s_11_41: cast zx s_11_40 -> bv
        let s_11_41: Bits = Bits::new(s_11_40 as u128, 56u16);
        // D s_11_42: cast zx s_11_38 -> bv
        let s_11_42: Bits = Bits::new(s_11_38 as u128, 4u16);
        // C s_11_43: const #3s : i
        let s_11_43: i128 = 3;
        // C s_11_44: const #1u : u64
        let s_11_44: u64 = 1;
        // C s_11_45: cast zx s_11_44 -> bv
        let s_11_45: Bits = Bits::new(s_11_44 as u128, 64u16);
        // C s_11_46: lsl s_11_45 s_11_43
        let s_11_46: Bits = s_11_45 << s_11_43;
        // C s_11_47: sub s_11_46 s_11_45
        let s_11_47: Bits = ((s_11_46) - (s_11_45));
        // D s_11_48: and s_11_42 s_11_47
        let s_11_48: Bits = ((s_11_42) & (s_11_47));
        // D s_11_49: lsl s_11_48 s_11_39
        let s_11_49: Bits = s_11_48 << s_11_39;
        // C s_11_50: lsl s_11_47 s_11_39
        let s_11_50: Bits = s_11_47 << s_11_39;
        // C s_11_51: cmpl s_11_50
        let s_11_51: Bits = !s_11_50;
        // D s_11_52: and s_11_41 s_11_51
        let s_11_52: Bits = ((s_11_41) & (s_11_51));
        // D s_11_53: or s_11_52 s_11_49
        let s_11_53: Bits = ((s_11_52) | (s_11_49));
        // D s_11_54: cast reint s_11_53 -> u56
        let s_11_54: u64 = (s_11_53.value() as u64);
        // D s_11_55: write-var tablebase <= s_11_54
        fn_state.tablebase = s_11_54;
        // D s_11_56: read-var tablebase:u56
        let s_11_56: u64 = fn_state.tablebase;
        // D s_11_57: write-var return_value <= s_11_56
        fn_state.return_value = s_11_56;
        // N s_11_58: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_12_0: read-var descriptor:bv
        let s_12_0: Bits = fn_state.descriptor;
        // D s_12_1: size-of s_12_0
        let s_12_1: u16 = s_12_0.length();
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #15s : i
        let s_12_3: i128 = 15;
        // D s_12_4: cmp-lt s_12_3 s_12_2
        let s_12_4: bool = ((s_12_3) < (s_12_2));
        // N s_12_5: assert s_12_4
        let s_12_5: () = assert!(s_12_4);
        // C s_12_6: const #12s : i
        let s_12_6: i128 = 12;
        // D s_12_7: read-var descriptor:bv
        let s_12_7: Bits = fn_state.descriptor;
        // C s_12_8: const #1s : i64
        let s_12_8: i64 = 1;
        // C s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // C s_12_10: const #3s : i
        let s_12_10: i128 = 3;
        // C s_12_11: add s_12_10 s_12_9
        let s_12_11: i128 = (s_12_10 + s_12_9);
        // D s_12_12: bit-extract s_12_7 s_12_6 s_12_11
        let s_12_12: Bits = (Bits::new(
            ((s_12_7) >> (s_12_6)).value(),
            u16::try_from(s_12_11).unwrap(),
        ));
        // D s_12_13: cast reint s_12_12 -> u8
        let s_12_13: u8 = (s_12_12.value() as u8);
        // C s_12_14: const #48s : i
        let s_12_14: i128 = 48;
        // D s_12_15: read-var tablebase:u56
        let s_12_15: u64 = fn_state.tablebase;
        // D s_12_16: cast zx s_12_15 -> bv
        let s_12_16: Bits = Bits::new(s_12_15 as u128, 56u16);
        // D s_12_17: cast zx s_12_13 -> bv
        let s_12_17: Bits = Bits::new(s_12_13 as u128, 4u16);
        // C s_12_18: const #3s : i
        let s_12_18: i128 = 3;
        // C s_12_19: const #1u : u64
        let s_12_19: u64 = 1;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 64u16);
        // C s_12_21: lsl s_12_20 s_12_18
        let s_12_21: Bits = s_12_20 << s_12_18;
        // C s_12_22: sub s_12_21 s_12_20
        let s_12_22: Bits = ((s_12_21) - (s_12_20));
        // D s_12_23: and s_12_17 s_12_22
        let s_12_23: Bits = ((s_12_17) & (s_12_22));
        // D s_12_24: lsl s_12_23 s_12_14
        let s_12_24: Bits = s_12_23 << s_12_14;
        // C s_12_25: lsl s_12_22 s_12_14
        let s_12_25: Bits = s_12_22 << s_12_14;
        // C s_12_26: cmpl s_12_25
        let s_12_26: Bits = !s_12_25;
        // D s_12_27: and s_12_16 s_12_26
        let s_12_27: Bits = ((s_12_16) & (s_12_26));
        // D s_12_28: or s_12_27 s_12_24
        let s_12_28: Bits = ((s_12_27) | (s_12_24));
        // D s_12_29: cast reint s_12_28 -> u56
        let s_12_29: u64 = (s_12_28.value() as u64);
        // D s_12_30: write-var tablebase <= s_12_29
        fn_state.tablebase = s_12_29;
        // D s_12_31: read-var tablebase:u56
        let s_12_31: u64 = fn_state.tablebase;
        // D s_12_32: write-var return_value <= s_12_31
        fn_state.return_value = s_12_31;
        // N s_12_33: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var tgx:u32
        let s_13_0: u32 = fn_state.tgx;
        // C s_13_1: const #2u : u32
        let s_13_1: u32 = 2;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#18097 <= s_13_2
        fn_state.gs_18097 = s_13_2;
        // N s_13_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_14_0: read-var descriptor:bv
        let s_14_0: Bits = fn_state.descriptor;
        // D s_14_1: size-of s_14_0
        let s_14_1: u16 = s_14_0.length();
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // C s_14_3: const #55s : i
        let s_14_3: i128 = 55;
        // D s_14_4: cmp-lt s_14_3 s_14_2
        let s_14_4: bool = ((s_14_3) < (s_14_2));
        // N s_14_5: assert s_14_4
        let s_14_5: () = assert!(s_14_4);
        // C s_14_6: const #48s : i
        let s_14_6: i128 = 48;
        // D s_14_7: read-var descriptor:bv
        let s_14_7: Bits = fn_state.descriptor;
        // C s_14_8: const #1s : i64
        let s_14_8: i64 = 1;
        // C s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // C s_14_10: const #7s : i
        let s_14_10: i128 = 7;
        // C s_14_11: add s_14_10 s_14_9
        let s_14_11: i128 = (s_14_10 + s_14_9);
        // D s_14_12: bit-extract s_14_7 s_14_6 s_14_11
        let s_14_12: Bits = (Bits::new(
            ((s_14_7) >> (s_14_6)).value(),
            u16::try_from(s_14_11).unwrap(),
        ));
        // D s_14_13: cast reint s_14_12 -> u8
        let s_14_13: u8 = (s_14_12.value() as u8);
        // C s_14_14: const #48s : i
        let s_14_14: i128 = 48;
        // D s_14_15: read-var tablebase:u56
        let s_14_15: u64 = fn_state.tablebase;
        // D s_14_16: cast zx s_14_15 -> bv
        let s_14_16: Bits = Bits::new(s_14_15 as u128, 56u16);
        // D s_14_17: cast zx s_14_13 -> bv
        let s_14_17: Bits = Bits::new(s_14_13 as u128, 8u16);
        // C s_14_18: const #7s : i
        let s_14_18: i128 = 7;
        // C s_14_19: const #1u : u64
        let s_14_19: u64 = 1;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 64u16);
        // C s_14_21: lsl s_14_20 s_14_18
        let s_14_21: Bits = s_14_20 << s_14_18;
        // C s_14_22: sub s_14_21 s_14_20
        let s_14_22: Bits = ((s_14_21) - (s_14_20));
        // D s_14_23: and s_14_17 s_14_22
        let s_14_23: Bits = ((s_14_17) & (s_14_22));
        // D s_14_24: lsl s_14_23 s_14_14
        let s_14_24: Bits = s_14_23 << s_14_14;
        // C s_14_25: lsl s_14_22 s_14_14
        let s_14_25: Bits = s_14_22 << s_14_14;
        // C s_14_26: cmpl s_14_25
        let s_14_26: Bits = !s_14_25;
        // D s_14_27: and s_14_16 s_14_26
        let s_14_27: Bits = ((s_14_16) & (s_14_26));
        // D s_14_28: or s_14_27 s_14_24
        let s_14_28: Bits = ((s_14_27) | (s_14_24));
        // D s_14_29: cast reint s_14_28 -> u56
        let s_14_29: u64 = (s_14_28.value() as u64);
        // D s_14_30: write-var tablebase <= s_14_29
        fn_state.tablebase = s_14_29;
        // D s_14_31: read-var tablebase:u56
        let s_14_31: u64 = fn_state.tablebase;
        // D s_14_32: write-var return_value <= s_14_31
        fn_state.return_value = s_14_31;
        // N s_14_33: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_15_0: read-var d128:u8
        let s_15_0: bool = fn_state.d128;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#18096 <= s_15_4
        fn_state.gs_18096 = s_15_4;
        // N s_15_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_16_0: const #1u : u32
        let s_16_0: u32 = 1;
        // D s_16_1: read-var tgx:u32
        let s_16_1: u32 = fn_state.tgx;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b18 b17
        if s_16_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var descriptor:bv
        let s_17_0: Bits = fn_state.descriptor;
        // D s_17_1: size-of s_17_0
        let s_17_1: u16 = s_17_0.length();
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // C s_17_3: const #47s : i
        let s_17_3: i128 = 47;
        // D s_17_4: cmp-lt s_17_3 s_17_2
        let s_17_4: bool = ((s_17_3) < (s_17_2));
        // N s_17_5: assert s_17_4
        let s_17_5: () = assert!(s_17_4);
        // C s_17_6: const #14s : i
        let s_17_6: i128 = 14;
        // D s_17_7: read-var descriptor:bv
        let s_17_7: Bits = fn_state.descriptor;
        // C s_17_8: const #1s : i64
        let s_17_8: i64 = 1;
        // C s_17_9: cast zx s_17_8 -> i
        let s_17_9: i128 = (i128::try_from(s_17_8).unwrap());
        // C s_17_10: const #33s : i
        let s_17_10: i128 = 33;
        // C s_17_11: add s_17_10 s_17_9
        let s_17_11: i128 = (s_17_10 + s_17_9);
        // D s_17_12: bit-extract s_17_7 s_17_6 s_17_11
        let s_17_12: Bits = (Bits::new(
            ((s_17_7) >> (s_17_6)).value(),
            u16::try_from(s_17_11).unwrap(),
        ));
        // D s_17_13: cast reint s_17_12 -> u34
        let s_17_13: u64 = (s_17_12.value() as u64);
        // C s_17_14: const #14s : i
        let s_17_14: i128 = 14;
        // D s_17_15: read-var tablebase:u56
        let s_17_15: u64 = fn_state.tablebase;
        // D s_17_16: cast zx s_17_15 -> bv
        let s_17_16: Bits = Bits::new(s_17_15 as u128, 56u16);
        // D s_17_17: cast zx s_17_13 -> bv
        let s_17_17: Bits = Bits::new(s_17_13 as u128, 34u16);
        // C s_17_18: const #33s : i
        let s_17_18: i128 = 33;
        // C s_17_19: const #1u : u64
        let s_17_19: u64 = 1;
        // C s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 64u16);
        // C s_17_21: lsl s_17_20 s_17_18
        let s_17_21: Bits = s_17_20 << s_17_18;
        // C s_17_22: sub s_17_21 s_17_20
        let s_17_22: Bits = ((s_17_21) - (s_17_20));
        // D s_17_23: and s_17_17 s_17_22
        let s_17_23: Bits = ((s_17_17) & (s_17_22));
        // D s_17_24: lsl s_17_23 s_17_14
        let s_17_24: Bits = s_17_23 << s_17_14;
        // C s_17_25: lsl s_17_22 s_17_14
        let s_17_25: Bits = s_17_22 << s_17_14;
        // C s_17_26: cmpl s_17_25
        let s_17_26: Bits = !s_17_25;
        // D s_17_27: and s_17_16 s_17_26
        let s_17_27: Bits = ((s_17_16) & (s_17_26));
        // D s_17_28: or s_17_27 s_17_24
        let s_17_28: Bits = ((s_17_27) | (s_17_24));
        // D s_17_29: cast reint s_17_28 -> u56
        let s_17_29: u64 = (s_17_28.value() as u64);
        // D s_17_30: write-var tablebase <= s_17_29
        fn_state.tablebase = s_17_29;
        // N s_17_31: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: read-var tgx:u32
        let s_18_1: u32 = fn_state.tgx;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var descriptor:bv
        let s_19_0: Bits = fn_state.descriptor;
        // D s_19_1: size-of s_19_0
        let s_19_1: u16 = s_19_0.length();
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // C s_19_3: const #47s : i
        let s_19_3: i128 = 47;
        // D s_19_4: cmp-lt s_19_3 s_19_2
        let s_19_4: bool = ((s_19_3) < (s_19_2));
        // N s_19_5: assert s_19_4
        let s_19_5: () = assert!(s_19_4);
        // C s_19_6: const #16s : i
        let s_19_6: i128 = 16;
        // D s_19_7: read-var descriptor:bv
        let s_19_7: Bits = fn_state.descriptor;
        // C s_19_8: const #1s : i64
        let s_19_8: i64 = 1;
        // C s_19_9: cast zx s_19_8 -> i
        let s_19_9: i128 = (i128::try_from(s_19_8).unwrap());
        // C s_19_10: const #31s : i
        let s_19_10: i128 = 31;
        // C s_19_11: add s_19_10 s_19_9
        let s_19_11: i128 = (s_19_10 + s_19_9);
        // D s_19_12: bit-extract s_19_7 s_19_6 s_19_11
        let s_19_12: Bits = (Bits::new(
            ((s_19_7) >> (s_19_6)).value(),
            u16::try_from(s_19_11).unwrap(),
        ));
        // D s_19_13: cast reint s_19_12 -> u32
        let s_19_13: u32 = (s_19_12.value() as u32);
        // C s_19_14: const #16s : i
        let s_19_14: i128 = 16;
        // D s_19_15: read-var tablebase:u56
        let s_19_15: u64 = fn_state.tablebase;
        // D s_19_16: cast zx s_19_15 -> bv
        let s_19_16: Bits = Bits::new(s_19_15 as u128, 56u16);
        // D s_19_17: cast zx s_19_13 -> bv
        let s_19_17: Bits = Bits::new(s_19_13 as u128, 32u16);
        // C s_19_18: const #31s : i
        let s_19_18: i128 = 31;
        // C s_19_19: const #1u : u64
        let s_19_19: u64 = 1;
        // C s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 64u16);
        // C s_19_21: lsl s_19_20 s_19_18
        let s_19_21: Bits = s_19_20 << s_19_18;
        // C s_19_22: sub s_19_21 s_19_20
        let s_19_22: Bits = ((s_19_21) - (s_19_20));
        // D s_19_23: and s_19_17 s_19_22
        let s_19_23: Bits = ((s_19_17) & (s_19_22));
        // D s_19_24: lsl s_19_23 s_19_14
        let s_19_24: Bits = s_19_23 << s_19_14;
        // C s_19_25: lsl s_19_22 s_19_14
        let s_19_25: Bits = s_19_22 << s_19_14;
        // C s_19_26: cmpl s_19_25
        let s_19_26: Bits = !s_19_25;
        // D s_19_27: and s_19_16 s_19_26
        let s_19_27: Bits = ((s_19_16) & (s_19_26));
        // D s_19_28: or s_19_27 s_19_24
        let s_19_28: Bits = ((s_19_27) | (s_19_24));
        // D s_19_29: cast reint s_19_28 -> u56
        let s_19_29: u64 = (s_19_28.value() as u64);
        // D s_19_30: write-var tablebase <= s_19_29
        fn_state.tablebase = s_19_29;
        // N s_19_31: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_20_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
