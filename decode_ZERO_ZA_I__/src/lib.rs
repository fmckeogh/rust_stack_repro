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
use execute_ZERO_ZA_I__::*;
use CurrentSVL_read::*;
use HaveSME::*;
use common::*;
pub fn decode_ZERO_ZA_I__<T: Tracer>(state: &mut State, tracer: &T, imm8: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        mask: u8,
        SVL: i64,
        imm8: u8,
    }
    let fn_state = FunctionState {
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
        // S s_0_1: call CurrentSVL_read(s_0_0)
        let s_0_1: i64 = CurrentSVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var SVL <= s_0_1
        fn_state.SVL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSME(s_0_3)
        let s_0_4: bool = HaveSME(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b12 b1
        if s_0_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var imm8:u8
        let s_1_0: u8 = fn_state.imm8;
        // D s_1_1: write-var mask <= s_1_0
        fn_state.mask = s_1_0;
        // D s_1_2: read-var SVL:i64
        let s_1_2: i64 = fn_state.SVL;
        // C s_1_3: const #128s : i
        let s_1_3: i128 = 128;
        // D s_1_4: cast zx s_1_2 -> i
        let s_1_4: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_5: cmp-eq s_1_4 s_1_3
        let s_1_5: bool = ((s_1_4) == (s_1_3));
        // D s_1_6: not s_1_5
        let s_1_6: bool = !s_1_5;
        // N s_1_7: branch s_1_6 b3 b2
        if s_1_6 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #128s : i64
        let s_2_0: i64 = 128;
        // C s_2_1: const #128s : i
        let s_2_1: i128 = 128;
        // C s_2_2: const #64s : i64
        let s_2_2: i64 = 64;
        // C s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // C s_2_4: div s_2_1 s_2_3
        let s_2_4: i128 = ((s_2_1) / (s_2_3));
        // C s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // C s_2_6: const #128s : i
        let s_2_6: i128 = 128;
        // C s_2_7: const #64s : i64
        let s_2_7: i64 = 64;
        // C s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // C s_2_9: div s_2_6 s_2_8
        let s_2_9: i128 = ((s_2_6) / (s_2_8));
        // C s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // C s_2_11: cast zx s_2_5 -> i
        let s_2_11: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_12: cast zx s_2_10 -> i
        let s_2_12: i128 = (i128::try_from(s_2_10).unwrap());
        // C s_2_13: mul s_2_11 s_2_12
        let s_2_13: i128 = ((s_2_11) * (s_2_12));
        // C s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // C s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // C s_2_16: const #64s : i64
        let s_2_16: i64 = 64;
        // C s_2_17: cast zx s_2_16 -> i
        let s_2_17: i128 = (i128::try_from(s_2_16).unwrap());
        // C s_2_18: mul s_2_15 s_2_17
        let s_2_18: i128 = ((s_2_15) * (s_2_17));
        // C s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // C s_2_20: cast zx s_2_19 -> i
        let s_2_20: i128 = (i128::try_from(s_2_19).unwrap());
        // C s_2_21: cast reint s_2_20 -> i64
        let s_2_21: i64 = (s_2_20 as i64);
        // C s_2_22: const #64s : i64
        let s_2_22: i64 = 64;
        // D s_2_23: read-var mask:u8
        let s_2_23: u8 = fn_state.mask;
        // D s_2_24: call execute_ZERO_ZA_I__(s_2_0, s_2_21, s_2_22, s_2_23)
        let s_2_24: () = execute_ZERO_ZA_I__(
            state,
            tracer,
            s_2_0,
            s_2_21,
            s_2_22,
            s_2_23,
        );
        // N s_2_25: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var SVL:i64
        let s_3_0: i64 = fn_state.SVL;
        // C s_3_1: const #256s : i
        let s_3_1: i128 = 256;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) == (s_3_1));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #256s : i64
        let s_4_0: i64 = 256;
        // C s_4_1: const #256s : i
        let s_4_1: i128 = 256;
        // C s_4_2: const #64s : i64
        let s_4_2: i64 = 64;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // C s_4_4: div s_4_1 s_4_3
        let s_4_4: i128 = ((s_4_1) / (s_4_3));
        // C s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // C s_4_6: const #256s : i
        let s_4_6: i128 = 256;
        // C s_4_7: const #64s : i64
        let s_4_7: i64 = 64;
        // C s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // C s_4_9: div s_4_6 s_4_8
        let s_4_9: i128 = ((s_4_6) / (s_4_8));
        // C s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // C s_4_11: cast zx s_4_5 -> i
        let s_4_11: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_12: cast zx s_4_10 -> i
        let s_4_12: i128 = (i128::try_from(s_4_10).unwrap());
        // C s_4_13: mul s_4_11 s_4_12
        let s_4_13: i128 = ((s_4_11) * (s_4_12));
        // C s_4_14: cast reint s_4_13 -> i64
        let s_4_14: i64 = (s_4_13 as i64);
        // C s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // C s_4_16: const #64s : i64
        let s_4_16: i64 = 64;
        // C s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // C s_4_18: mul s_4_15 s_4_17
        let s_4_18: i128 = ((s_4_15) * (s_4_17));
        // C s_4_19: cast reint s_4_18 -> i64
        let s_4_19: i64 = (s_4_18 as i64);
        // C s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // C s_4_21: cast reint s_4_20 -> i64
        let s_4_21: i64 = (s_4_20 as i64);
        // C s_4_22: const #64s : i64
        let s_4_22: i64 = 64;
        // D s_4_23: read-var mask:u8
        let s_4_23: u8 = fn_state.mask;
        // D s_4_24: call execute_ZERO_ZA_I__(s_4_0, s_4_21, s_4_22, s_4_23)
        let s_4_24: () = execute_ZERO_ZA_I__(
            state,
            tracer,
            s_4_0,
            s_4_21,
            s_4_22,
            s_4_23,
        );
        // N s_4_25: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var SVL:i64
        let s_5_0: i64 = fn_state.SVL;
        // C s_5_1: const #512s : i
        let s_5_1: i128 = 512;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #512s : i64
        let s_6_0: i64 = 512;
        // C s_6_1: const #512s : i
        let s_6_1: i128 = 512;
        // C s_6_2: const #64s : i64
        let s_6_2: i64 = 64;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // C s_6_4: div s_6_1 s_6_3
        let s_6_4: i128 = ((s_6_1) / (s_6_3));
        // C s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // C s_6_6: const #512s : i
        let s_6_6: i128 = 512;
        // C s_6_7: const #64s : i64
        let s_6_7: i64 = 64;
        // C s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // C s_6_9: div s_6_6 s_6_8
        let s_6_9: i128 = ((s_6_6) / (s_6_8));
        // C s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // C s_6_11: cast zx s_6_5 -> i
        let s_6_11: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_12: cast zx s_6_10 -> i
        let s_6_12: i128 = (i128::try_from(s_6_10).unwrap());
        // C s_6_13: mul s_6_11 s_6_12
        let s_6_13: i128 = ((s_6_11) * (s_6_12));
        // C s_6_14: cast reint s_6_13 -> i64
        let s_6_14: i64 = (s_6_13 as i64);
        // C s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // C s_6_16: const #64s : i64
        let s_6_16: i64 = 64;
        // C s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // C s_6_18: mul s_6_15 s_6_17
        let s_6_18: i128 = ((s_6_15) * (s_6_17));
        // C s_6_19: cast reint s_6_18 -> i64
        let s_6_19: i64 = (s_6_18 as i64);
        // C s_6_20: cast zx s_6_19 -> i
        let s_6_20: i128 = (i128::try_from(s_6_19).unwrap());
        // C s_6_21: cast reint s_6_20 -> i64
        let s_6_21: i64 = (s_6_20 as i64);
        // C s_6_22: const #64s : i64
        let s_6_22: i64 = 64;
        // D s_6_23: read-var mask:u8
        let s_6_23: u8 = fn_state.mask;
        // D s_6_24: call execute_ZERO_ZA_I__(s_6_0, s_6_21, s_6_22, s_6_23)
        let s_6_24: () = execute_ZERO_ZA_I__(
            state,
            tracer,
            s_6_0,
            s_6_21,
            s_6_22,
            s_6_23,
        );
        // N s_6_25: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var SVL:i64
        let s_7_0: i64 = fn_state.SVL;
        // C s_7_1: const #1024s : i
        let s_7_1: i128 = 1024;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
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
        // C s_8_0: const #1024s : i64
        let s_8_0: i64 = 1024;
        // C s_8_1: const #1024s : i
        let s_8_1: i128 = 1024;
        // C s_8_2: const #64s : i64
        let s_8_2: i64 = 64;
        // C s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_4: div s_8_1 s_8_3
        let s_8_4: i128 = ((s_8_1) / (s_8_3));
        // C s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // C s_8_6: const #1024s : i
        let s_8_6: i128 = 1024;
        // C s_8_7: const #64s : i64
        let s_8_7: i64 = 64;
        // C s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // C s_8_9: div s_8_6 s_8_8
        let s_8_9: i128 = ((s_8_6) / (s_8_8));
        // C s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // C s_8_11: cast zx s_8_5 -> i
        let s_8_11: i128 = (i128::try_from(s_8_5).unwrap());
        // C s_8_12: cast zx s_8_10 -> i
        let s_8_12: i128 = (i128::try_from(s_8_10).unwrap());
        // C s_8_13: mul s_8_11 s_8_12
        let s_8_13: i128 = ((s_8_11) * (s_8_12));
        // C s_8_14: cast reint s_8_13 -> i64
        let s_8_14: i64 = (s_8_13 as i64);
        // C s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // C s_8_16: const #64s : i64
        let s_8_16: i64 = 64;
        // C s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // C s_8_18: mul s_8_15 s_8_17
        let s_8_18: i128 = ((s_8_15) * (s_8_17));
        // C s_8_19: cast reint s_8_18 -> i64
        let s_8_19: i64 = (s_8_18 as i64);
        // C s_8_20: cast zx s_8_19 -> i
        let s_8_20: i128 = (i128::try_from(s_8_19).unwrap());
        // C s_8_21: cast reint s_8_20 -> i64
        let s_8_21: i64 = (s_8_20 as i64);
        // C s_8_22: const #64s : i64
        let s_8_22: i64 = 64;
        // D s_8_23: read-var mask:u8
        let s_8_23: u8 = fn_state.mask;
        // D s_8_24: call execute_ZERO_ZA_I__(s_8_0, s_8_21, s_8_22, s_8_23)
        let s_8_24: () = execute_ZERO_ZA_I__(
            state,
            tracer,
            s_8_0,
            s_8_21,
            s_8_22,
            s_8_23,
        );
        // N s_8_25: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var SVL:i64
        let s_9_0: i64 = fn_state.SVL;
        // C s_9_1: const #2048s : i
        let s_9_1: i128 = 2048;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
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
        // C s_10_0: const #2048s : i64
        let s_10_0: i64 = 2048;
        // C s_10_1: const #2048s : i
        let s_10_1: i128 = 2048;
        // C s_10_2: const #64s : i64
        let s_10_2: i64 = 64;
        // C s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // C s_10_4: div s_10_1 s_10_3
        let s_10_4: i128 = ((s_10_1) / (s_10_3));
        // C s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // C s_10_6: const #2048s : i
        let s_10_6: i128 = 2048;
        // C s_10_7: const #64s : i64
        let s_10_7: i64 = 64;
        // C s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (i128::try_from(s_10_7).unwrap());
        // C s_10_9: div s_10_6 s_10_8
        let s_10_9: i128 = ((s_10_6) / (s_10_8));
        // C s_10_10: cast reint s_10_9 -> i64
        let s_10_10: i64 = (s_10_9 as i64);
        // C s_10_11: cast zx s_10_5 -> i
        let s_10_11: i128 = (i128::try_from(s_10_5).unwrap());
        // C s_10_12: cast zx s_10_10 -> i
        let s_10_12: i128 = (i128::try_from(s_10_10).unwrap());
        // C s_10_13: mul s_10_11 s_10_12
        let s_10_13: i128 = ((s_10_11) * (s_10_12));
        // C s_10_14: cast reint s_10_13 -> i64
        let s_10_14: i64 = (s_10_13 as i64);
        // C s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // C s_10_16: const #64s : i64
        let s_10_16: i64 = 64;
        // C s_10_17: cast zx s_10_16 -> i
        let s_10_17: i128 = (i128::try_from(s_10_16).unwrap());
        // C s_10_18: mul s_10_15 s_10_17
        let s_10_18: i128 = ((s_10_15) * (s_10_17));
        // C s_10_19: cast reint s_10_18 -> i64
        let s_10_19: i64 = (s_10_18 as i64);
        // C s_10_20: cast zx s_10_19 -> i
        let s_10_20: i128 = (i128::try_from(s_10_19).unwrap());
        // C s_10_21: cast reint s_10_20 -> i64
        let s_10_21: i64 = (s_10_20 as i64);
        // C s_10_22: const #64s : i64
        let s_10_22: i64 = 64;
        // D s_10_23: read-var mask:u8
        let s_10_23: u8 = fn_state.mask;
        // D s_10_24: call execute_ZERO_ZA_I__(s_10_0, s_10_21, s_10_22, s_10_23)
        let s_10_24: () = execute_ZERO_ZA_I__(
            state,
            tracer,
            s_10_0,
            s_10_21,
            s_10_22,
            s_10_23,
        );
        // N s_10_25: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
