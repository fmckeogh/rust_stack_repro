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
use execute_aarch32_instrs_VPMAX_f_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VPMAX_f_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    op: bool,
    sz: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        gs_315131: bool,
        elements: i64,
        maximum: bool,
        D: bool,
        op: bool,
        sz: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        op,
        sz,
        Vn,
        Vd,
        N,
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
        // D s_2_0: read-var sz:u8
        let s_2_0: bool = fn_state.sz;
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
        // D s_3_1: write-var gs#315131 <= s_3_0
        fn_state.gs_315131 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#315131:u8
        let s_4_0: bool = fn_state.gs_315131;
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
        // D s_5_0: read-var op:u8
        let s_5_0: bool = fn_state.op;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var maximum <= s_5_4
        fn_state.maximum = s_5_4;
        // C s_5_6: const #16s : i64
        let s_5_6: i64 = 16;
        // D s_5_7: write-var esize <= s_5_6
        fn_state.esize = s_5_6;
        // C s_5_8: const #2s : i64
        let s_5_8: i64 = 2;
        // D s_5_9: write-var elements <= s_5_8
        fn_state.elements = s_5_8;
        // D s_5_10: read-var sz:u8
        let s_5_10: bool = fn_state.sz;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 1u16);
        // C s_5_12: const #0u : u8
        let s_5_12: bool = false;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 1u16);
        // D s_5_14: cmp-eq s_5_11 s_5_13
        let s_5_14: bool = ((s_5_11) == (s_5_13));
        // D s_5_15: not s_5_14
        let s_5_15: bool = !s_5_14;
        // N s_5_16: branch s_5_15 b8 b6
        if s_5_15 {
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
        // C s_6_0: const #32s : i64
        let s_6_0: i64 = 32;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // C s_6_2: const #2s : i64
        let s_6_2: i64 = 2;
        // D s_6_3: write-var elements <= s_6_2
        fn_state.elements = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // D s_7_1: read-var elements:i64
        let s_7_1: i64 = fn_state.elements;
        // D s_7_2: read-var D:u8
        let s_7_2: bool = fn_state.D;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: read-var Vd:u8
        let s_7_4: u8 = fn_state.Vd;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 4u16);
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: cast reint s_7_5 -> u128
        let s_7_8: u128 = (s_7_5.value() as u128);
        // D s_7_9: size-of s_7_5
        let s_7_9: u16 = s_7_5.length();
        // D s_7_10: lsl s_7_6 s_7_9
        let s_7_10: u128 = s_7_6 << s_7_9;
        // D s_7_11: or s_7_10 s_7_8
        let s_7_11: u128 = ((s_7_10) | (s_7_8));
        // D s_7_12: add s_7_7 s_7_9
        let s_7_12: u16 = (s_7_7 + s_7_9);
        // D s_7_13: create-bits s_7_11 s_7_12
        let s_7_13: Bits = Bits::new(s_7_11, s_7_12);
        // D s_7_14: cast reint s_7_13 -> u8
        let s_7_14: u8 = (s_7_13.value() as u8);
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 5u16);
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (s_7_15.value() as i128);
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: read-var N:u8
        let s_7_18: bool = fn_state.N;
        // D s_7_19: cast zx s_7_18 -> bv
        let s_7_19: Bits = Bits::new(s_7_18 as u128, 1u16);
        // D s_7_20: read-var Vn:u8
        let s_7_20: u8 = fn_state.Vn;
        // D s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 4u16);
        // D s_7_22: cast reint s_7_19 -> u128
        let s_7_22: u128 = (s_7_19.value() as u128);
        // D s_7_23: size-of s_7_19
        let s_7_23: u16 = s_7_19.length();
        // D s_7_24: cast reint s_7_21 -> u128
        let s_7_24: u128 = (s_7_21.value() as u128);
        // D s_7_25: size-of s_7_21
        let s_7_25: u16 = s_7_21.length();
        // D s_7_26: lsl s_7_22 s_7_25
        let s_7_26: u128 = s_7_22 << s_7_25;
        // D s_7_27: or s_7_26 s_7_24
        let s_7_27: u128 = ((s_7_26) | (s_7_24));
        // D s_7_28: add s_7_23 s_7_25
        let s_7_28: u16 = (s_7_23 + s_7_25);
        // D s_7_29: create-bits s_7_27 s_7_28
        let s_7_29: Bits = Bits::new(s_7_27, s_7_28);
        // D s_7_30: cast reint s_7_29 -> u8
        let s_7_30: u8 = (s_7_29.value() as u8);
        // D s_7_31: cast zx s_7_30 -> bv
        let s_7_31: Bits = Bits::new(s_7_30 as u128, 5u16);
        // D s_7_32: cast zx s_7_31 -> i
        let s_7_32: i128 = (s_7_31.value() as i128);
        // D s_7_33: cast reint s_7_32 -> i64
        let s_7_33: i64 = (s_7_32 as i64);
        // D s_7_34: read-var M:u8
        let s_7_34: bool = fn_state.M;
        // D s_7_35: cast zx s_7_34 -> bv
        let s_7_35: Bits = Bits::new(s_7_34 as u128, 1u16);
        // D s_7_36: read-var Vm:u8
        let s_7_36: u8 = fn_state.Vm;
        // D s_7_37: cast zx s_7_36 -> bv
        let s_7_37: Bits = Bits::new(s_7_36 as u128, 4u16);
        // D s_7_38: cast reint s_7_35 -> u128
        let s_7_38: u128 = (s_7_35.value() as u128);
        // D s_7_39: size-of s_7_35
        let s_7_39: u16 = s_7_35.length();
        // D s_7_40: cast reint s_7_37 -> u128
        let s_7_40: u128 = (s_7_37.value() as u128);
        // D s_7_41: size-of s_7_37
        let s_7_41: u16 = s_7_37.length();
        // D s_7_42: lsl s_7_38 s_7_41
        let s_7_42: u128 = s_7_38 << s_7_41;
        // D s_7_43: or s_7_42 s_7_40
        let s_7_43: u128 = ((s_7_42) | (s_7_40));
        // D s_7_44: add s_7_39 s_7_41
        let s_7_44: u16 = (s_7_39 + s_7_41);
        // D s_7_45: create-bits s_7_43 s_7_44
        let s_7_45: Bits = Bits::new(s_7_43, s_7_44);
        // D s_7_46: cast reint s_7_45 -> u8
        let s_7_46: u8 = (s_7_45.value() as u8);
        // D s_7_47: cast zx s_7_46 -> bv
        let s_7_47: Bits = Bits::new(s_7_46 as u128, 5u16);
        // D s_7_48: cast zx s_7_47 -> i
        let s_7_48: i128 = (s_7_47.value() as i128);
        // D s_7_49: cast reint s_7_48 -> i64
        let s_7_49: i64 = (s_7_48 as i64);
        // D s_7_50: cast zx s_7_0 -> i
        let s_7_50: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_51: cast reint s_7_50 -> i64
        let s_7_51: i64 = (s_7_50 as i64);
        // D s_7_52: read-var maximum:u8
        let s_7_52: bool = fn_state.maximum;
        // D s_7_53: call execute_aarch32_instrs_VPMAX_f_Op_A_txt(s_7_17, s_7_1, s_7_51, s_7_49, s_7_52, s_7_33)
        let s_7_53: () = execute_aarch32_instrs_VPMAX_f_Op_A_txt(
            state,
            tracer,
            s_7_17,
            s_7_1,
            s_7_51,
            s_7_49,
            s_7_52,
            s_7_33,
        );
        // N s_7_54: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16s : i64
        let s_8_0: i64 = 16;
        // D s_8_1: write-var esize <= s_8_0
        fn_state.esize = s_8_0;
        // C s_8_2: const #4s : i64
        let s_8_2: i64 = 4;
        // D s_8_3: write-var elements <= s_8_2
        fn_state.elements = s_8_2;
        // N s_8_4: jump b7
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
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call InITBlock(s_10_0)
        let s_10_1: bool = InITBlock(state, tracer, s_10_0);
        // D s_10_2: write-var gs#315131 <= s_10_1
        fn_state.gs_315131 = s_10_1;
        // N s_10_3: jump b4
        return block_4(state, tracer, fn_state);
    }
}
