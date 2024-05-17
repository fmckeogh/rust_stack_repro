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
use execute_aarch32_instrs_LDREXD_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDREXD_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    Rn: u8,
    Rt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_297428: bool,
        t: i64,
        t2: i64,
        gs_297430: bool,
        n: i64,
        cond: u8,
        Rn: u8,
        Rt: u8,
    }
    let fn_state = FunctionState {
        cond,
        Rn,
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
        // C s_2_11: const #1s : i
        let s_2_11: i128 = 1;
        // D s_2_12: read-var t:i64
        let s_2_12: i64 = fn_state.t;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: add s_2_13 s_2_11
        let s_2_14: i128 = (s_2_13 + s_2_11);
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: write-var t2 <= s_2_15
        fn_state.t2 = s_2_15;
        // D s_2_17: read-var Rn:u8
        let s_2_17: u8 = fn_state.Rn;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 4u16);
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (s_2_18.value() as i128);
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: write-var n <= s_2_20
        fn_state.n = s_2_20;
        // C s_2_22: const #0s : i
        let s_2_22: i128 = 0;
        // D s_2_23: read-var Rt:u8
        let s_2_23: u8 = fn_state.Rt;
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 4u16);
        // C s_2_25: const #1u : u64
        let s_2_25: u64 = 1;
        // D s_2_26: bit-extract s_2_24 s_2_22 s_2_25
        let s_2_26: Bits = (Bits::new(
            ((s_2_24) >> (s_2_22)).value(),
            u16::try_from(s_2_25).unwrap(),
        ));
        // D s_2_27: cast reint s_2_26 -> u8
        let s_2_27: bool = ((s_2_26.value()) != 0);
        // C s_2_28: const #0s : i
        let s_2_28: i128 = 0;
        // C s_2_29: const #0u : u64
        let s_2_29: u64 = 0;
        // D s_2_30: cast zx s_2_27 -> u64
        let s_2_30: u64 = (s_2_27 as u64);
        // C s_2_31: const #1u : u64
        let s_2_31: u64 = 1;
        // D s_2_32: and s_2_30 s_2_31
        let s_2_32: u64 = ((s_2_30) & (s_2_31));
        // D s_2_33: cmp-eq s_2_32 s_2_31
        let s_2_33: bool = ((s_2_32) == (s_2_31));
        // D s_2_34: lsl s_2_30 s_2_28
        let s_2_34: u64 = s_2_30 << s_2_28;
        // D s_2_35: or s_2_29 s_2_34
        let s_2_35: u64 = ((s_2_29) | (s_2_34));
        // D s_2_36: cmpl s_2_34
        let s_2_36: u64 = !s_2_34;
        // D s_2_37: and s_2_29 s_2_36
        let s_2_37: u64 = ((s_2_29) & (s_2_36));
        // D s_2_38: select s_2_33 s_2_35 s_2_37
        let s_2_38: u64 = if s_2_33 { s_2_35 } else { s_2_37 };
        // D s_2_39: cast trunc s_2_38 -> u8
        let s_2_39: bool = ((s_2_38) != 0);
        // D s_2_40: cast zx s_2_39 -> bv
        let s_2_40: Bits = Bits::new(s_2_39 as u128, 1u16);
        // C s_2_41: const #1u : u8
        let s_2_41: bool = true;
        // C s_2_42: cast zx s_2_41 -> bv
        let s_2_42: Bits = Bits::new(s_2_41 as u128, 1u16);
        // D s_2_43: cmp-eq s_2_40 s_2_42
        let s_2_43: bool = ((s_2_40) == (s_2_42));
        // N s_2_44: branch s_2_43 b10 b3
        if s_2_43 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var t2:i64
        let s_3_1: i64 = fn_state.t2;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#297428 <= s_3_3
        fn_state.gs_297428 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297428:u8
        let s_4_0: bool = fn_state.gs_297428;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#297430 <= s_5_3
        fn_state.gs_297430 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#297430:u8
        let s_6_0: bool = fn_state.gs_297430;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
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
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // D s_7_1: read-var t:i64
        let s_7_1: i64 = fn_state.t;
        // D s_7_2: read-var t2:i64
        let s_7_2: i64 = fn_state.t2;
        // D s_7_3: call execute_aarch32_instrs_LDREXD_Op_A_txt(s_7_0, s_7_1, s_7_2)
        let s_7_3: () = execute_aarch32_instrs_LDREXD_Op_A_txt(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
        );
        // N s_7_4: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#297430 <= s_9_0
        fn_state.gs_297430 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#297428 <= s_10_0
        fn_state.gs_297428 = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
