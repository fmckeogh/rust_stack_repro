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
use common::*;
pub fn decode_dmb_aarch64_instrs_system_barriers_dmb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    opc: u8,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_252389: u8,
        ga_252388: u8,
        opc: u8,
        CRm: u8,
    }
    let fn_state = FunctionState {
        opc,
        CRm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // D s_0_1: read-var CRm:u8
        let s_0_1: u8 = fn_state.CRm;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 4u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: write-var ga#252388 <= s_0_8
        fn_state.ga_252388 = s_0_8;
        // D s_0_10: read-var ga#252388:u8
        let s_0_10: u8 = fn_state.ga_252388;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // D s_0_15: not s_0_14
        let s_0_15: bool = !s_0_14;
        // N s_0_16: branch s_0_15 b9 b1
        if s_0_15 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var CRm:u8
        let s_2_1: u8 = fn_state.CRm;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #1s : i
        let s_2_5: i128 = 1;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_0 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: u8 = (s_2_7.value() as u8);
        // D s_2_9: write-var ga#252389 <= s_2_8
        fn_state.ga_252389 = s_2_8;
        // D s_2_10: read-var ga#252389:u8
        let s_2_10: u8 = fn_state.ga_252389;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 2u16);
        // C s_2_12: const #0u : u8
        let s_2_12: u8 = 0;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 2u16);
        // D s_2_14: cmp-eq s_2_11 s_2_13
        let s_2_14: bool = ((s_2_11) == (s_2_13));
        // D s_2_15: not s_2_14
        let s_2_15: bool = !s_2_14;
        // N s_2_16: branch s_2_15 b4 b3
        if s_2_15 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#252389:u8
        let s_4_0: u8 = fn_state.ga_252389;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #1u : u8
        let s_4_2: u8 = 1;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#252389:u8
        let s_6_0: u8 = fn_state.ga_252389;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
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
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#252388:u8
        let s_9_0: u8 = fn_state.ga_252388;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #1u : u8
        let s_9_2: u8 = 1;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
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
        // N s_10_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#252388:u8
        let s_11_0: u8 = fn_state.ga_252388;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
