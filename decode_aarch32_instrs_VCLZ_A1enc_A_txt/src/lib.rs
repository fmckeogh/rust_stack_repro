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
use ConditionPassed::*;
use execute_aarch32_instrs_VCLZ_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCLZ_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        d: i64,
        elements: i64,
        ga_352081: i64,
        gs_307644: bool,
        gs_307645: bool,
        D: bool,
        size: u8,
        Vd: u8,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
        Q,
        M,
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
        // N s_2_5: branch s_2_4 b15 b3
        if s_2_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Q:u8
        let s_3_0: bool = fn_state.Q;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b11 b4
        if s_3_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#307645 <= s_4_0
        fn_state.gs_307645 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#307645:u8
        let s_5_0: bool = fn_state.gs_307645;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #8s : i64
        let s_6_4: i64 = 8;
        // D s_6_5: lsl s_6_4 s_6_3
        let s_6_5: i64 = s_6_4 << s_6_3;
        // D s_6_6: write-var esize <= s_6_5
        fn_state.esize = s_6_5;
        // C s_6_7: const #64s : i
        let s_6_7: i128 = 64;
        // D s_6_8: read-var esize:i64
        let s_6_8: i64 = fn_state.esize;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: div s_6_7 s_6_9
        let s_6_10: i128 = ((s_6_7) / (s_6_9));
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // D s_6_12: write-var elements <= s_6_11
        fn_state.elements = s_6_11;
        // D s_6_13: read-var D:u8
        let s_6_13: bool = fn_state.D;
        // D s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 1u16);
        // D s_6_15: read-var Vd:u8
        let s_6_15: u8 = fn_state.Vd;
        // D s_6_16: cast zx s_6_15 -> bv
        let s_6_16: Bits = Bits::new(s_6_15 as u128, 4u16);
        // D s_6_17: cast reint s_6_14 -> u128
        let s_6_17: u128 = (s_6_14.value() as u128);
        // D s_6_18: size-of s_6_14
        let s_6_18: u16 = s_6_14.length();
        // D s_6_19: cast reint s_6_16 -> u128
        let s_6_19: u128 = (s_6_16.value() as u128);
        // D s_6_20: size-of s_6_16
        let s_6_20: u16 = s_6_16.length();
        // D s_6_21: lsl s_6_17 s_6_20
        let s_6_21: u128 = s_6_17 << s_6_20;
        // D s_6_22: or s_6_21 s_6_19
        let s_6_22: u128 = ((s_6_21) | (s_6_19));
        // D s_6_23: add s_6_18 s_6_20
        let s_6_23: u16 = (s_6_18 + s_6_20);
        // D s_6_24: create-bits s_6_22 s_6_23
        let s_6_24: Bits = Bits::new(s_6_22, s_6_23);
        // D s_6_25: cast reint s_6_24 -> u8
        let s_6_25: u8 = (s_6_24.value() as u8);
        // D s_6_26: cast zx s_6_25 -> bv
        let s_6_26: Bits = Bits::new(s_6_25 as u128, 5u16);
        // D s_6_27: cast zx s_6_26 -> i
        let s_6_27: i128 = (s_6_26.value() as i128);
        // D s_6_28: cast reint s_6_27 -> i64
        let s_6_28: i64 = (s_6_27 as i64);
        // D s_6_29: write-var d <= s_6_28
        fn_state.d = s_6_28;
        // D s_6_30: read-var M:u8
        let s_6_30: bool = fn_state.M;
        // D s_6_31: cast zx s_6_30 -> bv
        let s_6_31: Bits = Bits::new(s_6_30 as u128, 1u16);
        // D s_6_32: read-var Vm:u8
        let s_6_32: u8 = fn_state.Vm;
        // D s_6_33: cast zx s_6_32 -> bv
        let s_6_33: Bits = Bits::new(s_6_32 as u128, 4u16);
        // D s_6_34: cast reint s_6_31 -> u128
        let s_6_34: u128 = (s_6_31.value() as u128);
        // D s_6_35: size-of s_6_31
        let s_6_35: u16 = s_6_31.length();
        // D s_6_36: cast reint s_6_33 -> u128
        let s_6_36: u128 = (s_6_33.value() as u128);
        // D s_6_37: size-of s_6_33
        let s_6_37: u16 = s_6_33.length();
        // D s_6_38: lsl s_6_34 s_6_37
        let s_6_38: u128 = s_6_34 << s_6_37;
        // D s_6_39: or s_6_38 s_6_36
        let s_6_39: u128 = ((s_6_38) | (s_6_36));
        // D s_6_40: add s_6_35 s_6_37
        let s_6_40: u16 = (s_6_35 + s_6_37);
        // D s_6_41: create-bits s_6_39 s_6_40
        let s_6_41: Bits = Bits::new(s_6_39, s_6_40);
        // D s_6_42: cast reint s_6_41 -> u8
        let s_6_42: u8 = (s_6_41.value() as u8);
        // D s_6_43: cast zx s_6_42 -> bv
        let s_6_43: Bits = Bits::new(s_6_42 as u128, 5u16);
        // D s_6_44: cast zx s_6_43 -> i
        let s_6_44: i128 = (s_6_43.value() as i128);
        // D s_6_45: cast reint s_6_44 -> i64
        let s_6_45: i64 = (s_6_44 as i64);
        // D s_6_46: write-var m <= s_6_45
        fn_state.m = s_6_45;
        // D s_6_47: read-var Q:u8
        let s_6_47: bool = fn_state.Q;
        // D s_6_48: cast zx s_6_47 -> bv
        let s_6_48: Bits = Bits::new(s_6_47 as u128, 1u16);
        // C s_6_49: const #0u : u8
        let s_6_49: bool = false;
        // C s_6_50: cast zx s_6_49 -> bv
        let s_6_50: Bits = Bits::new(s_6_49 as u128, 1u16);
        // D s_6_51: cmp-eq s_6_48 s_6_50
        let s_6_51: bool = ((s_6_48) == (s_6_50));
        // N s_6_52: branch s_6_51 b9 b7
        if s_6_51 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // D s_7_1: write-var ga#352081 <= s_7_0
        fn_state.ga_352081 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#352081:i64
        let s_8_0: i64 = fn_state.ga_352081;
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var elements:i64
        let s_8_4: i64 = fn_state.elements;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var d:i64
        let s_8_6: i64 = fn_state.d;
        // D s_8_7: read-var m:i64
        let s_8_7: i64 = fn_state.m;
        // D s_8_8: call execute_aarch32_instrs_VCLZ_Op_A_txt(s_8_6, s_8_5, s_8_3, s_8_7, s_8_0)
        let s_8_8: () = execute_aarch32_instrs_VCLZ_Op_A_txt(
            state,
            tracer,
            s_8_6,
            s_8_5,
            s_8_3,
            s_8_7,
            s_8_0,
        );
        // N s_8_9: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i64
        let s_9_0: i64 = 1;
        // D s_9_1: write-var ga#352081 <= s_9_0
        fn_state.ga_352081 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var Vd:u8
        let s_11_1: u8 = fn_state.Vd;
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
        // N s_11_22: branch s_11_21 b14 b12
        if s_11_21 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vm:u8
        let s_12_1: u8 = fn_state.Vm;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // D s_12_22: write-var gs#307644 <= s_12_21
        fn_state.gs_307644 = s_12_21;
        // N s_12_23: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#307644:u8
        let s_13_0: bool = fn_state.gs_307644;
        // D s_13_1: write-var gs#307645 <= s_13_0
        fn_state.gs_307645 = s_13_0;
        // N s_13_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#307644 <= s_14_0
        fn_state.gs_307644 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
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
