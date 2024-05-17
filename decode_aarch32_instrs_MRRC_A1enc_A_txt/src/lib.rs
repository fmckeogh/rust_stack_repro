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
use execute_aarch32_instrs_MRRC_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MRRC_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    Rt2: u8,
    Rt: u8,
    coproc: bool,
    opc1: u8,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        ga_345189: i64,
        t2: i64,
        cp: i64,
        gs_298759: bool,
        gs_298758: bool,
        cond: u8,
        Rt2: u8,
        Rt: u8,
        coproc: bool,
        opc1: u8,
        CRm: u8,
    }
    let fn_state = FunctionState {
        cond,
        Rt2,
        Rt,
        coproc,
        opc1,
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
        // D s_2_11: read-var Rt2:u8
        let s_2_11: u8 = fn_state.Rt2;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var t2 <= s_2_14
        fn_state.t2 = s_2_14;
        // C s_2_16: const #0s : i
        let s_2_16: i128 = 0;
        // D s_2_17: read-var coproc:u8
        let s_2_17: bool = fn_state.coproc;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u64
        let s_2_19: u64 = 1;
        // D s_2_20: bit-extract s_2_18 s_2_16 s_2_19
        let s_2_20: Bits = (Bits::new(
            ((s_2_18) >> (s_2_16)).value(),
            u16::try_from(s_2_19).unwrap(),
        ));
        // D s_2_21: cast reint s_2_20 -> u8
        let s_2_21: bool = ((s_2_20.value()) != 0);
        // C s_2_22: const #0s : i
        let s_2_22: i128 = 0;
        // C s_2_23: const #0u : u64
        let s_2_23: u64 = 0;
        // D s_2_24: cast zx s_2_21 -> u64
        let s_2_24: u64 = (s_2_21 as u64);
        // C s_2_25: const #1u : u64
        let s_2_25: u64 = 1;
        // D s_2_26: and s_2_24 s_2_25
        let s_2_26: u64 = ((s_2_24) & (s_2_25));
        // D s_2_27: cmp-eq s_2_26 s_2_25
        let s_2_27: bool = ((s_2_26) == (s_2_25));
        // D s_2_28: lsl s_2_24 s_2_22
        let s_2_28: u64 = s_2_24 << s_2_22;
        // D s_2_29: or s_2_23 s_2_28
        let s_2_29: u64 = ((s_2_23) | (s_2_28));
        // D s_2_30: cmpl s_2_28
        let s_2_30: u64 = !s_2_28;
        // D s_2_31: and s_2_23 s_2_30
        let s_2_31: u64 = ((s_2_23) & (s_2_30));
        // D s_2_32: select s_2_27 s_2_29 s_2_31
        let s_2_32: u64 = if s_2_27 { s_2_29 } else { s_2_31 };
        // D s_2_33: cast trunc s_2_32 -> u8
        let s_2_33: bool = ((s_2_32) != 0);
        // D s_2_34: cast zx s_2_33 -> bv
        let s_2_34: Bits = Bits::new(s_2_33 as u128, 1u16);
        // C s_2_35: const #0u : u8
        let s_2_35: bool = false;
        // C s_2_36: cast zx s_2_35 -> bv
        let s_2_36: Bits = Bits::new(s_2_35 as u128, 1u16);
        // D s_2_37: cmp-eq s_2_34 s_2_36
        let s_2_37: bool = ((s_2_34) == (s_2_36));
        // N s_2_38: branch s_2_37 b13 b3
        if s_2_37 {
            return block_13(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#345189 <= s_3_0
        fn_state.ga_345189 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#345189:i64
        let s_4_0: i64 = fn_state.ga_345189;
        // D s_4_1: write-var cp <= s_4_0
        fn_state.cp = s_4_0;
        // C s_4_2: const #15s : i
        let s_4_2: i128 = 15;
        // D s_4_3: read-var t:i64
        let s_4_3: i64 = fn_state.t;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cmp-eq s_4_4 s_4_2
        let s_4_5: bool = ((s_4_4) == (s_4_2));
        // N s_4_6: branch s_4_5 b12 b5
        if s_4_5 {
            return block_12(state, tracer, fn_state);
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
        // D s_5_1: read-var t2:i64
        let s_5_1: i64 = fn_state.t2;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#298758 <= s_5_3
        fn_state.gs_298758 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#298758:u8
        let s_6_0: bool = fn_state.gs_298758;
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
        // D s_7_0: read-var t:i64
        let s_7_0: i64 = fn_state.t;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var t2:i64
        let s_7_2: i64 = fn_state.t2;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: write-var gs#298759 <= s_7_4
        fn_state.gs_298759 = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#298759:u8
        let s_8_0: bool = fn_state.gs_298759;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
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
        // D s_9_0: read-var cp:i64
        let s_9_0: i64 = fn_state.cp;
        // D s_9_1: read-var t:i64
        let s_9_1: i64 = fn_state.t;
        // D s_9_2: read-var t2:i64
        let s_9_2: i64 = fn_state.t2;
        // D s_9_3: call execute_aarch32_instrs_MRRC_Op_A_txt(s_9_0, s_9_1, s_9_2)
        let s_9_3: () = execute_aarch32_instrs_MRRC_Op_A_txt(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
        );
        // N s_9_4: return
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
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#298759 <= s_11_0
        fn_state.gs_298759 = s_11_0;
        // N s_11_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#298758 <= s_12_0
        fn_state.gs_298758 = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #14s : i64
        let s_13_0: i64 = 14;
        // D s_13_1: write-var ga#345189 <= s_13_0
        fn_state.ga_345189 = s_13_0;
        // N s_13_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}