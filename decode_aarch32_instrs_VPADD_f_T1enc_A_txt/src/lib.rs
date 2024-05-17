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
use InITBlock::*;
use execute_aarch32_instrs_VPADD_f_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VPADD_f_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    sz: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        elements: i64,
        gs_314934: bool,
        D: bool,
        sz: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        sz,
        Vn,
        Vd,
        N,
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
        // N s_2_5: branch s_2_4 b12 b3
        if s_2_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var sz:u8
        let s_3_0: bool = fn_state.sz;
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
        // D s_4_1: write-var gs#314934 <= s_4_0
        fn_state.gs_314934 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#314934:u8
        let s_5_0: bool = fn_state.gs_314934;
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
        // C s_6_0: const #16s : i64
        let s_6_0: i64 = 16;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // C s_6_2: const #2s : i64
        let s_6_2: i64 = 2;
        // D s_6_3: write-var elements <= s_6_2
        fn_state.elements = s_6_2;
        // D s_6_4: read-var sz:u8
        let s_6_4: bool = fn_state.sz;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // C s_6_6: const #0u : u8
        let s_6_6: bool = false;
        // C s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 1u16);
        // D s_6_8: cmp-eq s_6_5 s_6_7
        let s_6_8: bool = ((s_6_5) == (s_6_7));
        // D s_6_9: not s_6_8
        let s_6_9: bool = !s_6_8;
        // N s_6_10: branch s_6_9 b9 b7
        if s_6_9 {
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
        // C s_7_0: const #32s : i64
        let s_7_0: i64 = 32;
        // D s_7_1: write-var esize <= s_7_0
        fn_state.esize = s_7_0;
        // C s_7_2: const #2s : i64
        let s_7_2: i64 = 2;
        // D s_7_3: write-var elements <= s_7_2
        fn_state.elements = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // D s_8_1: read-var elements:i64
        let s_8_1: i64 = fn_state.elements;
        // D s_8_2: read-var D:u8
        let s_8_2: bool = fn_state.D;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: read-var Vd:u8
        let s_8_4: u8 = fn_state.Vd;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 4u16);
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: cast reint s_8_5 -> u128
        let s_8_8: u128 = (s_8_5.value() as u128);
        // D s_8_9: size-of s_8_5
        let s_8_9: u16 = s_8_5.length();
        // D s_8_10: lsl s_8_6 s_8_9
        let s_8_10: u128 = s_8_6 << s_8_9;
        // D s_8_11: or s_8_10 s_8_8
        let s_8_11: u128 = ((s_8_10) | (s_8_8));
        // D s_8_12: add s_8_7 s_8_9
        let s_8_12: u16 = (s_8_7 + s_8_9);
        // D s_8_13: create-bits s_8_11 s_8_12
        let s_8_13: Bits = Bits::new(s_8_11, s_8_12);
        // D s_8_14: cast reint s_8_13 -> u8
        let s_8_14: u8 = (s_8_13.value() as u8);
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 5u16);
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (s_8_15.value() as i128);
        // D s_8_17: cast reint s_8_16 -> i64
        let s_8_17: i64 = (s_8_16 as i64);
        // D s_8_18: read-var N:u8
        let s_8_18: bool = fn_state.N;
        // D s_8_19: cast zx s_8_18 -> bv
        let s_8_19: Bits = Bits::new(s_8_18 as u128, 1u16);
        // D s_8_20: read-var Vn:u8
        let s_8_20: u8 = fn_state.Vn;
        // D s_8_21: cast zx s_8_20 -> bv
        let s_8_21: Bits = Bits::new(s_8_20 as u128, 4u16);
        // D s_8_22: cast reint s_8_19 -> u128
        let s_8_22: u128 = (s_8_19.value() as u128);
        // D s_8_23: size-of s_8_19
        let s_8_23: u16 = s_8_19.length();
        // D s_8_24: cast reint s_8_21 -> u128
        let s_8_24: u128 = (s_8_21.value() as u128);
        // D s_8_25: size-of s_8_21
        let s_8_25: u16 = s_8_21.length();
        // D s_8_26: lsl s_8_22 s_8_25
        let s_8_26: u128 = s_8_22 << s_8_25;
        // D s_8_27: or s_8_26 s_8_24
        let s_8_27: u128 = ((s_8_26) | (s_8_24));
        // D s_8_28: add s_8_23 s_8_25
        let s_8_28: u16 = (s_8_23 + s_8_25);
        // D s_8_29: create-bits s_8_27 s_8_28
        let s_8_29: Bits = Bits::new(s_8_27, s_8_28);
        // D s_8_30: cast reint s_8_29 -> u8
        let s_8_30: u8 = (s_8_29.value() as u8);
        // D s_8_31: cast zx s_8_30 -> bv
        let s_8_31: Bits = Bits::new(s_8_30 as u128, 5u16);
        // D s_8_32: cast zx s_8_31 -> i
        let s_8_32: i128 = (s_8_31.value() as i128);
        // D s_8_33: cast reint s_8_32 -> i64
        let s_8_33: i64 = (s_8_32 as i64);
        // D s_8_34: read-var M:u8
        let s_8_34: bool = fn_state.M;
        // D s_8_35: cast zx s_8_34 -> bv
        let s_8_35: Bits = Bits::new(s_8_34 as u128, 1u16);
        // D s_8_36: read-var Vm:u8
        let s_8_36: u8 = fn_state.Vm;
        // D s_8_37: cast zx s_8_36 -> bv
        let s_8_37: Bits = Bits::new(s_8_36 as u128, 4u16);
        // D s_8_38: cast reint s_8_35 -> u128
        let s_8_38: u128 = (s_8_35.value() as u128);
        // D s_8_39: size-of s_8_35
        let s_8_39: u16 = s_8_35.length();
        // D s_8_40: cast reint s_8_37 -> u128
        let s_8_40: u128 = (s_8_37.value() as u128);
        // D s_8_41: size-of s_8_37
        let s_8_41: u16 = s_8_37.length();
        // D s_8_42: lsl s_8_38 s_8_41
        let s_8_42: u128 = s_8_38 << s_8_41;
        // D s_8_43: or s_8_42 s_8_40
        let s_8_43: u128 = ((s_8_42) | (s_8_40));
        // D s_8_44: add s_8_39 s_8_41
        let s_8_44: u16 = (s_8_39 + s_8_41);
        // D s_8_45: create-bits s_8_43 s_8_44
        let s_8_45: Bits = Bits::new(s_8_43, s_8_44);
        // D s_8_46: cast reint s_8_45 -> u8
        let s_8_46: u8 = (s_8_45.value() as u8);
        // D s_8_47: cast zx s_8_46 -> bv
        let s_8_47: Bits = Bits::new(s_8_46 as u128, 5u16);
        // D s_8_48: cast zx s_8_47 -> i
        let s_8_48: i128 = (s_8_47.value() as i128);
        // D s_8_49: cast reint s_8_48 -> i64
        let s_8_49: i64 = (s_8_48 as i64);
        // D s_8_50: cast zx s_8_0 -> i
        let s_8_50: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_51: cast reint s_8_50 -> i64
        let s_8_51: i64 = (s_8_50 as i64);
        // D s_8_52: call execute_aarch32_instrs_VPADD_f_Op_A_txt(s_8_17, s_8_1, s_8_51, s_8_49, s_8_33)
        let s_8_52: () = execute_aarch32_instrs_VPADD_f_Op_A_txt(
            state,
            tracer,
            s_8_17,
            s_8_1,
            s_8_51,
            s_8_49,
            s_8_33,
        );
        // N s_8_53: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16s : i64
        let s_9_0: i64 = 16;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // C s_9_2: const #4s : i64
        let s_9_2: i64 = 4;
        // D s_9_3: write-var elements <= s_9_2
        fn_state.elements = s_9_2;
        // N s_9_4: jump b8
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
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call InITBlock(s_11_0)
        let s_11_1: bool = InITBlock(state, tracer, s_11_0);
        // D s_11_2: write-var gs#314934 <= s_11_1
        fn_state.gs_314934 = s_11_1;
        // N s_11_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
