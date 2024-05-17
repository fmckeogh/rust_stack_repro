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
use execute_aarch32_instrs_VMOVL_Op_A_txt::*;
use fdiv_int::*;
use common::*;
pub fn decode_aarch32_instrs_VMOVL_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    imm3H: u8,
    Vd: u8,
    L: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_313092: bool,
        gs_313093: bool,
        U: bool,
        D: bool,
        imm3H: u8,
        Vd: u8,
        L: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        imm3H,
        Vd,
        L,
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
        // D s_2_0: read-var imm3H:u8
        let s_2_0: u8 = fn_state.imm3H;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 3u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 3u16);
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
        // D s_3_0: read-var imm3H:u8
        let s_3_0: u8 = fn_state.imm3H;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 3u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 3u16);
        // D s_3_4: cmp-ne s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) != (s_3_3));
        // N s_3_5: branch s_3_4 b13 b4
        if s_3_4 {
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
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#313092 <= s_4_0
        fn_state.gs_313092 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#313092:u8
        let s_5_0: bool = fn_state.gs_313092;
        // N s_5_1: branch s_5_0 b12 b6
        if s_5_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#313093 <= s_6_0
        fn_state.gs_313093 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#313093:u8
        let s_7_0: bool = fn_state.gs_313093;
        // N s_7_1: branch s_7_0 b11 b8
        if s_7_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var Vd:u8
        let s_8_1: u8 = fn_state.Vd;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 4u16);
        // C s_8_3: const #1u : u64
        let s_8_3: u64 = 1;
        // D s_8_4: bit-extract s_8_2 s_8_0 s_8_3
        let s_8_4: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // C s_8_7: const #0u : u64
        let s_8_7: u64 = 0;
        // D s_8_8: cast zx s_8_5 -> u64
        let s_8_8: u64 = (s_8_5 as u64);
        // C s_8_9: const #1u : u64
        let s_8_9: u64 = 1;
        // D s_8_10: and s_8_8 s_8_9
        let s_8_10: u64 = ((s_8_8) & (s_8_9));
        // D s_8_11: cmp-eq s_8_10 s_8_9
        let s_8_11: bool = ((s_8_10) == (s_8_9));
        // D s_8_12: lsl s_8_8 s_8_6
        let s_8_12: u64 = s_8_8 << s_8_6;
        // D s_8_13: or s_8_7 s_8_12
        let s_8_13: u64 = ((s_8_7) | (s_8_12));
        // D s_8_14: cmpl s_8_12
        let s_8_14: u64 = !s_8_12;
        // D s_8_15: and s_8_7 s_8_14
        let s_8_15: u64 = ((s_8_7) & (s_8_14));
        // D s_8_16: select s_8_11 s_8_13 s_8_15
        let s_8_16: u64 = if s_8_11 { s_8_13 } else { s_8_15 };
        // D s_8_17: cast trunc s_8_16 -> u8
        let s_8_17: bool = ((s_8_16) != 0);
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // C s_8_19: const #1u : u8
        let s_8_19: bool = true;
        // C s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_21: cmp-eq s_8_18 s_8_20
        let s_8_21: bool = ((s_8_18) == (s_8_20));
        // N s_8_22: branch s_8_21 b10 b9
        if s_8_21 {
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
        // D s_9_0: read-var imm3H:u8
        let s_9_0: u8 = fn_state.imm3H;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 3u16);
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #8s : i
        let s_9_4: i128 = 8;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: mul s_9_4 s_9_5
        let s_9_6: i128 = ((s_9_4) * (s_9_5));
        // D s_9_7: cast reint s_9_6 -> i64
        let s_9_7: i64 = (s_9_6 as i64);
        // D s_9_8: read-var U:u8
        let s_9_8: bool = fn_state.U;
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 1u16);
        // C s_9_10: const #1u : u8
        let s_9_10: bool = true;
        // C s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 1u16);
        // D s_9_12: cmp-eq s_9_9 s_9_11
        let s_9_12: bool = ((s_9_9) == (s_9_11));
        // C s_9_13: const #64s : i
        let s_9_13: i128 = 64;
        // D s_9_14: cast zx s_9_7 -> i
        let s_9_14: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_15: call fdiv_int(s_9_13, s_9_14)
        let s_9_15: i128 = fdiv_int(state, tracer, s_9_13, s_9_14);
        // D s_9_16: read-var D:u8
        let s_9_16: bool = fn_state.D;
        // D s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 1u16);
        // D s_9_18: read-var Vd:u8
        let s_9_18: u8 = fn_state.Vd;
        // D s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 4u16);
        // D s_9_20: cast reint s_9_17 -> u128
        let s_9_20: u128 = (s_9_17.value() as u128);
        // D s_9_21: size-of s_9_17
        let s_9_21: u16 = s_9_17.length();
        // D s_9_22: cast reint s_9_19 -> u128
        let s_9_22: u128 = (s_9_19.value() as u128);
        // D s_9_23: size-of s_9_19
        let s_9_23: u16 = s_9_19.length();
        // D s_9_24: lsl s_9_20 s_9_23
        let s_9_24: u128 = s_9_20 << s_9_23;
        // D s_9_25: or s_9_24 s_9_22
        let s_9_25: u128 = ((s_9_24) | (s_9_22));
        // D s_9_26: add s_9_21 s_9_23
        let s_9_26: u16 = (s_9_21 + s_9_23);
        // D s_9_27: create-bits s_9_25 s_9_26
        let s_9_27: Bits = Bits::new(s_9_25, s_9_26);
        // D s_9_28: cast reint s_9_27 -> u8
        let s_9_28: u8 = (s_9_27.value() as u8);
        // D s_9_29: cast zx s_9_28 -> bv
        let s_9_29: Bits = Bits::new(s_9_28 as u128, 5u16);
        // D s_9_30: cast zx s_9_29 -> i
        let s_9_30: i128 = (s_9_29.value() as i128);
        // D s_9_31: cast reint s_9_30 -> i64
        let s_9_31: i64 = (s_9_30 as i64);
        // D s_9_32: read-var M:u8
        let s_9_32: bool = fn_state.M;
        // D s_9_33: cast zx s_9_32 -> bv
        let s_9_33: Bits = Bits::new(s_9_32 as u128, 1u16);
        // D s_9_34: read-var Vm:u8
        let s_9_34: u8 = fn_state.Vm;
        // D s_9_35: cast zx s_9_34 -> bv
        let s_9_35: Bits = Bits::new(s_9_34 as u128, 4u16);
        // D s_9_36: cast reint s_9_33 -> u128
        let s_9_36: u128 = (s_9_33.value() as u128);
        // D s_9_37: size-of s_9_33
        let s_9_37: u16 = s_9_33.length();
        // D s_9_38: cast reint s_9_35 -> u128
        let s_9_38: u128 = (s_9_35.value() as u128);
        // D s_9_39: size-of s_9_35
        let s_9_39: u16 = s_9_35.length();
        // D s_9_40: lsl s_9_36 s_9_39
        let s_9_40: u128 = s_9_36 << s_9_39;
        // D s_9_41: or s_9_40 s_9_38
        let s_9_41: u128 = ((s_9_40) | (s_9_38));
        // D s_9_42: add s_9_37 s_9_39
        let s_9_42: u16 = (s_9_37 + s_9_39);
        // D s_9_43: create-bits s_9_41 s_9_42
        let s_9_43: Bits = Bits::new(s_9_41, s_9_42);
        // D s_9_44: cast reint s_9_43 -> u8
        let s_9_44: u8 = (s_9_43.value() as u8);
        // D s_9_45: cast zx s_9_44 -> bv
        let s_9_45: Bits = Bits::new(s_9_44 as u128, 5u16);
        // D s_9_46: cast zx s_9_45 -> i
        let s_9_46: i128 = (s_9_45.value() as i128);
        // D s_9_47: cast reint s_9_46 -> i64
        let s_9_47: i64 = (s_9_46 as i64);
        // D s_9_48: cast zx s_9_7 -> i
        let s_9_48: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_49: cast reint s_9_48 -> i64
        let s_9_49: i64 = (s_9_48 as i64);
        // D s_9_50: call execute_aarch32_instrs_VMOVL_Op_A_txt(s_9_31, s_9_15, s_9_49, s_9_47, s_9_12)
        let s_9_50: () = execute_aarch32_instrs_VMOVL_Op_A_txt(
            state,
            tracer,
            s_9_31,
            s_9_15,
            s_9_49,
            s_9_47,
            s_9_12,
        );
        // N s_9_51: return
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
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var imm3H:u8
        let s_12_0: u8 = fn_state.imm3H;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 3u16);
        // C s_12_2: const #4u : u8
        let s_12_2: u8 = 4;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 3u16);
        // D s_12_4: cmp-ne s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) != (s_12_3));
        // D s_12_5: write-var gs#313093 <= s_12_4
        fn_state.gs_313093 = s_12_4;
        // N s_12_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var imm3H:u8
        let s_13_0: u8 = fn_state.imm3H;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 3u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 3u16);
        // D s_13_4: cmp-ne s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) != (s_13_3));
        // D s_13_5: write-var gs#313092 <= s_13_4
        fn_state.gs_313092 = s_13_4;
        // N s_13_6: jump b5
        return block_5(state, tracer, fn_state);
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
