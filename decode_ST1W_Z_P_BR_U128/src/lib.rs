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
use HaveSVE2p1::*;
use CurrentVL_read::*;
use execute_ST1W_Z_P_BR_U128::*;
use common::*;
pub fn decode_ST1W_Z_P_BR_U128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rm: u8,
    Pg: u8,
    Rn: u8,
    Zt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        VL: i64,
        n: i64,
        g: i64,
        Rm: u8,
        Pg: u8,
        Rn: u8,
        Zt: u8,
    }
    let fn_state = FunctionState {
        Rm,
        Pg,
        Rn,
        Zt,
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
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSVE2p1(s_0_3)
        let s_0_4: bool = HaveSVE2p1(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b14 b1
        if s_0_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rm:u8
        let s_1_0: u8 = fn_state.Rm;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // C s_1_2: const #31u : u8
        let s_1_2: u8 = 31;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 5u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b13 b2
        if s_1_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Zt:u8
        let s_2_0: u8 = fn_state.Zt;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 5u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var t <= s_2_3
        fn_state.t = s_2_3;
        // D s_2_5: read-var Rn:u8
        let s_2_5: u8 = fn_state.Rn;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 5u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var n <= s_2_8
        fn_state.n = s_2_8;
        // D s_2_10: read-var Rm:u8
        let s_2_10: u8 = fn_state.Rm;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 5u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: write-var m <= s_2_13
        fn_state.m = s_2_13;
        // D s_2_15: read-var Pg:u8
        let s_2_15: u8 = fn_state.Pg;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 3u16);
        // D s_2_17: cast zx s_2_16 -> i
        let s_2_17: i128 = (s_2_16.value() as i128);
        // D s_2_18: cast reint s_2_17 -> i64
        let s_2_18: i64 = (s_2_17 as i64);
        // D s_2_19: write-var g <= s_2_18
        fn_state.g = s_2_18;
        // D s_2_20: read-var VL:i64
        let s_2_20: i64 = fn_state.VL;
        // C s_2_21: const #128s : i
        let s_2_21: i128 = 128;
        // D s_2_22: cast zx s_2_20 -> i
        let s_2_22: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_23: cmp-eq s_2_22 s_2_21
        let s_2_23: bool = ((s_2_22) == (s_2_21));
        // D s_2_24: not s_2_23
        let s_2_24: bool = !s_2_23;
        // N s_2_25: branch s_2_24 b4 b3
        if s_2_24 {
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
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // C s_3_1: const #128s : i64
        let s_3_1: i64 = 128;
        // D s_3_2: read-var g:i64
        let s_3_2: i64 = fn_state.g;
        // D s_3_3: read-var m:i64
        let s_3_3: i64 = fn_state.m;
        // C s_3_4: const #32s : i64
        let s_3_4: i64 = 32;
        // D s_3_5: read-var n:i64
        let s_3_5: i64 = fn_state.n;
        // D s_3_6: read-var t:i64
        let s_3_6: i64 = fn_state.t;
        // D s_3_7: call execute_ST1W_Z_P_BR_U128(s_3_0, s_3_1, s_3_2, s_3_3, s_3_4, s_3_5, s_3_6)
        let s_3_7: () = execute_ST1W_Z_P_BR_U128(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
            s_3_4,
            s_3_5,
            s_3_6,
        );
        // N s_3_8: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VL:i64
        let s_4_0: i64 = fn_state.VL;
        // C s_4_1: const #256s : i
        let s_4_1: i128 = 256;
        // D s_4_2: cast zx s_4_0 -> i
        let s_4_2: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_1
        let s_4_3: bool = ((s_4_2) == (s_4_1));
        // D s_4_4: not s_4_3
        let s_4_4: bool = !s_4_3;
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
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
        // C s_5_0: const #256s : i64
        let s_5_0: i64 = 256;
        // C s_5_1: const #128s : i64
        let s_5_1: i64 = 128;
        // D s_5_2: read-var g:i64
        let s_5_2: i64 = fn_state.g;
        // D s_5_3: read-var m:i64
        let s_5_3: i64 = fn_state.m;
        // C s_5_4: const #32s : i64
        let s_5_4: i64 = 32;
        // D s_5_5: read-var n:i64
        let s_5_5: i64 = fn_state.n;
        // D s_5_6: read-var t:i64
        let s_5_6: i64 = fn_state.t;
        // D s_5_7: call execute_ST1W_Z_P_BR_U128(s_5_0, s_5_1, s_5_2, s_5_3, s_5_4, s_5_5, s_5_6)
        let s_5_7: () = execute_ST1W_Z_P_BR_U128(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
            s_5_3,
            s_5_4,
            s_5_5,
            s_5_6,
        );
        // N s_5_8: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VL:i64
        let s_6_0: i64 = fn_state.VL;
        // C s_6_1: const #512s : i
        let s_6_1: i128 = 512;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
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
        // C s_7_0: const #512s : i64
        let s_7_0: i64 = 512;
        // C s_7_1: const #128s : i64
        let s_7_1: i64 = 128;
        // D s_7_2: read-var g:i64
        let s_7_2: i64 = fn_state.g;
        // D s_7_3: read-var m:i64
        let s_7_3: i64 = fn_state.m;
        // C s_7_4: const #32s : i64
        let s_7_4: i64 = 32;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: read-var t:i64
        let s_7_6: i64 = fn_state.t;
        // D s_7_7: call execute_ST1W_Z_P_BR_U128(s_7_0, s_7_1, s_7_2, s_7_3, s_7_4, s_7_5, s_7_6)
        let s_7_7: () = execute_ST1W_Z_P_BR_U128(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
            s_7_3,
            s_7_4,
            s_7_5,
            s_7_6,
        );
        // N s_7_8: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #1024s : i
        let s_8_1: i128 = 1024;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
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
        // C s_9_0: const #1024s : i64
        let s_9_0: i64 = 1024;
        // C s_9_1: const #128s : i64
        let s_9_1: i64 = 128;
        // D s_9_2: read-var g:i64
        let s_9_2: i64 = fn_state.g;
        // D s_9_3: read-var m:i64
        let s_9_3: i64 = fn_state.m;
        // C s_9_4: const #32s : i64
        let s_9_4: i64 = 32;
        // D s_9_5: read-var n:i64
        let s_9_5: i64 = fn_state.n;
        // D s_9_6: read-var t:i64
        let s_9_6: i64 = fn_state.t;
        // D s_9_7: call execute_ST1W_Z_P_BR_U128(s_9_0, s_9_1, s_9_2, s_9_3, s_9_4, s_9_5, s_9_6)
        let s_9_7: () = execute_ST1W_Z_P_BR_U128(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
            s_9_4,
            s_9_5,
            s_9_6,
        );
        // N s_9_8: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #2048s : i
        let s_10_1: i128 = 2048;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2048s : i64
        let s_11_0: i64 = 2048;
        // C s_11_1: const #128s : i64
        let s_11_1: i64 = 128;
        // D s_11_2: read-var g:i64
        let s_11_2: i64 = fn_state.g;
        // D s_11_3: read-var m:i64
        let s_11_3: i64 = fn_state.m;
        // C s_11_4: const #32s : i64
        let s_11_4: i64 = 32;
        // D s_11_5: read-var n:i64
        let s_11_5: i64 = fn_state.n;
        // D s_11_6: read-var t:i64
        let s_11_6: i64 = fn_state.t;
        // D s_11_7: call execute_ST1W_Z_P_BR_U128(s_11_0, s_11_1, s_11_2, s_11_3, s_11_4, s_11_5, s_11_6)
        let s_11_7: () = execute_ST1W_Z_P_BR_U128(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
            s_11_3,
            s_11_4,
            s_11_5,
            s_11_6,
        );
        // N s_11_8: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
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
}