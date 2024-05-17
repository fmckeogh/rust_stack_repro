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
use HaveAArch32Int8MatMulExt::*;
use execute_aarch32_instrs_DOT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_DOT_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    U: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_327379: bool,
        n: i64,
        d: i64,
        op1_unsigned: bool,
        gs_327378: bool,
        i: i64,
        ga_367507: i64,
        op2_unsigned: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        U: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vn,
        Vd,
        N,
        Q,
        M,
        U,
        Vm,
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
        // S s_0_1: call HaveAArch32Int8MatMulExt(s_0_0)
        let s_0_1: bool = HaveAArch32Int8MatMulExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b13 b1
        if s_0_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Q:u8
        let s_1_0: bool = fn_state.Q;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b9 b2
        if s_1_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#327379 <= s_2_0
        fn_state.gs_327379 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#327379:u8
        let s_3_0: bool = fn_state.gs_327379;
        // N s_3_1: branch s_3_0 b8 b4
        if s_3_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var U:u8
        let s_4_0: bool = fn_state.U;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var op1_unsigned <= s_4_4
        fn_state.op1_unsigned = s_4_4;
        // D s_4_6: read-var U:u8
        let s_4_6: bool = fn_state.U;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // C s_4_8: const #1u : u8
        let s_4_8: bool = true;
        // C s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 1u16);
        // D s_4_10: cmp-eq s_4_7 s_4_9
        let s_4_10: bool = ((s_4_7) == (s_4_9));
        // D s_4_11: write-var op2_unsigned <= s_4_10
        fn_state.op2_unsigned = s_4_10;
        // D s_4_12: read-var D:u8
        let s_4_12: bool = fn_state.D;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 1u16);
        // D s_4_14: read-var Vd:u8
        let s_4_14: u8 = fn_state.Vd;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 4u16);
        // D s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: cast reint s_4_15 -> u128
        let s_4_18: u128 = (s_4_15.value() as u128);
        // D s_4_19: size-of s_4_15
        let s_4_19: u16 = s_4_15.length();
        // D s_4_20: lsl s_4_16 s_4_19
        let s_4_20: u128 = s_4_16 << s_4_19;
        // D s_4_21: or s_4_20 s_4_18
        let s_4_21: u128 = ((s_4_20) | (s_4_18));
        // D s_4_22: add s_4_17 s_4_19
        let s_4_22: u16 = (s_4_17 + s_4_19);
        // D s_4_23: create-bits s_4_21 s_4_22
        let s_4_23: Bits = Bits::new(s_4_21, s_4_22);
        // D s_4_24: cast reint s_4_23 -> u8
        let s_4_24: u8 = (s_4_23.value() as u8);
        // D s_4_25: cast zx s_4_24 -> bv
        let s_4_25: Bits = Bits::new(s_4_24 as u128, 5u16);
        // D s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (s_4_25.value() as i128);
        // D s_4_27: cast reint s_4_26 -> i64
        let s_4_27: i64 = (s_4_26 as i64);
        // D s_4_28: write-var d <= s_4_27
        fn_state.d = s_4_27;
        // D s_4_29: read-var N:u8
        let s_4_29: bool = fn_state.N;
        // D s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 1u16);
        // D s_4_31: read-var Vn:u8
        let s_4_31: u8 = fn_state.Vn;
        // D s_4_32: cast zx s_4_31 -> bv
        let s_4_32: Bits = Bits::new(s_4_31 as u128, 4u16);
        // D s_4_33: cast reint s_4_30 -> u128
        let s_4_33: u128 = (s_4_30.value() as u128);
        // D s_4_34: size-of s_4_30
        let s_4_34: u16 = s_4_30.length();
        // D s_4_35: cast reint s_4_32 -> u128
        let s_4_35: u128 = (s_4_32.value() as u128);
        // D s_4_36: size-of s_4_32
        let s_4_36: u16 = s_4_32.length();
        // D s_4_37: lsl s_4_33 s_4_36
        let s_4_37: u128 = s_4_33 << s_4_36;
        // D s_4_38: or s_4_37 s_4_35
        let s_4_38: u128 = ((s_4_37) | (s_4_35));
        // D s_4_39: add s_4_34 s_4_36
        let s_4_39: u16 = (s_4_34 + s_4_36);
        // D s_4_40: create-bits s_4_38 s_4_39
        let s_4_40: Bits = Bits::new(s_4_38, s_4_39);
        // D s_4_41: cast reint s_4_40 -> u8
        let s_4_41: u8 = (s_4_40.value() as u8);
        // D s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 5u16);
        // D s_4_43: cast zx s_4_42 -> i
        let s_4_43: i128 = (s_4_42.value() as i128);
        // D s_4_44: cast reint s_4_43 -> i64
        let s_4_44: i64 = (s_4_43 as i64);
        // D s_4_45: write-var n <= s_4_44
        fn_state.n = s_4_44;
        // D s_4_46: read-var Vm:u8
        let s_4_46: u8 = fn_state.Vm;
        // D s_4_47: cast zx s_4_46 -> bv
        let s_4_47: Bits = Bits::new(s_4_46 as u128, 4u16);
        // D s_4_48: cast zx s_4_47 -> i
        let s_4_48: i128 = (s_4_47.value() as i128);
        // D s_4_49: cast reint s_4_48 -> i64
        let s_4_49: i64 = (s_4_48 as i64);
        // D s_4_50: write-var m <= s_4_49
        fn_state.m = s_4_49;
        // D s_4_51: read-var M:u8
        let s_4_51: bool = fn_state.M;
        // D s_4_52: cast zx s_4_51 -> bv
        let s_4_52: Bits = Bits::new(s_4_51 as u128, 1u16);
        // D s_4_53: cast zx s_4_52 -> i
        let s_4_53: i128 = (s_4_52.value() as i128);
        // D s_4_54: cast reint s_4_53 -> i64
        let s_4_54: i64 = (s_4_53 as i64);
        // D s_4_55: write-var i <= s_4_54
        fn_state.i = s_4_54;
        // D s_4_56: read-var Q:u8
        let s_4_56: bool = fn_state.Q;
        // D s_4_57: cast zx s_4_56 -> bv
        let s_4_57: Bits = Bits::new(s_4_56 as u128, 1u16);
        // C s_4_58: const #1u : u8
        let s_4_58: bool = true;
        // C s_4_59: cast zx s_4_58 -> bv
        let s_4_59: Bits = Bits::new(s_4_58 as u128, 1u16);
        // D s_4_60: cmp-eq s_4_57 s_4_59
        let s_4_60: bool = ((s_4_57) == (s_4_59));
        // N s_4_61: branch s_4_60 b7 b5
        if s_4_60 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i64
        let s_5_0: i64 = 1;
        // D s_5_1: write-var ga#367507 <= s_5_0
        fn_state.ga_367507 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#367507:i64
        let s_6_0: i64 = fn_state.ga_367507;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // D s_6_2: read-var i:i64
        let s_6_2: i64 = fn_state.i;
        // D s_6_3: read-var m:i64
        let s_6_3: i64 = fn_state.m;
        // D s_6_4: read-var n:i64
        let s_6_4: i64 = fn_state.n;
        // D s_6_5: read-var op1_unsigned:u8
        let s_6_5: bool = fn_state.op1_unsigned;
        // D s_6_6: read-var op2_unsigned:u8
        let s_6_6: bool = fn_state.op2_unsigned;
        // D s_6_7: call execute_aarch32_instrs_DOT_Op_A_txt(s_6_1, s_6_2, s_6_3, s_6_4, s_6_5, s_6_6, s_6_0)
        let s_6_7: () = execute_aarch32_instrs_DOT_Op_A_txt(
            state,
            tracer,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_0,
        );
        // N s_6_8: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // D s_7_1: write-var ga#367507 <= s_7_0
        fn_state.ga_367507 = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var Vd:u8
        let s_9_1: u8 = fn_state.Vd;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 4u16);
        // C s_9_3: const #1u : u64
        let s_9_3: u64 = 1;
        // D s_9_4: bit-extract s_9_2 s_9_0 s_9_3
        let s_9_4: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_3).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: bool = ((s_9_4.value()) != 0);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #0u : u64
        let s_9_7: u64 = 0;
        // D s_9_8: cast zx s_9_5 -> u64
        let s_9_8: u64 = (s_9_5 as u64);
        // C s_9_9: const #1u : u64
        let s_9_9: u64 = 1;
        // D s_9_10: and s_9_8 s_9_9
        let s_9_10: u64 = ((s_9_8) & (s_9_9));
        // D s_9_11: cmp-eq s_9_10 s_9_9
        let s_9_11: bool = ((s_9_10) == (s_9_9));
        // D s_9_12: lsl s_9_8 s_9_6
        let s_9_12: u64 = s_9_8 << s_9_6;
        // D s_9_13: or s_9_7 s_9_12
        let s_9_13: u64 = ((s_9_7) | (s_9_12));
        // D s_9_14: cmpl s_9_12
        let s_9_14: u64 = !s_9_12;
        // D s_9_15: and s_9_7 s_9_14
        let s_9_15: u64 = ((s_9_7) & (s_9_14));
        // D s_9_16: select s_9_11 s_9_13 s_9_15
        let s_9_16: u64 = if s_9_11 { s_9_13 } else { s_9_15 };
        // D s_9_17: cast trunc s_9_16 -> u8
        let s_9_17: bool = ((s_9_16) != 0);
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 1u16);
        // C s_9_19: const #1u : u8
        let s_9_19: bool = true;
        // C s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: cmp-eq s_9_18 s_9_20
        let s_9_21: bool = ((s_9_18) == (s_9_20));
        // N s_9_22: branch s_9_21 b12 b10
        if s_9_21 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var Vn:u8
        let s_10_1: u8 = fn_state.Vn;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 4u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // C s_10_19: const #1u : u8
        let s_10_19: bool = true;
        // C s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: cmp-eq s_10_18 s_10_20
        let s_10_21: bool = ((s_10_18) == (s_10_20));
        // D s_10_22: write-var gs#327378 <= s_10_21
        fn_state.gs_327378 = s_10_21;
        // N s_10_23: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#327378:u8
        let s_11_0: bool = fn_state.gs_327378;
        // D s_11_1: write-var gs#327379 <= s_11_0
        fn_state.gs_327379 = s_11_0;
        // N s_11_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#327378 <= s_12_0
        fn_state.gs_327378 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
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
}
