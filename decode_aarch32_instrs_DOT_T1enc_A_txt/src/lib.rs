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
use InITBlock::*;
use execute_aarch32_instrs_DOT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_DOT_T1enc_A_txt<T: Tracer>(
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
        gs_327398: bool,
        n: i64,
        d: i64,
        op1_unsigned: bool,
        i: i64,
        gs_327399: bool,
        ga_367528: i64,
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
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b15 b1
        if s_0_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveAArch32Int8MatMulExt(s_1_0)
        let s_1_1: bool = HaveAArch32Int8MatMulExt(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b14 b2
        if s_1_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b10 b3
        if s_2_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#327399 <= s_3_0
        fn_state.gs_327399 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#327399:u8
        let s_4_0: bool = fn_state.gs_327399;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var U:u8
        let s_5_0: bool = fn_state.U;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var op1_unsigned <= s_5_4
        fn_state.op1_unsigned = s_5_4;
        // D s_5_6: read-var U:u8
        let s_5_6: bool = fn_state.U;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 1u16);
        // C s_5_8: const #1u : u8
        let s_5_8: bool = true;
        // C s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 1u16);
        // D s_5_10: cmp-eq s_5_7 s_5_9
        let s_5_10: bool = ((s_5_7) == (s_5_9));
        // D s_5_11: write-var op2_unsigned <= s_5_10
        fn_state.op2_unsigned = s_5_10;
        // D s_5_12: read-var D:u8
        let s_5_12: bool = fn_state.D;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 1u16);
        // D s_5_14: read-var Vd:u8
        let s_5_14: u8 = fn_state.Vd;
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 4u16);
        // D s_5_16: cast reint s_5_13 -> u128
        let s_5_16: u128 = (s_5_13.value() as u128);
        // D s_5_17: size-of s_5_13
        let s_5_17: u16 = s_5_13.length();
        // D s_5_18: cast reint s_5_15 -> u128
        let s_5_18: u128 = (s_5_15.value() as u128);
        // D s_5_19: size-of s_5_15
        let s_5_19: u16 = s_5_15.length();
        // D s_5_20: lsl s_5_16 s_5_19
        let s_5_20: u128 = s_5_16 << s_5_19;
        // D s_5_21: or s_5_20 s_5_18
        let s_5_21: u128 = ((s_5_20) | (s_5_18));
        // D s_5_22: add s_5_17 s_5_19
        let s_5_22: u16 = (s_5_17 + s_5_19);
        // D s_5_23: create-bits s_5_21 s_5_22
        let s_5_23: Bits = Bits::new(s_5_21, s_5_22);
        // D s_5_24: cast reint s_5_23 -> u8
        let s_5_24: u8 = (s_5_23.value() as u8);
        // D s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 5u16);
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (s_5_25.value() as i128);
        // D s_5_27: cast reint s_5_26 -> i64
        let s_5_27: i64 = (s_5_26 as i64);
        // D s_5_28: write-var d <= s_5_27
        fn_state.d = s_5_27;
        // D s_5_29: read-var N:u8
        let s_5_29: bool = fn_state.N;
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 1u16);
        // D s_5_31: read-var Vn:u8
        let s_5_31: u8 = fn_state.Vn;
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 4u16);
        // D s_5_33: cast reint s_5_30 -> u128
        let s_5_33: u128 = (s_5_30.value() as u128);
        // D s_5_34: size-of s_5_30
        let s_5_34: u16 = s_5_30.length();
        // D s_5_35: cast reint s_5_32 -> u128
        let s_5_35: u128 = (s_5_32.value() as u128);
        // D s_5_36: size-of s_5_32
        let s_5_36: u16 = s_5_32.length();
        // D s_5_37: lsl s_5_33 s_5_36
        let s_5_37: u128 = s_5_33 << s_5_36;
        // D s_5_38: or s_5_37 s_5_35
        let s_5_38: u128 = ((s_5_37) | (s_5_35));
        // D s_5_39: add s_5_34 s_5_36
        let s_5_39: u16 = (s_5_34 + s_5_36);
        // D s_5_40: create-bits s_5_38 s_5_39
        let s_5_40: Bits = Bits::new(s_5_38, s_5_39);
        // D s_5_41: cast reint s_5_40 -> u8
        let s_5_41: u8 = (s_5_40.value() as u8);
        // D s_5_42: cast zx s_5_41 -> bv
        let s_5_42: Bits = Bits::new(s_5_41 as u128, 5u16);
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (s_5_42.value() as i128);
        // D s_5_44: cast reint s_5_43 -> i64
        let s_5_44: i64 = (s_5_43 as i64);
        // D s_5_45: write-var n <= s_5_44
        fn_state.n = s_5_44;
        // D s_5_46: read-var Vm:u8
        let s_5_46: u8 = fn_state.Vm;
        // D s_5_47: cast zx s_5_46 -> bv
        let s_5_47: Bits = Bits::new(s_5_46 as u128, 4u16);
        // D s_5_48: cast zx s_5_47 -> i
        let s_5_48: i128 = (s_5_47.value() as i128);
        // D s_5_49: cast reint s_5_48 -> i64
        let s_5_49: i64 = (s_5_48 as i64);
        // D s_5_50: write-var m <= s_5_49
        fn_state.m = s_5_49;
        // D s_5_51: read-var M:u8
        let s_5_51: bool = fn_state.M;
        // D s_5_52: cast zx s_5_51 -> bv
        let s_5_52: Bits = Bits::new(s_5_51 as u128, 1u16);
        // D s_5_53: cast zx s_5_52 -> i
        let s_5_53: i128 = (s_5_52.value() as i128);
        // D s_5_54: cast reint s_5_53 -> i64
        let s_5_54: i64 = (s_5_53 as i64);
        // D s_5_55: write-var i <= s_5_54
        fn_state.i = s_5_54;
        // D s_5_56: read-var Q:u8
        let s_5_56: bool = fn_state.Q;
        // D s_5_57: cast zx s_5_56 -> bv
        let s_5_57: Bits = Bits::new(s_5_56 as u128, 1u16);
        // C s_5_58: const #1u : u8
        let s_5_58: bool = true;
        // C s_5_59: cast zx s_5_58 -> bv
        let s_5_59: Bits = Bits::new(s_5_58 as u128, 1u16);
        // D s_5_60: cmp-eq s_5_57 s_5_59
        let s_5_60: bool = ((s_5_57) == (s_5_59));
        // N s_5_61: branch s_5_60 b8 b6
        if s_5_60 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1s : i64
        let s_6_0: i64 = 1;
        // D s_6_1: write-var ga#367528 <= s_6_0
        fn_state.ga_367528 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#367528:i64
        let s_7_0: i64 = fn_state.ga_367528;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // D s_7_2: read-var i:i64
        let s_7_2: i64 = fn_state.i;
        // D s_7_3: read-var m:i64
        let s_7_3: i64 = fn_state.m;
        // D s_7_4: read-var n:i64
        let s_7_4: i64 = fn_state.n;
        // D s_7_5: read-var op1_unsigned:u8
        let s_7_5: bool = fn_state.op1_unsigned;
        // D s_7_6: read-var op2_unsigned:u8
        let s_7_6: bool = fn_state.op2_unsigned;
        // D s_7_7: call execute_aarch32_instrs_DOT_Op_A_txt(s_7_1, s_7_2, s_7_3, s_7_4, s_7_5, s_7_6, s_7_0)
        let s_7_7: () = execute_aarch32_instrs_DOT_Op_A_txt(
            state,
            tracer,
            s_7_1,
            s_7_2,
            s_7_3,
            s_7_4,
            s_7_5,
            s_7_6,
            s_7_0,
        );
        // N s_7_8: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2s : i64
        let s_8_0: i64 = 2;
        // D s_8_1: write-var ga#367528 <= s_8_0
        fn_state.ga_367528 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var Vd:u8
        let s_10_1: u8 = fn_state.Vd;
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
        // N s_10_22: branch s_10_21 b13 b11
        if s_10_21 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var Vn:u8
        let s_11_1: u8 = fn_state.Vn;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 4u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #1u : u8
        let s_11_19: bool = true;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // D s_11_22: write-var gs#327398 <= s_11_21
        fn_state.gs_327398 = s_11_21;
        // N s_11_23: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#327398:u8
        let s_12_0: bool = fn_state.gs_327398;
        // D s_12_1: write-var gs#327399 <= s_12_0
        fn_state.gs_327399 = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#327398 <= s_13_0
        fn_state.gs_327398 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
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
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
}
