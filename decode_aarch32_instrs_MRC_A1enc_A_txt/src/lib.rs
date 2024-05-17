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
use execute_aarch32_instrs_MRC_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MRC_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    opc1: u8,
    CRn: u8,
    Rt: u8,
    coproc: bool,
    opc2: u8,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_345174: i64,
        t: i64,
        cond: u8,
        opc1: u8,
        CRn: u8,
        Rt: u8,
        coproc: bool,
        opc2: u8,
        CRm: u8,
    }
    let fn_state = FunctionState {
        cond,
        opc1,
        CRn,
        Rt,
        coproc,
        opc2,
        CRm,
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
        // C s_2_11: const #0s : i
        let s_2_11: i128 = 0;
        // D s_2_12: read-var coproc:u8
        let s_2_12: bool = fn_state.coproc;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // C s_2_14: const #1u : u64
        let s_2_14: u64 = 1;
        // D s_2_15: bit-extract s_2_13 s_2_11 s_2_14
        let s_2_15: Bits = (Bits::new(
            ((s_2_13) >> (s_2_11)).value(),
            u16::try_from(s_2_14).unwrap(),
        ));
        // D s_2_16: cast reint s_2_15 -> u8
        let s_2_16: bool = ((s_2_15.value()) != 0);
        // C s_2_17: const #0s : i
        let s_2_17: i128 = 0;
        // C s_2_18: const #0u : u64
        let s_2_18: u64 = 0;
        // D s_2_19: cast zx s_2_16 -> u64
        let s_2_19: u64 = (s_2_16 as u64);
        // C s_2_20: const #1u : u64
        let s_2_20: u64 = 1;
        // D s_2_21: and s_2_19 s_2_20
        let s_2_21: u64 = ((s_2_19) & (s_2_20));
        // D s_2_22: cmp-eq s_2_21 s_2_20
        let s_2_22: bool = ((s_2_21) == (s_2_20));
        // D s_2_23: lsl s_2_19 s_2_17
        let s_2_23: u64 = s_2_19 << s_2_17;
        // D s_2_24: or s_2_18 s_2_23
        let s_2_24: u64 = ((s_2_18) | (s_2_23));
        // D s_2_25: cmpl s_2_23
        let s_2_25: u64 = !s_2_23;
        // D s_2_26: and s_2_18 s_2_25
        let s_2_26: u64 = ((s_2_18) & (s_2_25));
        // D s_2_27: select s_2_22 s_2_24 s_2_26
        let s_2_27: u64 = if s_2_22 { s_2_24 } else { s_2_26 };
        // D s_2_28: cast trunc s_2_27 -> u8
        let s_2_28: bool = ((s_2_27) != 0);
        // D s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 1u16);
        // C s_2_30: const #0u : u8
        let s_2_30: bool = false;
        // C s_2_31: cast zx s_2_30 -> bv
        let s_2_31: Bits = Bits::new(s_2_30 as u128, 1u16);
        // D s_2_32: cmp-eq s_2_29 s_2_31
        let s_2_32: bool = ((s_2_29) == (s_2_31));
        // N s_2_33: branch s_2_32 b5 b3
        if s_2_32 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #15s : i64
        let s_3_0: i64 = 15;
        // D s_3_1: write-var ga#345174 <= s_3_0
        fn_state.ga_345174 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#345174:i64
        let s_4_0: i64 = fn_state.ga_345174;
        // D s_4_1: read-var t:i64
        let s_4_1: i64 = fn_state.t;
        // D s_4_2: call execute_aarch32_instrs_MRC_Op_A_txt(s_4_0, s_4_1)
        let s_4_2: () = execute_aarch32_instrs_MRC_Op_A_txt(state, tracer, s_4_0, s_4_1);
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #14s : i64
        let s_5_0: i64 = 14;
        // D s_5_1: write-var ga#345174 <= s_5_0
        fn_state.ga_345174 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
