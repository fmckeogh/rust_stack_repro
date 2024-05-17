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
use HaveAESExt::*;
use execute_aarch32_instrs_AESMC_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_AESMC_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323855: bool,
        D: bool,
        size: u8,
        Vd: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveAESExt(s_2_0)
        let s_2_1: bool = HaveAESExt(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b11 b3
        if s_2_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-ne s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) != (s_3_3));
        // N s_3_5: branch s_3_4 b10 b4
        if s_3_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var Vd:u8
        let s_4_1: u8 = fn_state.Vd;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // N s_4_22: branch s_4_21 b9 b5
        if s_4_21 {
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
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var Vm:u8
        let s_5_1: u8 = fn_state.Vm;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 4u16);
        // C s_5_3: const #1u : u64
        let s_5_3: u64 = 1;
        // D s_5_4: bit-extract s_5_2 s_5_0 s_5_3
        let s_5_4: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_3).unwrap(),
        ));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: bool = ((s_5_4.value()) != 0);
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // C s_5_7: const #0u : u64
        let s_5_7: u64 = 0;
        // D s_5_8: cast zx s_5_5 -> u64
        let s_5_8: u64 = (s_5_5 as u64);
        // C s_5_9: const #1u : u64
        let s_5_9: u64 = 1;
        // D s_5_10: and s_5_8 s_5_9
        let s_5_10: u64 = ((s_5_8) & (s_5_9));
        // D s_5_11: cmp-eq s_5_10 s_5_9
        let s_5_11: bool = ((s_5_10) == (s_5_9));
        // D s_5_12: lsl s_5_8 s_5_6
        let s_5_12: u64 = s_5_8 << s_5_6;
        // D s_5_13: or s_5_7 s_5_12
        let s_5_13: u64 = ((s_5_7) | (s_5_12));
        // D s_5_14: cmpl s_5_12
        let s_5_14: u64 = !s_5_12;
        // D s_5_15: and s_5_7 s_5_14
        let s_5_15: u64 = ((s_5_7) & (s_5_14));
        // D s_5_16: select s_5_11 s_5_13 s_5_15
        let s_5_16: u64 = if s_5_11 { s_5_13 } else { s_5_15 };
        // D s_5_17: cast trunc s_5_16 -> u8
        let s_5_17: bool = ((s_5_16) != 0);
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // C s_5_19: const #1u : u8
        let s_5_19: bool = true;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // D s_5_22: write-var gs#323855 <= s_5_21
        fn_state.gs_323855 = s_5_21;
        // N s_5_23: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#323855:u8
        let s_6_0: bool = fn_state.gs_323855;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var D:u8
        let s_7_0: bool = fn_state.D;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // D s_7_2: read-var Vd:u8
        let s_7_2: u8 = fn_state.Vd;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: cast reint s_7_1 -> u128
        let s_7_4: u128 = (s_7_1.value() as u128);
        // D s_7_5: size-of s_7_1
        let s_7_5: u16 = s_7_1.length();
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: lsl s_7_4 s_7_7
        let s_7_8: u128 = s_7_4 << s_7_7;
        // D s_7_9: or s_7_8 s_7_6
        let s_7_9: u128 = ((s_7_8) | (s_7_6));
        // D s_7_10: add s_7_5 s_7_7
        let s_7_10: u16 = (s_7_5 + s_7_7);
        // D s_7_11: create-bits s_7_9 s_7_10
        let s_7_11: Bits = Bits::new(s_7_9, s_7_10);
        // D s_7_12: cast reint s_7_11 -> u8
        let s_7_12: u8 = (s_7_11.value() as u8);
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 5u16);
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (s_7_13.value() as i128);
        // D s_7_15: cast reint s_7_14 -> i64
        let s_7_15: i64 = (s_7_14 as i64);
        // D s_7_16: read-var M:u8
        let s_7_16: bool = fn_state.M;
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 1u16);
        // D s_7_18: read-var Vm:u8
        let s_7_18: u8 = fn_state.Vm;
        // D s_7_19: cast zx s_7_18 -> bv
        let s_7_19: Bits = Bits::new(s_7_18 as u128, 4u16);
        // D s_7_20: cast reint s_7_17 -> u128
        let s_7_20: u128 = (s_7_17.value() as u128);
        // D s_7_21: size-of s_7_17
        let s_7_21: u16 = s_7_17.length();
        // D s_7_22: cast reint s_7_19 -> u128
        let s_7_22: u128 = (s_7_19.value() as u128);
        // D s_7_23: size-of s_7_19
        let s_7_23: u16 = s_7_19.length();
        // D s_7_24: lsl s_7_20 s_7_23
        let s_7_24: u128 = s_7_20 << s_7_23;
        // D s_7_25: or s_7_24 s_7_22
        let s_7_25: u128 = ((s_7_24) | (s_7_22));
        // D s_7_26: add s_7_21 s_7_23
        let s_7_26: u16 = (s_7_21 + s_7_23);
        // D s_7_27: create-bits s_7_25 s_7_26
        let s_7_27: Bits = Bits::new(s_7_25, s_7_26);
        // D s_7_28: cast reint s_7_27 -> u8
        let s_7_28: u8 = (s_7_27.value() as u8);
        // D s_7_29: cast zx s_7_28 -> bv
        let s_7_29: Bits = Bits::new(s_7_28 as u128, 5u16);
        // D s_7_30: cast zx s_7_29 -> i
        let s_7_30: i128 = (s_7_29.value() as i128);
        // D s_7_31: cast reint s_7_30 -> i64
        let s_7_31: i64 = (s_7_30 as i64);
        // D s_7_32: call execute_aarch32_instrs_AESMC_Op_A_txt(s_7_15, s_7_31)
        let s_7_32: () = execute_aarch32_instrs_AESMC_Op_A_txt(
            state,
            tracer,
            s_7_15,
            s_7_31,
        );
        // N s_7_33: return
        return;
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
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#323855 <= s_9_0
        fn_state.gs_323855 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
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
}
