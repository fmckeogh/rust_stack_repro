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
use CurrentVL_read::*;
use execute_FMOPA_ZA32_PP_ZZ_16::*;
use HaveSME::*;
use common::*;
pub fn decode_FMOPA_ZA32_PP_ZZ_16<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Zm: u8,
    Pm: u8,
    Pn: u8,
    Zn: u8,
    S: bool,
    ZAda: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        a: i64,
        VL: i64,
        b: i64,
        da: i64,
        Zm: u8,
        Pm: u8,
        Pn: u8,
        Zn: u8,
        S: bool,
        ZAda: u8,
    }
    let fn_state = FunctionState {
        Zm,
        Pm,
        Pn,
        Zn,
        S,
        ZAda,
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
        // D s_1_0: read-var Pn:u8
        let s_1_0: u8 = fn_state.Pn;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 3u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var a <= s_1_3
        fn_state.a = s_1_3;
        // D s_1_5: read-var Pm:u8
        let s_1_5: u8 = fn_state.Pm;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 3u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var b <= s_1_8
        fn_state.b = s_1_8;
        // D s_1_10: read-var Zn:u8
        let s_1_10: u8 = fn_state.Zn;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var n <= s_1_13
        fn_state.n = s_1_13;
        // D s_1_15: read-var Zm:u8
        let s_1_15: u8 = fn_state.Zm;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 5u16);
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (s_1_16.value() as i128);
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: write-var m <= s_1_18
        fn_state.m = s_1_18;
        // D s_1_20: read-var ZAda:u8
        let s_1_20: u8 = fn_state.ZAda;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 2u16);
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (s_1_21.value() as i128);
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: write-var da <= s_1_23
        fn_state.da = s_1_23;
        // D s_1_25: read-var VL:i64
        let s_1_25: i64 = fn_state.VL;
        // C s_1_26: const #128s : i
        let s_1_26: i128 = 128;
        // D s_1_27: cast zx s_1_25 -> i
        let s_1_27: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_28: cmp-eq s_1_27 s_1_26
        let s_1_28: bool = ((s_1_27) == (s_1_26));
        // D s_1_29: not s_1_28
        let s_1_29: bool = !s_1_28;
        // N s_1_30: branch s_1_29 b3 b2
        if s_1_29 {
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
        // C s_2_1: const #4s : i64
        let s_2_1: i64 = 4;
        // C s_2_2: const #4s : i64
        let s_2_2: i64 = 4;
        // C s_2_3: cast zx s_2_1 -> i
        let s_2_3: i128 = (i128::try_from(s_2_1).unwrap());
        // C s_2_4: cast zx s_2_2 -> i
        let s_2_4: i128 = (i128::try_from(s_2_2).unwrap());
        // C s_2_5: mul s_2_3 s_2_4
        let s_2_5: i128 = ((s_2_3) * (s_2_4));
        // C s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // C s_2_7: const #32s : i
        let s_2_7: i128 = 32;
        // C s_2_8: cast zx s_2_6 -> i
        let s_2_8: i128 = (i128::try_from(s_2_6).unwrap());
        // C s_2_9: mul s_2_8 s_2_7
        let s_2_9: i128 = ((s_2_8) * (s_2_7));
        // C s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // C s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // C s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // D s_2_13: read-var a:i64
        let s_2_13: i64 = fn_state.a;
        // D s_2_14: read-var b:i64
        let s_2_14: i64 = fn_state.b;
        // D s_2_15: read-var da:i64
        let s_2_15: i64 = fn_state.da;
        // D s_2_16: read-var m:i64
        let s_2_16: i64 = fn_state.m;
        // D s_2_17: read-var n:i64
        let s_2_17: i64 = fn_state.n;
        // C s_2_18: const #0u : u8
        let s_2_18: bool = false;
        // D s_2_19: call execute_FMOPA_ZA32_PP_ZZ_16(s_2_0, s_2_12, s_2_13, s_2_14, s_2_15, s_2_16, s_2_17, s_2_18)
        let s_2_19: () = execute_FMOPA_ZA32_PP_ZZ_16(
            state,
            tracer,
            s_2_0,
            s_2_12,
            s_2_13,
            s_2_14,
            s_2_15,
            s_2_16,
            s_2_17,
            s_2_18,
        );
        // N s_2_20: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VL:i64
        let s_3_0: i64 = fn_state.VL;
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
        // C s_4_1: const #8s : i64
        let s_4_1: i64 = 8;
        // C s_4_2: const #8s : i64
        let s_4_2: i64 = 8;
        // C s_4_3: cast zx s_4_1 -> i
        let s_4_3: i128 = (i128::try_from(s_4_1).unwrap());
        // C s_4_4: cast zx s_4_2 -> i
        let s_4_4: i128 = (i128::try_from(s_4_2).unwrap());
        // C s_4_5: mul s_4_3 s_4_4
        let s_4_5: i128 = ((s_4_3) * (s_4_4));
        // C s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // C s_4_7: const #32s : i
        let s_4_7: i128 = 32;
        // C s_4_8: cast zx s_4_6 -> i
        let s_4_8: i128 = (i128::try_from(s_4_6).unwrap());
        // C s_4_9: mul s_4_8 s_4_7
        let s_4_9: i128 = ((s_4_8) * (s_4_7));
        // C s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // C s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // C s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // D s_4_13: read-var a:i64
        let s_4_13: i64 = fn_state.a;
        // D s_4_14: read-var b:i64
        let s_4_14: i64 = fn_state.b;
        // D s_4_15: read-var da:i64
        let s_4_15: i64 = fn_state.da;
        // D s_4_16: read-var m:i64
        let s_4_16: i64 = fn_state.m;
        // D s_4_17: read-var n:i64
        let s_4_17: i64 = fn_state.n;
        // C s_4_18: const #0u : u8
        let s_4_18: bool = false;
        // D s_4_19: call execute_FMOPA_ZA32_PP_ZZ_16(s_4_0, s_4_12, s_4_13, s_4_14, s_4_15, s_4_16, s_4_17, s_4_18)
        let s_4_19: () = execute_FMOPA_ZA32_PP_ZZ_16(
            state,
            tracer,
            s_4_0,
            s_4_12,
            s_4_13,
            s_4_14,
            s_4_15,
            s_4_16,
            s_4_17,
            s_4_18,
        );
        // N s_4_20: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
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
        // C s_6_1: const #16s : i64
        let s_6_1: i64 = 16;
        // C s_6_2: const #16s : i64
        let s_6_2: i64 = 16;
        // C s_6_3: cast zx s_6_1 -> i
        let s_6_3: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_4: cast zx s_6_2 -> i
        let s_6_4: i128 = (i128::try_from(s_6_2).unwrap());
        // C s_6_5: mul s_6_3 s_6_4
        let s_6_5: i128 = ((s_6_3) * (s_6_4));
        // C s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // C s_6_7: const #32s : i
        let s_6_7: i128 = 32;
        // C s_6_8: cast zx s_6_6 -> i
        let s_6_8: i128 = (i128::try_from(s_6_6).unwrap());
        // C s_6_9: mul s_6_8 s_6_7
        let s_6_9: i128 = ((s_6_8) * (s_6_7));
        // C s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // C s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // C s_6_12: cast reint s_6_11 -> i64
        let s_6_12: i64 = (s_6_11 as i64);
        // D s_6_13: read-var a:i64
        let s_6_13: i64 = fn_state.a;
        // D s_6_14: read-var b:i64
        let s_6_14: i64 = fn_state.b;
        // D s_6_15: read-var da:i64
        let s_6_15: i64 = fn_state.da;
        // D s_6_16: read-var m:i64
        let s_6_16: i64 = fn_state.m;
        // D s_6_17: read-var n:i64
        let s_6_17: i64 = fn_state.n;
        // C s_6_18: const #0u : u8
        let s_6_18: bool = false;
        // D s_6_19: call execute_FMOPA_ZA32_PP_ZZ_16(s_6_0, s_6_12, s_6_13, s_6_14, s_6_15, s_6_16, s_6_17, s_6_18)
        let s_6_19: () = execute_FMOPA_ZA32_PP_ZZ_16(
            state,
            tracer,
            s_6_0,
            s_6_12,
            s_6_13,
            s_6_14,
            s_6_15,
            s_6_16,
            s_6_17,
            s_6_18,
        );
        // N s_6_20: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
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
        // C s_8_1: const #32s : i64
        let s_8_1: i64 = 32;
        // C s_8_2: const #32s : i64
        let s_8_2: i64 = 32;
        // C s_8_3: cast zx s_8_1 -> i
        let s_8_3: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_4: cast zx s_8_2 -> i
        let s_8_4: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_5: mul s_8_3 s_8_4
        let s_8_5: i128 = ((s_8_3) * (s_8_4));
        // C s_8_6: cast reint s_8_5 -> i64
        let s_8_6: i64 = (s_8_5 as i64);
        // C s_8_7: const #32s : i
        let s_8_7: i128 = 32;
        // C s_8_8: cast zx s_8_6 -> i
        let s_8_8: i128 = (i128::try_from(s_8_6).unwrap());
        // C s_8_9: mul s_8_8 s_8_7
        let s_8_9: i128 = ((s_8_8) * (s_8_7));
        // C s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // C s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // C s_8_12: cast reint s_8_11 -> i64
        let s_8_12: i64 = (s_8_11 as i64);
        // D s_8_13: read-var a:i64
        let s_8_13: i64 = fn_state.a;
        // D s_8_14: read-var b:i64
        let s_8_14: i64 = fn_state.b;
        // D s_8_15: read-var da:i64
        let s_8_15: i64 = fn_state.da;
        // D s_8_16: read-var m:i64
        let s_8_16: i64 = fn_state.m;
        // D s_8_17: read-var n:i64
        let s_8_17: i64 = fn_state.n;
        // C s_8_18: const #0u : u8
        let s_8_18: bool = false;
        // D s_8_19: call execute_FMOPA_ZA32_PP_ZZ_16(s_8_0, s_8_12, s_8_13, s_8_14, s_8_15, s_8_16, s_8_17, s_8_18)
        let s_8_19: () = execute_FMOPA_ZA32_PP_ZZ_16(
            state,
            tracer,
            s_8_0,
            s_8_12,
            s_8_13,
            s_8_14,
            s_8_15,
            s_8_16,
            s_8_17,
            s_8_18,
        );
        // N s_8_20: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
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
        // C s_10_1: const #64s : i64
        let s_10_1: i64 = 64;
        // C s_10_2: const #64s : i64
        let s_10_2: i64 = 64;
        // C s_10_3: cast zx s_10_1 -> i
        let s_10_3: i128 = (i128::try_from(s_10_1).unwrap());
        // C s_10_4: cast zx s_10_2 -> i
        let s_10_4: i128 = (i128::try_from(s_10_2).unwrap());
        // C s_10_5: mul s_10_3 s_10_4
        let s_10_5: i128 = ((s_10_3) * (s_10_4));
        // C s_10_6: cast reint s_10_5 -> i64
        let s_10_6: i64 = (s_10_5 as i64);
        // C s_10_7: const #32s : i
        let s_10_7: i128 = 32;
        // C s_10_8: cast zx s_10_6 -> i
        let s_10_8: i128 = (i128::try_from(s_10_6).unwrap());
        // C s_10_9: mul s_10_8 s_10_7
        let s_10_9: i128 = ((s_10_8) * (s_10_7));
        // C s_10_10: cast reint s_10_9 -> i64
        let s_10_10: i64 = (s_10_9 as i64);
        // C s_10_11: cast zx s_10_10 -> i
        let s_10_11: i128 = (i128::try_from(s_10_10).unwrap());
        // C s_10_12: cast reint s_10_11 -> i64
        let s_10_12: i64 = (s_10_11 as i64);
        // D s_10_13: read-var a:i64
        let s_10_13: i64 = fn_state.a;
        // D s_10_14: read-var b:i64
        let s_10_14: i64 = fn_state.b;
        // D s_10_15: read-var da:i64
        let s_10_15: i64 = fn_state.da;
        // D s_10_16: read-var m:i64
        let s_10_16: i64 = fn_state.m;
        // D s_10_17: read-var n:i64
        let s_10_17: i64 = fn_state.n;
        // C s_10_18: const #0u : u8
        let s_10_18: bool = false;
        // D s_10_19: call execute_FMOPA_ZA32_PP_ZZ_16(s_10_0, s_10_12, s_10_13, s_10_14, s_10_15, s_10_16, s_10_17, s_10_18)
        let s_10_19: () = execute_FMOPA_ZA32_PP_ZZ_16(
            state,
            tracer,
            s_10_0,
            s_10_12,
            s_10_13,
            s_10_14,
            s_10_15,
            s_10_16,
            s_10_17,
            s_10_18,
        );
        // N s_10_20: return
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
