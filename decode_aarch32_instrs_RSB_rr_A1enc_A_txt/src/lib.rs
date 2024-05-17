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
use execute_aarch32_instrs_RSB_rr_Op_A_txt::*;
use DecodeRegShift::*;
use common::*;
pub fn decode_aarch32_instrs_RSB_rr_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    S: bool,
    Rn: u8,
    Rd: u8,
    Rs: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        s: i64,
        setflags: bool,
        gs_300115: bool,
        gs_300113: bool,
        shift_t: u32,
        n: i64,
        gs_300111: bool,
        d: i64,
        cond: u8,
        S: bool,
        Rn: u8,
        Rd: u8,
        Rs: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        cond,
        S,
        Rn,
        Rd,
        Rs,
        stype,
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
        // D s_2_6: read-var Rd:u8
        let s_2_6: u8 = fn_state.Rd;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var d <= s_2_9
        fn_state.d = s_2_9;
        // D s_2_11: read-var Rn:u8
        let s_2_11: u8 = fn_state.Rn;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var n <= s_2_14
        fn_state.n = s_2_14;
        // D s_2_16: read-var Rm:u8
        let s_2_16: u8 = fn_state.Rm;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 4u16);
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (s_2_17.value() as i128);
        // D s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // D s_2_20: write-var m <= s_2_19
        fn_state.m = s_2_19;
        // D s_2_21: read-var Rs:u8
        let s_2_21: u8 = fn_state.Rs;
        // D s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 4u16);
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (s_2_22.value() as i128);
        // D s_2_24: cast reint s_2_23 -> i64
        let s_2_24: i64 = (s_2_23 as i64);
        // D s_2_25: write-var s <= s_2_24
        fn_state.s = s_2_24;
        // D s_2_26: read-var S:u8
        let s_2_26: bool = fn_state.S;
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 1u16);
        // C s_2_28: const #1u : u8
        let s_2_28: bool = true;
        // C s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 1u16);
        // D s_2_30: cmp-eq s_2_27 s_2_29
        let s_2_30: bool = ((s_2_27) == (s_2_29));
        // D s_2_31: write-var setflags <= s_2_30
        fn_state.setflags = s_2_30;
        // D s_2_32: read-var stype:u8
        let s_2_32: u8 = fn_state.stype;
        // D s_2_33: call DecodeRegShift(s_2_32)
        let s_2_33: u32 = DecodeRegShift(state, tracer, s_2_32);
        // D s_2_34: write-var shift_t <= s_2_33
        fn_state.shift_t = s_2_33;
        // C s_2_35: const #15s : i
        let s_2_35: i128 = 15;
        // D s_2_36: read-var d:i64
        let s_2_36: i64 = fn_state.d;
        // D s_2_37: cast zx s_2_36 -> i
        let s_2_37: i128 = (i128::try_from(s_2_36).unwrap());
        // D s_2_38: cmp-eq s_2_37 s_2_35
        let s_2_38: bool = ((s_2_37) == (s_2_35));
        // N s_2_39: branch s_2_38 b13 b3
        if s_2_38 {
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
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#300111 <= s_3_3
        fn_state.gs_300111 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#300111:u8
        let s_4_0: bool = fn_state.gs_300111;
        // N s_4_1: branch s_4_0 b12 b5
        if s_4_0 {
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
        // D s_5_1: read-var m:i64
        let s_5_1: i64 = fn_state.m;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#300113 <= s_5_3
        fn_state.gs_300113 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#300113:u8
        let s_6_0: bool = fn_state.gs_300113;
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
        // C s_7_0: const #15s : i
        let s_7_0: i128 = 15;
        // D s_7_1: read-var s:i64
        let s_7_1: i64 = fn_state.s;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_0
        let s_7_3: bool = ((s_7_2) == (s_7_0));
        // D s_7_4: write-var gs#300115 <= s_7_3
        fn_state.gs_300115 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#300115:u8
        let s_8_0: bool = fn_state.gs_300115;
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
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: read-var m:i64
        let s_9_1: i64 = fn_state.m;
        // D s_9_2: read-var n:i64
        let s_9_2: i64 = fn_state.n;
        // D s_9_3: read-var s:i64
        let s_9_3: i64 = fn_state.s;
        // D s_9_4: read-var setflags:u8
        let s_9_4: bool = fn_state.setflags;
        // D s_9_5: read-var shift_t:u32
        let s_9_5: u32 = fn_state.shift_t;
        // D s_9_6: call execute_aarch32_instrs_RSB_rr_Op_A_txt(s_9_0, s_9_1, s_9_2, s_9_3, s_9_4, s_9_5)
        let s_9_6: () = execute_aarch32_instrs_RSB_rr_Op_A_txt(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
            s_9_4,
            s_9_5,
        );
        // N s_9_7: return
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
        // D s_11_1: write-var gs#300115 <= s_11_0
        fn_state.gs_300115 = s_11_0;
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
        // D s_12_1: write-var gs#300113 <= s_12_0
        fn_state.gs_300113 = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#300111 <= s_13_0
        fn_state.gs_300111 = s_13_0;
        // N s_13_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
