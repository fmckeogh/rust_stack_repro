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
use execute_aarch32_instrs_LDRH_l_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRH_l_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rt: u8,
    imm4H: u8,
    imm4L: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        imm32: u32,
        wback: bool,
        gs_297553: bool,
        gs_297548: bool,
        gs_297555: bool,
        add: bool,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rt: u8,
        imm4H: u8,
        imm4L: u8,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
        Rt,
        imm4H,
        imm4L,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var P:u8
        let s_2_6: bool = fn_state.P;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #0u : u8
        let s_2_8: bool = false;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b15 b3
        if s_2_10 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#297548 <= s_3_0
        fn_state.gs_297548 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297548:u8
        let s_4_0: bool = fn_state.gs_297548;
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
    ) -> () {
        // D s_5_0: read-var Rt:u8
        let s_5_0: u8 = fn_state.Rt;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var t <= s_5_3
        fn_state.t = s_5_3;
        // D s_5_5: read-var imm4H:u8
        let s_5_5: u8 = fn_state.imm4H;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 4u16);
        // D s_5_7: read-var imm4L:u8
        let s_5_7: u8 = fn_state.imm4L;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 4u16);
        // D s_5_9: cast reint s_5_6 -> u128
        let s_5_9: u128 = (s_5_6.value() as u128);
        // D s_5_10: size-of s_5_6
        let s_5_10: u16 = s_5_6.length();
        // D s_5_11: cast reint s_5_8 -> u128
        let s_5_11: u128 = (s_5_8.value() as u128);
        // D s_5_12: size-of s_5_8
        let s_5_12: u16 = s_5_8.length();
        // D s_5_13: lsl s_5_9 s_5_12
        let s_5_13: u128 = s_5_9 << s_5_12;
        // D s_5_14: or s_5_13 s_5_11
        let s_5_14: u128 = ((s_5_13) | (s_5_11));
        // D s_5_15: add s_5_10 s_5_12
        let s_5_15: u16 = (s_5_10 + s_5_12);
        // D s_5_16: create-bits s_5_14 s_5_15
        let s_5_16: Bits = Bits::new(s_5_14, s_5_15);
        // D s_5_17: cast reint s_5_16 -> u8
        let s_5_17: u8 = (s_5_16.value() as u8);
        // C s_5_18: const #32s : i
        let s_5_18: i128 = 32;
        // D s_5_19: cast zx s_5_17 -> bv
        let s_5_19: Bits = Bits::new(s_5_17 as u128, 8u16);
        // D s_5_20: bits-cast zx s_5_19 -> bv length s_5_18
        let s_5_20: Bits = s_5_19.zero_extend(s_5_18);
        // D s_5_21: cast reint s_5_20 -> u32
        let s_5_21: u32 = (s_5_20.value() as u32);
        // D s_5_22: write-var imm32 <= s_5_21
        fn_state.imm32 = s_5_21;
        // D s_5_23: read-var U:u8
        let s_5_23: bool = fn_state.U;
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 1u16);
        // C s_5_25: const #1u : u8
        let s_5_25: bool = true;
        // C s_5_26: cast zx s_5_25 -> bv
        let s_5_26: Bits = Bits::new(s_5_25 as u128, 1u16);
        // D s_5_27: cmp-eq s_5_24 s_5_26
        let s_5_27: bool = ((s_5_24) == (s_5_26));
        // D s_5_28: write-var add <= s_5_27
        fn_state.add = s_5_27;
        // D s_5_29: read-var P:u8
        let s_5_29: bool = fn_state.P;
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 1u16);
        // C s_5_31: const #0u : u8
        let s_5_31: bool = false;
        // C s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 1u16);
        // D s_5_33: cmp-eq s_5_30 s_5_32
        let s_5_33: bool = ((s_5_30) == (s_5_32));
        // N s_5_34: branch s_5_33 b13 b6
        if s_5_33 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var W:u8
        let s_6_0: bool = fn_state.W;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var gs#297553 <= s_6_4
        fn_state.gs_297553 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#297553:u8
        let s_7_0: bool = fn_state.gs_297553;
        // D s_7_1: write-var wback <= s_7_0
        fn_state.wback = s_7_0;
        // C s_7_2: const #15s : i
        let s_7_2: i128 = 15;
        // D s_7_3: read-var t:i64
        let s_7_3: i64 = fn_state.t;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cmp-eq s_7_4 s_7_2
        let s_7_5: bool = ((s_7_4) == (s_7_2));
        // N s_7_6: branch s_7_5 b12 b8
        if s_7_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var wback:u8
        let s_8_0: bool = fn_state.wback;
        // D s_8_1: write-var gs#297555 <= s_8_0
        fn_state.gs_297555 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#297555:u8
        let s_9_0: bool = fn_state.gs_297555;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var add:u8
        let s_10_0: bool = fn_state.add;
        // D s_10_1: read-var imm32:u32
        let s_10_1: u32 = fn_state.imm32;
        // D s_10_2: read-var t:i64
        let s_10_2: i64 = fn_state.t;
        // D s_10_3: call execute_aarch32_instrs_LDRH_l_Op_A_txt(s_10_0, s_10_1, s_10_2)
        let s_10_3: () = execute_aarch32_instrs_LDRH_l_Op_A_txt(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
        );
        // N s_10_4: return
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
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#297555 <= s_12_0
        fn_state.gs_297555 = s_12_0;
        // N s_12_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#297553 <= s_13_0
        fn_state.gs_297553 = s_13_0;
        // N s_13_2: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_15_0: read-var W:u8
        let s_15_0: bool = fn_state.W;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#297548 <= s_15_4
        fn_state.gs_297548 = s_15_4;
        // N s_15_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
