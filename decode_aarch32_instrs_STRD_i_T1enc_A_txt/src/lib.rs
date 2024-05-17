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
use execute_aarch32_instrs_STRD_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STRD_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    Rt: u8,
    Rt2: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_302524: bool,
        t: i64,
        imm32: u32,
        t2: i64,
        gs_302533: bool,
        n: i64,
        index: bool,
        add: bool,
        wback: bool,
        gs_302537: bool,
        gs_302539: bool,
        gs_302534: bool,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        Rt: u8,
        Rt2: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        P,
        U,
        W,
        Rn,
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
        // N s_2_5: branch s_2_4 b23 b3
        if s_2_4 {
            return block_23(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#302524 <= s_3_0
        fn_state.gs_302524 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#302524:u8
        let s_4_0: bool = fn_state.gs_302524;
        // N s_4_1: branch s_4_0 b22 b5
        if s_4_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_5_10: read-var Rn:u8
        let s_5_10: u8 = fn_state.Rn;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 4u16);
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (s_5_11.value() as i128);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var n <= s_5_13
        fn_state.n = s_5_13;
        // C s_5_15: const #32s : i
        let s_5_15: i128 = 32;
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // C s_5_17: const #8s : i
        let s_5_17: i128 = 8;
        // C s_5_18: const #2s : i
        let s_5_18: i128 = 2;
        // D s_5_19: read-var imm8:u8
        let s_5_19: u8 = fn_state.imm8;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 8u16);
        // D s_5_21: call place_slice(s_5_15, s_5_20, s_5_16, s_5_17, s_5_18)
        let s_5_21: Bits = place_slice(
            state,
            tracer,
            s_5_15,
            s_5_20,
            s_5_16,
            s_5_17,
            s_5_18,
        );
        // D s_5_22: cast reint s_5_21 -> u32
        let s_5_22: u32 = (s_5_21.value() as u32);
        // D s_5_23: write-var imm32 <= s_5_22
        fn_state.imm32 = s_5_22;
        // D s_5_24: read-var P:u8
        let s_5_24: bool = fn_state.P;
        // D s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 1u16);
        // C s_5_26: const #1u : u8
        let s_5_26: bool = true;
        // C s_5_27: cast zx s_5_26 -> bv
        let s_5_27: Bits = Bits::new(s_5_26 as u128, 1u16);
        // D s_5_28: cmp-eq s_5_25 s_5_27
        let s_5_28: bool = ((s_5_25) == (s_5_27));
        // D s_5_29: write-var index <= s_5_28
        fn_state.index = s_5_28;
        // D s_5_30: read-var U:u8
        let s_5_30: bool = fn_state.U;
        // D s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 1u16);
        // C s_5_32: const #1u : u8
        let s_5_32: bool = true;
        // C s_5_33: cast zx s_5_32 -> bv
        let s_5_33: Bits = Bits::new(s_5_32 as u128, 1u16);
        // D s_5_34: cmp-eq s_5_31 s_5_33
        let s_5_34: bool = ((s_5_31) == (s_5_33));
        // D s_5_35: write-var add <= s_5_34
        fn_state.add = s_5_34;
        // D s_5_36: read-var W:u8
        let s_5_36: bool = fn_state.W;
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 1u16);
        // C s_5_38: const #1u : u8
        let s_5_38: bool = true;
        // C s_5_39: cast zx s_5_38 -> bv
        let s_5_39: Bits = Bits::new(s_5_38 as u128, 1u16);
        // D s_5_40: cmp-eq s_5_37 s_5_39
        let s_5_40: bool = ((s_5_37) == (s_5_39));
        // D s_5_41: write-var wback <= s_5_40
        fn_state.wback = s_5_40;
        // D s_5_42: read-var wback:u8
        let s_5_42: bool = fn_state.wback;
        // N s_5_43: branch s_5_42 b18 b6
        if s_5_42 {
            return block_18(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#302534 <= s_6_0
        fn_state.gs_302534 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#302534:u8
        let s_7_0: bool = fn_state.gs_302534;
        // N s_7_1: branch s_7_0 b17 b8
        if s_7_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #15s : i
        let s_8_0: i128 = 15;
        // D s_8_1: read-var n:i64
        let s_8_1: i64 = fn_state.n;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // N s_8_4: branch s_8_3 b16 b9
        if s_8_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #15s : i
        let s_9_0: i128 = 15;
        // D s_9_1: read-var t:i64
        let s_9_1: i64 = fn_state.t;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) == (s_9_0));
        // D s_9_4: write-var gs#302537 <= s_9_3
        fn_state.gs_302537 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#302537:u8
        let s_10_0: bool = fn_state.gs_302537;
        // N s_10_1: branch s_10_0 b15 b11
        if s_10_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #15s : i
        let s_11_0: i128 = 15;
        // D s_11_1: read-var t2:i64
        let s_11_1: i64 = fn_state.t2;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_0
        let s_11_3: bool = ((s_11_2) == (s_11_0));
        // D s_11_4: write-var gs#302539 <= s_11_3
        fn_state.gs_302539 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#302539:u8
        let s_12_0: bool = fn_state.gs_302539;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var add:u8
        let s_13_0: bool = fn_state.add;
        // D s_13_1: read-var imm32:u32
        let s_13_1: u32 = fn_state.imm32;
        // D s_13_2: read-var index:u8
        let s_13_2: bool = fn_state.index;
        // D s_13_3: read-var n:i64
        let s_13_3: i64 = fn_state.n;
        // D s_13_4: read-var t:i64
        let s_13_4: i64 = fn_state.t;
        // D s_13_5: read-var t2:i64
        let s_13_5: i64 = fn_state.t2;
        // D s_13_6: read-var wback:u8
        let s_13_6: bool = fn_state.wback;
        // D s_13_7: call execute_aarch32_instrs_STRD_i_Op_A_txt(s_13_0, s_13_1, s_13_2, s_13_3, s_13_4, s_13_5, s_13_6)
        let s_13_7: () = execute_aarch32_instrs_STRD_i_Op_A_txt(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_6,
        );
        // N s_13_8: return
        return;
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
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#302539 <= s_15_0
        fn_state.gs_302539 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#302537 <= s_16_0
        fn_state.gs_302537 = s_16_0;
        // N s_16_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: read-var t:i64
        let s_18_2: i64 = fn_state.t;
        // D s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b21 b19
        if s_18_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
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
        // D s_19_2: read-var t2:i64
        let s_19_2: i64 = fn_state.t2;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#302533 <= s_19_4
        fn_state.gs_302533 = s_19_4;
        // N s_19_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#302533:u8
        let s_20_0: bool = fn_state.gs_302533;
        // D s_20_1: write-var gs#302534 <= s_20_0
        fn_state.gs_302534 = s_20_0;
        // N s_20_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#302533 <= s_21_0
        fn_state.gs_302533 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var W:u8
        let s_23_0: bool = fn_state.W;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #0u : u8
        let s_23_2: bool = false;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#302524 <= s_23_4
        fn_state.gs_302524 = s_23_4;
        // N s_23_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
