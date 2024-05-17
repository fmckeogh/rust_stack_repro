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
use place_slice::*;
use execute_aarch32_instrs_LDRD_l_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRD_l_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    P: bool,
    U: bool,
    W: bool,
    Rt: u8,
    Rt2: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        imm32: u32,
        t2: i64,
        gs_297306: bool,
        gs_297305: bool,
        gs_297295: bool,
        add: bool,
        P: bool,
        U: bool,
        W: bool,
        Rt: u8,
        Rt2: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        P,
        U,
        W,
        Rt,
        Rt2,
        imm8,
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
        // D s_2_0: read-var P:u8
        let s_2_0: bool = fn_state.P;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b17 b3
        if s_2_4 {
            return block_17(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#297295 <= s_3_0
        fn_state.gs_297295 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297295:u8
        let s_4_0: bool = fn_state.gs_297295;
        // N s_4_1: branch s_4_0 b16 b5
        if s_4_0 {
            return block_16(state, tracer, fn_state);
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
        // D s_5_5: read-var Rt2:u8
        let s_5_5: u8 = fn_state.Rt2;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 4u16);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (s_5_6.value() as i128);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: write-var t2 <= s_5_8
        fn_state.t2 = s_5_8;
        // C s_5_10: const #32s : i
        let s_5_10: i128 = 32;
        // C s_5_11: const #0s : i
        let s_5_11: i128 = 0;
        // C s_5_12: const #8s : i
        let s_5_12: i128 = 8;
        // C s_5_13: const #2s : i
        let s_5_13: i128 = 2;
        // D s_5_14: read-var imm8:u8
        let s_5_14: u8 = fn_state.imm8;
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 8u16);
        // D s_5_16: call place_slice(s_5_10, s_5_15, s_5_11, s_5_12, s_5_13)
        let s_5_16: Bits = place_slice(
            state,
            tracer,
            s_5_10,
            s_5_15,
            s_5_11,
            s_5_12,
            s_5_13,
        );
        // D s_5_17: cast reint s_5_16 -> u32
        let s_5_17: u32 = (s_5_16.value() as u32);
        // D s_5_18: write-var imm32 <= s_5_17
        fn_state.imm32 = s_5_17;
        // D s_5_19: read-var U:u8
        let s_5_19: bool = fn_state.U;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // C s_5_21: const #1u : u8
        let s_5_21: bool = true;
        // C s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // D s_5_23: cmp-eq s_5_20 s_5_22
        let s_5_23: bool = ((s_5_20) == (s_5_22));
        // D s_5_24: write-var add <= s_5_23
        fn_state.add = s_5_23;
        // C s_5_25: const #15s : i
        let s_5_25: i128 = 15;
        // D s_5_26: read-var t:i64
        let s_5_26: i64 = fn_state.t;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: cmp-eq s_5_27 s_5_25
        let s_5_28: bool = ((s_5_27) == (s_5_25));
        // N s_5_29: branch s_5_28 b15 b6
        if s_5_28 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #15s : i
        let s_6_0: i128 = 15;
        // D s_6_1: read-var t2:i64
        let s_6_1: i64 = fn_state.t2;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // D s_6_4: write-var gs#297305 <= s_6_3
        fn_state.gs_297305 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#297305:u8
        let s_7_0: bool = fn_state.gs_297305;
        // N s_7_1: branch s_7_0 b14 b8
        if s_7_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var t:i64
        let s_8_0: i64 = fn_state.t;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var t2:i64
        let s_8_2: i64 = fn_state.t2;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: write-var gs#297306 <= s_8_4
        fn_state.gs_297306 = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#297306:u8
        let s_9_0: bool = fn_state.gs_297306;
        // N s_9_1: branch s_9_0 b13 b10
        if s_9_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var W:u8
        let s_10_0: bool = fn_state.W;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var add:u8
        let s_11_0: bool = fn_state.add;
        // D s_11_1: read-var imm32:u32
        let s_11_1: u32 = fn_state.imm32;
        // D s_11_2: read-var t:i64
        let s_11_2: i64 = fn_state.t;
        // D s_11_3: read-var t2:i64
        let s_11_3: i64 = fn_state.t2;
        // D s_11_4: call execute_aarch32_instrs_LDRD_l_Op_A_txt(s_11_0, s_11_1, s_11_2, s_11_3)
        let s_11_4: () = execute_aarch32_instrs_LDRD_l_Op_A_txt(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
            s_11_3,
        );
        // N s_11_5: return
        return;
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
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#297306 <= s_14_0
        fn_state.gs_297306 = s_14_0;
        // N s_14_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#297305 <= s_15_0
        fn_state.gs_297305 = s_15_0;
        // N s_15_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var W:u8
        let s_17_0: bool = fn_state.W;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #0u : u8
        let s_17_2: bool = false;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#297295 <= s_17_4
        fn_state.gs_297295 = s_17_4;
        // N s_17_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
