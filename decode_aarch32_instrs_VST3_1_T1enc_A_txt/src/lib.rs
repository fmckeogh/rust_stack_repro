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
use neq_int::*;
use execute_aarch32_instrs_VST3_1_Op_A_txt::*;
use ConditionPassed::*;
use common::*;
pub fn decode_aarch32_instrs_VST3_1_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    index_align: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        d2: i64,
        index: i64,
        n: i64,
        d: i64,
        gs_321405: bool,
        register_index: bool,
        gs_321408: bool,
        wback: bool,
        d3: i64,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        index_align: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        size,
        index_align,
        Rm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b14 b3
        if s_2_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var index_align:u8
        let s_3_1: u8 = fn_state.index_align;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #0u : u8
        let s_3_19: bool = false;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-ne s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) != (s_3_20));
        // N s_3_22: branch s_3_21 b13 b4
        if s_3_21 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var index_align:u8
        let s_4_1: u8 = fn_state.index_align;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #2s : i
        let s_4_5: i128 = 2;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 3u16);
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (s_4_9.value() as i128);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: write-var index <= s_4_11
        fn_state.index = s_4_11;
        // C s_4_13: const #1s : i64
        let s_4_13: i64 = 1;
        // D s_4_14: read-var D:u8
        let s_4_14: bool = fn_state.D;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 1u16);
        // D s_4_16: read-var Vd:u8
        let s_4_16: u8 = fn_state.Vd;
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 4u16);
        // D s_4_18: cast reint s_4_15 -> u128
        let s_4_18: u128 = (s_4_15.value() as u128);
        // D s_4_19: size-of s_4_15
        let s_4_19: u16 = s_4_15.length();
        // D s_4_20: cast reint s_4_17 -> u128
        let s_4_20: u128 = (s_4_17.value() as u128);
        // D s_4_21: size-of s_4_17
        let s_4_21: u16 = s_4_17.length();
        // D s_4_22: lsl s_4_18 s_4_21
        let s_4_22: u128 = s_4_18 << s_4_21;
        // D s_4_23: or s_4_22 s_4_20
        let s_4_23: u128 = ((s_4_22) | (s_4_20));
        // D s_4_24: add s_4_19 s_4_21
        let s_4_24: u16 = (s_4_19 + s_4_21);
        // D s_4_25: create-bits s_4_23 s_4_24
        let s_4_25: Bits = Bits::new(s_4_23, s_4_24);
        // D s_4_26: cast reint s_4_25 -> u8
        let s_4_26: u8 = (s_4_25.value() as u8);
        // D s_4_27: cast zx s_4_26 -> bv
        let s_4_27: Bits = Bits::new(s_4_26 as u128, 5u16);
        // D s_4_28: cast zx s_4_27 -> i
        let s_4_28: i128 = (s_4_27.value() as i128);
        // D s_4_29: cast reint s_4_28 -> i64
        let s_4_29: i64 = (s_4_28 as i64);
        // D s_4_30: write-var d <= s_4_29
        fn_state.d = s_4_29;
        // D s_4_31: read-var d:i64
        let s_4_31: i64 = fn_state.d;
        // D s_4_32: cast zx s_4_31 -> i
        let s_4_32: i128 = (i128::try_from(s_4_31).unwrap());
        // C s_4_33: cast zx s_4_13 -> i
        let s_4_33: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_34: add s_4_32 s_4_33
        let s_4_34: i128 = (s_4_32 + s_4_33);
        // D s_4_35: cast reint s_4_34 -> i64
        let s_4_35: i64 = (s_4_34 as i64);
        // D s_4_36: write-var d2 <= s_4_35
        fn_state.d2 = s_4_35;
        // D s_4_37: read-var d2:i64
        let s_4_37: i64 = fn_state.d2;
        // D s_4_38: cast zx s_4_37 -> i
        let s_4_38: i128 = (i128::try_from(s_4_37).unwrap());
        // C s_4_39: cast zx s_4_13 -> i
        let s_4_39: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_40: add s_4_38 s_4_39
        let s_4_40: i128 = (s_4_38 + s_4_39);
        // D s_4_41: cast reint s_4_40 -> i64
        let s_4_41: i64 = (s_4_40 as i64);
        // D s_4_42: write-var d3 <= s_4_41
        fn_state.d3 = s_4_41;
        // D s_4_43: read-var Rn:u8
        let s_4_43: u8 = fn_state.Rn;
        // D s_4_44: cast zx s_4_43 -> bv
        let s_4_44: Bits = Bits::new(s_4_43 as u128, 4u16);
        // D s_4_45: cast zx s_4_44 -> i
        let s_4_45: i128 = (s_4_44.value() as i128);
        // D s_4_46: cast reint s_4_45 -> i64
        let s_4_46: i64 = (s_4_45 as i64);
        // D s_4_47: write-var n <= s_4_46
        fn_state.n = s_4_46;
        // D s_4_48: read-var Rm:u8
        let s_4_48: u8 = fn_state.Rm;
        // D s_4_49: cast zx s_4_48 -> bv
        let s_4_49: Bits = Bits::new(s_4_48 as u128, 4u16);
        // D s_4_50: cast zx s_4_49 -> i
        let s_4_50: i128 = (s_4_49.value() as i128);
        // D s_4_51: cast reint s_4_50 -> i64
        let s_4_51: i64 = (s_4_50 as i64);
        // D s_4_52: write-var m <= s_4_51
        fn_state.m = s_4_51;
        // C s_4_53: const #15s : i
        let s_4_53: i128 = 15;
        // D s_4_54: read-var m:i64
        let s_4_54: i64 = fn_state.m;
        // D s_4_55: cast zx s_4_54 -> i
        let s_4_55: i128 = (i128::try_from(s_4_54).unwrap());
        // D s_4_56: call neq_int(s_4_55, s_4_53)
        let s_4_56: bool = neq_int(state, tracer, s_4_55, s_4_53);
        // D s_4_57: write-var wback <= s_4_56
        fn_state.wback = s_4_56;
        // C s_4_58: const #15s : i
        let s_4_58: i128 = 15;
        // D s_4_59: read-var m:i64
        let s_4_59: i64 = fn_state.m;
        // D s_4_60: cast zx s_4_59 -> i
        let s_4_60: i128 = (i128::try_from(s_4_59).unwrap());
        // D s_4_61: call neq_int(s_4_60, s_4_58)
        let s_4_61: bool = neq_int(state, tracer, s_4_60, s_4_58);
        // N s_4_62: branch s_4_61 b12 b5
        if s_4_61 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#321405 <= s_5_0
        fn_state.gs_321405 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#321405:u8
        let s_6_0: bool = fn_state.gs_321405;
        // D s_6_1: write-var register_index <= s_6_0
        fn_state.register_index = s_6_0;
        // C s_6_2: const #15s : i
        let s_6_2: i128 = 15;
        // D s_6_3: read-var n:i64
        let s_6_3: i64 = fn_state.n;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cmp-eq s_6_4 s_6_2
        let s_6_5: bool = ((s_6_4) == (s_6_2));
        // N s_6_6: branch s_6_5 b11 b7
        if s_6_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #31s : i
        let s_7_0: i128 = 31;
        // D s_7_1: read-var d3:i64
        let s_7_1: i64 = fn_state.d3;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cmp-gt s_7_2 s_7_0
        let s_7_3: bool = ((s_7_2) > (s_7_0));
        // D s_7_4: write-var gs#321408 <= s_7_3
        fn_state.gs_321408 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#321408:u8
        let s_8_0: bool = fn_state.gs_321408;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var d2:i64
        let s_9_0: i64 = fn_state.d2;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var d3:i64
        let s_9_2: i64 = fn_state.d3;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var d:i64
        let s_9_4: i64 = fn_state.d;
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // D s_9_6: read-var index:i64
        let s_9_6: i64 = fn_state.index;
        // D s_9_7: read-var m:i64
        let s_9_7: i64 = fn_state.m;
        // D s_9_8: read-var n:i64
        let s_9_8: i64 = fn_state.n;
        // D s_9_9: read-var register_index:u8
        let s_9_9: bool = fn_state.register_index;
        // D s_9_10: read-var wback:u8
        let s_9_10: bool = fn_state.wback;
        // D s_9_11: call execute_aarch32_instrs_VST3_1_Op_A_txt(s_9_4, s_9_1, s_9_3, s_9_5, s_9_6, s_9_7, s_9_8, s_9_9, s_9_10)
        let s_9_11: () = execute_aarch32_instrs_VST3_1_Op_A_txt(
            state,
            tracer,
            s_9_4,
            s_9_1,
            s_9_3,
            s_9_5,
            s_9_6,
            s_9_7,
            s_9_8,
            s_9_9,
            s_9_10,
        );
        // N s_9_12: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#321408 <= s_11_0
        fn_state.gs_321408 = s_11_0;
        // N s_11_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #13s : i
        let s_12_0: i128 = 13;
        // D s_12_1: read-var m:i64
        let s_12_1: i64 = fn_state.m;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: call neq_int(s_12_2, s_12_0)
        let s_12_3: bool = neq_int(state, tracer, s_12_2, s_12_0);
        // D s_12_4: write-var gs#321405 <= s_12_3
        fn_state.gs_321405 = s_12_3;
        // N s_12_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
}
