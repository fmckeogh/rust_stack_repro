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
use execute_aarch32_instrs_STRD_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STRD_r_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    Rt: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        t2: i64,
        gs_302581: bool,
        gs_302574: bool,
        gs_302575: bool,
        gs_302582: bool,
        gs_302580: bool,
        n: i64,
        index: bool,
        add: bool,
        wback: bool,
        gs_302578: bool,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        Rt: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
        Rn,
        Rt,
        Rm,
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
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // D s_2_7: read-var Rt:u8
        let s_2_7: u8 = fn_state.Rt;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 4u16);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: bit-extract s_2_8 s_2_6 s_2_9
        let s_2_10: Bits = (Bits::new(
            ((s_2_8) >> (s_2_6)).value(),
            u16::try_from(s_2_9).unwrap(),
        ));
        // D s_2_11: cast reint s_2_10 -> u8
        let s_2_11: bool = ((s_2_10.value()) != 0);
        // C s_2_12: const #0s : i
        let s_2_12: i128 = 0;
        // C s_2_13: const #0u : u64
        let s_2_13: u64 = 0;
        // D s_2_14: cast zx s_2_11 -> u64
        let s_2_14: u64 = (s_2_11 as u64);
        // C s_2_15: const #1u : u64
        let s_2_15: u64 = 1;
        // D s_2_16: and s_2_14 s_2_15
        let s_2_16: u64 = ((s_2_14) & (s_2_15));
        // D s_2_17: cmp-eq s_2_16 s_2_15
        let s_2_17: bool = ((s_2_16) == (s_2_15));
        // D s_2_18: lsl s_2_14 s_2_12
        let s_2_18: u64 = s_2_14 << s_2_12;
        // D s_2_19: or s_2_13 s_2_18
        let s_2_19: u64 = ((s_2_13) | (s_2_18));
        // D s_2_20: cmpl s_2_18
        let s_2_20: u64 = !s_2_18;
        // D s_2_21: and s_2_13 s_2_20
        let s_2_21: u64 = ((s_2_13) & (s_2_20));
        // D s_2_22: select s_2_17 s_2_19 s_2_21
        let s_2_22: u64 = if s_2_17 { s_2_19 } else { s_2_21 };
        // D s_2_23: cast trunc s_2_22 -> u8
        let s_2_23: bool = ((s_2_22) != 0);
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 1u16);
        // C s_2_25: const #1u : u8
        let s_2_25: bool = true;
        // C s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 1u16);
        // D s_2_27: cmp-eq s_2_24 s_2_26
        let s_2_27: bool = ((s_2_24) == (s_2_26));
        // N s_2_28: branch s_2_27 b28 b3
        if s_2_27 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var t <= s_3_3
        fn_state.t = s_3_3;
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // D s_3_6: read-var t:i64
        let s_3_6: i64 = fn_state.t;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: add s_3_7 s_3_5
        let s_3_8: i128 = (s_3_7 + s_3_5);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var t2 <= s_3_9
        fn_state.t2 = s_3_9;
        // D s_3_11: read-var Rn:u8
        let s_3_11: u8 = fn_state.Rn;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 4u16);
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (s_3_12.value() as i128);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: write-var n <= s_3_14
        fn_state.n = s_3_14;
        // D s_3_16: read-var Rm:u8
        let s_3_16: u8 = fn_state.Rm;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 4u16);
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (s_3_17.value() as i128);
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: write-var m <= s_3_19
        fn_state.m = s_3_19;
        // D s_3_21: read-var P:u8
        let s_3_21: bool = fn_state.P;
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 1u16);
        // C s_3_23: const #1u : u8
        let s_3_23: bool = true;
        // C s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 1u16);
        // D s_3_25: cmp-eq s_3_22 s_3_24
        let s_3_25: bool = ((s_3_22) == (s_3_24));
        // D s_3_26: write-var index <= s_3_25
        fn_state.index = s_3_25;
        // D s_3_27: read-var U:u8
        let s_3_27: bool = fn_state.U;
        // D s_3_28: cast zx s_3_27 -> bv
        let s_3_28: Bits = Bits::new(s_3_27 as u128, 1u16);
        // C s_3_29: const #1u : u8
        let s_3_29: bool = true;
        // C s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 1u16);
        // D s_3_31: cmp-eq s_3_28 s_3_30
        let s_3_31: bool = ((s_3_28) == (s_3_30));
        // D s_3_32: write-var add <= s_3_31
        fn_state.add = s_3_31;
        // D s_3_33: read-var P:u8
        let s_3_33: bool = fn_state.P;
        // D s_3_34: cast zx s_3_33 -> bv
        let s_3_34: Bits = Bits::new(s_3_33 as u128, 1u16);
        // C s_3_35: const #0u : u8
        let s_3_35: bool = false;
        // C s_3_36: cast zx s_3_35 -> bv
        let s_3_36: Bits = Bits::new(s_3_35 as u128, 1u16);
        // D s_3_37: cmp-eq s_3_34 s_3_36
        let s_3_37: bool = ((s_3_34) == (s_3_36));
        // N s_3_38: branch s_3_37 b27 b4
        if s_3_37 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var W:u8
        let s_4_0: bool = fn_state.W;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var gs#302574 <= s_4_4
        fn_state.gs_302574 = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#302574:u8
        let s_5_0: bool = fn_state.gs_302574;
        // D s_5_1: write-var wback <= s_5_0
        fn_state.wback = s_5_0;
        // D s_5_2: read-var P:u8
        let s_5_2: bool = fn_state.P;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b26 b6
        if s_5_6 {
            return block_26(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#302575 <= s_6_0
        fn_state.gs_302575 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#302575:u8
        let s_7_0: bool = fn_state.gs_302575;
        // N s_7_1: branch s_7_0 b25 b8
        if s_7_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_8_1: read-var t2:i64
        let s_8_1: i64 = fn_state.t2;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // N s_8_4: branch s_8_3 b24 b9
        if s_8_3 {
            return block_24(state, tracer, fn_state);
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
        // D s_9_1: read-var m:i64
        let s_9_1: i64 = fn_state.m;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) == (s_9_0));
        // D s_9_4: write-var gs#302578 <= s_9_3
        fn_state.gs_302578 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#302578:u8
        let s_10_0: bool = fn_state.gs_302578;
        // N s_10_1: branch s_10_0 b23 b11
        if s_10_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var wback:u8
        let s_11_0: bool = fn_state.wback;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#302582 <= s_12_0
        fn_state.gs_302582 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#302582:u8
        let s_13_0: bool = fn_state.gs_302582;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var add:u8
        let s_14_0: bool = fn_state.add;
        // D s_14_1: read-var index:u8
        let s_14_1: bool = fn_state.index;
        // D s_14_2: read-var m:i64
        let s_14_2: i64 = fn_state.m;
        // D s_14_3: read-var n:i64
        let s_14_3: i64 = fn_state.n;
        // D s_14_4: read-var t:i64
        let s_14_4: i64 = fn_state.t;
        // D s_14_5: read-var t2:i64
        let s_14_5: i64 = fn_state.t2;
        // D s_14_6: read-var wback:u8
        let s_14_6: bool = fn_state.wback;
        // D s_14_7: call execute_aarch32_instrs_STRD_r_Op_A_txt(s_14_0, s_14_1, s_14_2, s_14_3, s_14_4, s_14_5, s_14_6)
        let s_14_7: () = execute_aarch32_instrs_STRD_r_Op_A_txt(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
            s_14_3,
            s_14_4,
            s_14_5,
            s_14_6,
        );
        // N s_14_8: return
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
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #15s : i
        let s_16_0: i128 = 15;
        // D s_16_1: read-var n:i64
        let s_16_1: i64 = fn_state.n;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_0
        let s_16_3: bool = ((s_16_2) == (s_16_0));
        // N s_16_4: branch s_16_3 b22 b17
        if s_16_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var t:i64
        let s_17_2: i64 = fn_state.t;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#302580 <= s_17_4
        fn_state.gs_302580 = s_17_4;
        // N s_17_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#302580:u8
        let s_18_0: bool = fn_state.gs_302580;
        // N s_18_1: branch s_18_0 b21 b19
        if s_18_0 {
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
        // D s_19_5: write-var gs#302581 <= s_19_4
        fn_state.gs_302581 = s_19_4;
        // N s_19_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#302581:u8
        let s_20_0: bool = fn_state.gs_302581;
        // D s_20_1: write-var gs#302582 <= s_20_0
        fn_state.gs_302582 = s_20_0;
        // N s_20_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#302581 <= s_21_0
        fn_state.gs_302581 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#302580 <= s_22_0
        fn_state.gs_302580 = s_22_0;
        // N s_22_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#302578 <= s_24_0
        fn_state.gs_302578 = s_24_0;
        // N s_24_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var W:u8
        let s_26_0: bool = fn_state.W;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#302575 <= s_26_4
        fn_state.gs_302575 = s_26_4;
        // N s_26_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#302574 <= s_27_0
        fn_state.gs_302574 = s_27_0;
        // N s_27_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
}
