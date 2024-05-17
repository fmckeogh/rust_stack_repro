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
use execute_aarch32_instrs_STRB_i_OpT_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STRB_i_T3enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rt: u8,
    P: bool,
    U: bool,
    W: bool,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_302356: bool,
        gs_302346: bool,
        t: i64,
        imm32: u32,
        gs_302347: bool,
        n: i64,
        index: bool,
        add: bool,
        gs_302345: bool,
        wback: bool,
        gs_302348: bool,
        gs_302355: bool,
        Rn: u8,
        Rt: u8,
        P: bool,
        U: bool,
        W: bool,
        imm8: u8,
    }
    let fn_state = FunctionState {
        Rn,
        Rt,
        P,
        U,
        W,
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
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b26 b3
        if s_2_4 {
            return block_26(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#302345 <= s_3_0
        fn_state.gs_302345 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#302345:u8
        let s_4_0: bool = fn_state.gs_302345;
        // N s_4_1: branch s_4_0 b25 b5
        if s_4_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#302346 <= s_5_0
        fn_state.gs_302346 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#302346:u8
        let s_6_0: bool = fn_state.gs_302346;
        // N s_6_1: branch s_6_0 b24 b7
        if s_6_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Rn:u8
        let s_7_0: u8 = fn_state.Rn;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 4u16);
        // C s_7_2: const #15u : u8
        let s_7_2: u8 = 15;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b23 b8
        if s_7_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var P:u8
        let s_8_0: bool = fn_state.P;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b22 b9
        if s_8_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#302347 <= s_9_0
        fn_state.gs_302347 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#302347:u8
        let s_10_0: bool = fn_state.gs_302347;
        // D s_10_1: write-var gs#302348 <= s_10_0
        fn_state.gs_302348 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#302348:u8
        let s_11_0: bool = fn_state.gs_302348;
        // N s_11_1: branch s_11_0 b21 b12
        if s_11_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var Rt:u8
        let s_12_0: u8 = fn_state.Rt;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 4u16);
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (s_12_1.value() as i128);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: write-var t <= s_12_3
        fn_state.t = s_12_3;
        // D s_12_5: read-var Rn:u8
        let s_12_5: u8 = fn_state.Rn;
        // D s_12_6: cast zx s_12_5 -> bv
        let s_12_6: Bits = Bits::new(s_12_5 as u128, 4u16);
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (s_12_6.value() as i128);
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: write-var n <= s_12_8
        fn_state.n = s_12_8;
        // C s_12_10: const #32s : i
        let s_12_10: i128 = 32;
        // D s_12_11: read-var imm8:u8
        let s_12_11: u8 = fn_state.imm8;
        // D s_12_12: cast zx s_12_11 -> bv
        let s_12_12: Bits = Bits::new(s_12_11 as u128, 8u16);
        // D s_12_13: bits-cast zx s_12_12 -> bv length s_12_10
        let s_12_13: Bits = s_12_12.zero_extend(s_12_10);
        // D s_12_14: cast reint s_12_13 -> u32
        let s_12_14: u32 = (s_12_13.value() as u32);
        // D s_12_15: write-var imm32 <= s_12_14
        fn_state.imm32 = s_12_14;
        // D s_12_16: read-var P:u8
        let s_12_16: bool = fn_state.P;
        // D s_12_17: cast zx s_12_16 -> bv
        let s_12_17: Bits = Bits::new(s_12_16 as u128, 1u16);
        // C s_12_18: const #1u : u8
        let s_12_18: bool = true;
        // C s_12_19: cast zx s_12_18 -> bv
        let s_12_19: Bits = Bits::new(s_12_18 as u128, 1u16);
        // D s_12_20: cmp-eq s_12_17 s_12_19
        let s_12_20: bool = ((s_12_17) == (s_12_19));
        // D s_12_21: write-var index <= s_12_20
        fn_state.index = s_12_20;
        // D s_12_22: read-var U:u8
        let s_12_22: bool = fn_state.U;
        // D s_12_23: cast zx s_12_22 -> bv
        let s_12_23: Bits = Bits::new(s_12_22 as u128, 1u16);
        // C s_12_24: const #1u : u8
        let s_12_24: bool = true;
        // C s_12_25: cast zx s_12_24 -> bv
        let s_12_25: Bits = Bits::new(s_12_24 as u128, 1u16);
        // D s_12_26: cmp-eq s_12_23 s_12_25
        let s_12_26: bool = ((s_12_23) == (s_12_25));
        // D s_12_27: write-var add <= s_12_26
        fn_state.add = s_12_26;
        // D s_12_28: read-var W:u8
        let s_12_28: bool = fn_state.W;
        // D s_12_29: cast zx s_12_28 -> bv
        let s_12_29: Bits = Bits::new(s_12_28 as u128, 1u16);
        // C s_12_30: const #1u : u8
        let s_12_30: bool = true;
        // C s_12_31: cast zx s_12_30 -> bv
        let s_12_31: Bits = Bits::new(s_12_30 as u128, 1u16);
        // D s_12_32: cmp-eq s_12_29 s_12_31
        let s_12_32: bool = ((s_12_29) == (s_12_31));
        // D s_12_33: write-var wback <= s_12_32
        fn_state.wback = s_12_32;
        // C s_12_34: const #15s : i
        let s_12_34: i128 = 15;
        // D s_12_35: read-var t:i64
        let s_12_35: i64 = fn_state.t;
        // D s_12_36: cast zx s_12_35 -> i
        let s_12_36: i128 = (i128::try_from(s_12_35).unwrap());
        // D s_12_37: cmp-eq s_12_36 s_12_34
        let s_12_37: bool = ((s_12_36) == (s_12_34));
        // N s_12_38: branch s_12_37 b20 b13
        if s_12_37 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var wback:u8
        let s_13_0: bool = fn_state.wback;
        // N s_13_1: branch s_13_0 b19 b14
        if s_13_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#302355 <= s_14_0
        fn_state.gs_302355 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#302355:u8
        let s_15_0: bool = fn_state.gs_302355;
        // D s_15_1: write-var gs#302356 <= s_15_0
        fn_state.gs_302356 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#302356:u8
        let s_16_0: bool = fn_state.gs_302356;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var add:u8
        let s_17_0: bool = fn_state.add;
        // D s_17_1: read-var imm32:u32
        let s_17_1: u32 = fn_state.imm32;
        // D s_17_2: read-var index:u8
        let s_17_2: bool = fn_state.index;
        // D s_17_3: read-var n:i64
        let s_17_3: i64 = fn_state.n;
        // D s_17_4: read-var t:i64
        let s_17_4: i64 = fn_state.t;
        // D s_17_5: read-var wback:u8
        let s_17_5: bool = fn_state.wback;
        // D s_17_6: call execute_aarch32_instrs_STRB_i_OpT_A_txt(s_17_0, s_17_1, s_17_2, s_17_3, s_17_4, s_17_5)
        let s_17_6: () = execute_aarch32_instrs_STRB_i_OpT_A_txt(
            state,
            tracer,
            s_17_0,
            s_17_1,
            s_17_2,
            s_17_3,
            s_17_4,
            s_17_5,
        );
        // N s_17_7: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var n:i64
        let s_19_0: i64 = fn_state.n;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var t:i64
        let s_19_2: i64 = fn_state.t;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#302355 <= s_19_4
        fn_state.gs_302355 = s_19_4;
        // N s_19_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#302356 <= s_20_0
        fn_state.gs_302356 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var W:u8
        let s_22_0: bool = fn_state.W;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#302347 <= s_22_4
        fn_state.gs_302347 = s_22_4;
        // N s_22_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#302348 <= s_23_0
        fn_state.gs_302348 = s_23_0;
        // N s_23_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var W:u8
        let s_25_0: bool = fn_state.W;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#302346 <= s_25_4
        fn_state.gs_302346 = s_25_4;
        // N s_25_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var U:u8
        let s_26_0: bool = fn_state.U;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#302345 <= s_26_4
        fn_state.gs_302345 = s_26_4;
        // N s_26_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
