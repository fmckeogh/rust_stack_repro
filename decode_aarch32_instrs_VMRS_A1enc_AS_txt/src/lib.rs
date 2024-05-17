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
use execute_aarch32_instrs_VMRS_Op_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMRS_A1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    reg: u8,
    Rt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323691: bool,
        t: i64,
        gs_323680: bool,
        cond: u8,
        reg: u8,
        Rt: u8,
    }
    let fn_state = FunctionState {
        cond,
        reg,
        Rt,
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
        // D s_2_6: read-var Rt:u8
        let s_2_6: u8 = fn_state.Rt;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var t <= s_2_9
        fn_state.t = s_2_9;
        // D s_2_11: read-var reg:u8
        let s_2_11: u8 = fn_state.reg;
        // C s_2_12: const #1s : i
        let s_2_12: i128 = 1;
        // D s_2_13: cast zx s_2_11 -> bv
        let s_2_13: Bits = Bits::new(s_2_11 as u128, 4u16);
        // C s_2_14: const #1s : i64
        let s_2_14: i64 = 1;
        // C s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // C s_2_16: const #2s : i
        let s_2_16: i128 = 2;
        // C s_2_17: add s_2_16 s_2_15
        let s_2_17: i128 = (s_2_16 + s_2_15);
        // D s_2_18: bit-extract s_2_13 s_2_12 s_2_17
        let s_2_18: Bits = (Bits::new(
            ((s_2_13) >> (s_2_12)).value(),
            u16::try_from(s_2_17).unwrap(),
        ));
        // D s_2_19: cast reint s_2_18 -> u8
        let s_2_19: u8 = (s_2_18.value() as u8);
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 3u16);
        // C s_2_21: const #0u : u8
        let s_2_21: u8 = 0;
        // C s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 3u16);
        // D s_2_23: cmp-eq s_2_20 s_2_22
        let s_2_23: bool = ((s_2_20) == (s_2_22));
        // D s_2_24: not s_2_23
        let s_2_24: bool = !s_2_23;
        // N s_2_25: branch s_2_24 b12 b3
        if s_2_24 {
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
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#323680 <= s_3_0
        fn_state.gs_323680 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#323680:u8
        let s_4_0: bool = fn_state.gs_323680;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b11 b5
        if s_4_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #15s : i
        let s_5_0: i128 = 15;
        // D s_5_1: read-var t:i64
        let s_5_1: i64 = fn_state.t;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b10 b6
        if s_5_3 {
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
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#323691 <= s_6_0
        fn_state.gs_323691 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#323691:u8
        let s_7_0: bool = fn_state.gs_323691;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var reg:u8
        let s_8_0: u8 = fn_state.reg;
        // D s_8_1: read-var t:i64
        let s_8_1: i64 = fn_state.t;
        // D s_8_2: call execute_aarch32_instrs_VMRS_Op_AS_txt(s_8_0, s_8_1)
        let s_8_2: () = execute_aarch32_instrs_VMRS_Op_AS_txt(
            state,
            tracer,
            s_8_0,
            s_8_1,
        );
        // N s_8_3: return
        return;
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
        // D s_10_0: read-var reg:u8
        let s_10_0: u8 = fn_state.reg;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 4u16);
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 4u16);
        // D s_10_4: cmp-ne s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) != (s_10_3));
        // D s_10_5: write-var gs#323691 <= s_10_4
        fn_state.gs_323691 = s_10_4;
        // N s_10_6: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_12_0: read-var reg:u8
        let s_12_0: u8 = fn_state.reg;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 4u16);
        // C s_12_2: const #5u : u8
        let s_12_2: u8 = 5;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 4u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
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
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#323680 <= s_13_0
        fn_state.gs_323680 = s_13_0;
        // N s_13_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var reg:u8
        let s_14_0: u8 = fn_state.reg;
        // C s_14_1: const #1s : i
        let s_14_1: i128 = 1;
        // D s_14_2: cast zx s_14_0 -> bv
        let s_14_2: Bits = Bits::new(s_14_0 as u128, 4u16);
        // C s_14_3: const #1s : i64
        let s_14_3: i64 = 1;
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #2s : i
        let s_14_5: i128 = 2;
        // C s_14_6: add s_14_5 s_14_4
        let s_14_6: i128 = (s_14_5 + s_14_4);
        // D s_14_7: bit-extract s_14_2 s_14_1 s_14_6
        let s_14_7: Bits = (Bits::new(
            ((s_14_2) >> (s_14_1)).value(),
            u16::try_from(s_14_6).unwrap(),
        ));
        // D s_14_8: cast reint s_14_7 -> u8
        let s_14_8: u8 = (s_14_7.value() as u8);
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 3u16);
        // C s_14_10: const #3u : u8
        let s_14_10: u8 = 3;
        // C s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 3u16);
        // D s_14_12: cmp-eq s_14_9 s_14_11
        let s_14_12: bool = ((s_14_9) == (s_14_11));
        // D s_14_13: not s_14_12
        let s_14_13: bool = !s_14_12;
        // N s_14_14: branch s_14_13 b16 b15
        if s_14_13 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#323680 <= s_15_0
        fn_state.gs_323680 = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var reg:u8
        let s_16_0: u8 = fn_state.reg;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 4u16);
        // C s_16_2: const #8u : u8
        let s_16_2: u8 = 8;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 4u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: not s_16_4
        let s_16_5: bool = !s_16_4;
        // N s_16_6: branch s_16_5 b18 b17
        if s_16_5 {
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
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#323680 <= s_17_0
        fn_state.gs_323680 = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#323680 <= s_18_0
        fn_state.gs_323680 = s_18_0;
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
