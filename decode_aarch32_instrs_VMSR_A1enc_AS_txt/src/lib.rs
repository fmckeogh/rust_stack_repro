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
use execute_aarch32_instrs_VMSR_Op_AS_txt::*;
use ConditionPassed::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use common::*;
pub fn decode_aarch32_instrs_VMSR_A1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    reg: u8,
    Rt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323726: bool,
        t: i64,
        gs_323730: bool,
        c: u32,
        gs_323721: bool,
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
        // N s_2_25: branch s_2_24 b21 b3
        if s_2_24 {
            return block_21(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#323721 <= s_3_0
        fn_state.gs_323721 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#323721:u8
        let s_4_0: bool = fn_state.gs_323721;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b20 b5
        if s_4_1 {
            return block_20(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#323726 <= s_5_0
        fn_state.gs_323726 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#323726:u8
        let s_6_0: bool = fn_state.gs_323726;
        // N s_6_1: branch s_6_0 b11 b7
        if s_6_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #15s : i
        let s_8_0: i128 = 15;
        // D s_8_1: read-var t:i64
        let s_8_1: i64 = fn_state.t;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
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
        // D s_9_0: read-var reg:u8
        let s_9_0: u8 = fn_state.reg;
        // D s_9_1: read-var t:i64
        let s_9_1: i64 = fn_state.t;
        // D s_9_2: call execute_aarch32_instrs_VMSR_Op_AS_txt(s_9_0, s_9_1)
        let s_9_2: () = execute_aarch32_instrs_VMSR_Op_AS_txt(
            state,
            tracer,
            s_9_0,
            s_9_1,
        );
        // N s_9_3: return
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
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // S s_11_1: call ConstrainUnpredictable(s_11_0)
        let s_11_1: u32 = ConstrainUnpredictable(state, tracer, s_11_0);
        // D s_11_2: write-var c <= s_11_1
        fn_state.c = s_11_1;
        // D s_11_3: read-var c:u32
        let s_11_3: u32 = fn_state.c;
        // C s_11_4: const #2u : u32
        let s_11_4: u32 = 2;
        // D s_11_5: cmp-eq s_11_3 s_11_4
        let s_11_5: bool = ((s_11_3) == (s_11_4));
        // N s_11_6: branch s_11_5 b19 b12
        if s_11_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var c:u32
        let s_12_0: u32 = fn_state.c;
        // C s_12_1: const #4u : u32
        let s_12_1: u32 = 4;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: write-var gs#323730 <= s_12_2
        fn_state.gs_323730 = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#323730:u8
        let s_13_0: bool = fn_state.gs_323730;
        // N s_13_1: assert s_13_0
        let s_13_1: () = assert!(s_13_0);
        // C s_13_2: const #2u : u32
        let s_13_2: u32 = 2;
        // D s_13_3: read-var c:u32
        let s_13_3: u32 = fn_state.c;
        // D s_13_4: cmp-eq s_13_2 s_13_3
        let s_13_4: bool = ((s_13_2) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
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
        // C s_15_0: const #4u : u32
        let s_15_0: u32 = 4;
        // D s_15_1: read-var c:u32
        let s_15_1: u32 = fn_state.c;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b18 b16
        if s_15_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call EndOfInstruction(s_16_0)
        let s_16_1: () = EndOfInstruction(state, tracer, s_16_0);
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#323730 <= s_19_0
        fn_state.gs_323730 = s_19_0;
        // N s_19_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var reg:u8
        let s_20_0: u8 = fn_state.reg;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 4u16);
        // C s_20_2: const #8u : u8
        let s_20_2: u8 = 8;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 4u16);
        // D s_20_4: cmp-ne s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) != (s_20_3));
        // D s_20_5: write-var gs#323726 <= s_20_4
        fn_state.gs_323726 = s_20_4;
        // N s_20_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#323721 <= s_21_0
        fn_state.gs_323721 = s_21_0;
        // N s_21_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
