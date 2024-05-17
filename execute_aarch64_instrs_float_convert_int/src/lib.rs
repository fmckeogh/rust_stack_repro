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
use Vpart_read::*;
use V_read::*;
use FPCR_read::*;
use Vpart_set::*;
use FPToFixed::*;
use X_read::*;
use Zeros::*;
use FixedToFP::*;
use X_set::*;
use CheckFPEnabled64::*;
use Elem_set::*;
use u__id::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use IsMerging::*;
use FPToFixedJS::*;
use common::*;
pub fn execute_aarch64_instrs_float_convert_int<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    fltsize: i64,
    intsize: i64,
    n: i64,
    op: u32,
    part: i64,
    rounding: u32,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_150092: bool,
        gs_150064: bool,
        intvalshadow_1300: Bits,
        fltval: Bits,
        gs_150084: bool,
        ga_253748: i64,
        intvalshadow_1299: Bits,
        intsizeshadow_1296: i64,
        gs_150069: bool,
        gs_691236: ProductTypef506aa96a892fe52,
        fltvalshadow_1301: Bits,
        gs_150062: bool,
        gs_150056: bool,
        gs_150067: bool,
        gs_150098: bool,
        gs_150076: bool,
        gs_150081: bool,
        fpcr: ProductType5c790c8ef59cc8b2,
        gs_150070: bool,
        fltvalshadow_1298: Bits,
        intvalshadow_1303: Bits,
        fsize: i64,
        ga_253749: Bits,
        gs_150083: bool,
        merge: bool,
        gs_150078: bool,
        fltsizeshadow_1297: i64,
        ga_253718: i64,
        d: i64,
        fltsize: i64,
        intsize: i64,
        n: i64,
        op: u32,
        part: i64,
        rounding: u32,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        fltsize,
        intsize,
        n,
        op,
        part,
        rounding,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var intsize:i64
        let s_0_0: i64 = fn_state.intsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var intsizeshadow#1296 <= s_0_2
        fn_state.intsizeshadow_1296 = s_0_2;
        // D s_0_4: read-var fltsize:i64
        let s_0_4: i64 = fn_state.fltsize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var fltsizeshadow#1297 <= s_0_6
        fn_state.fltsizeshadow_1297 = s_0_6;
        // D s_0_8: read-var op:u32
        let s_0_8: u32 = fn_state.op;
        // C s_0_9: const #4u : u32
        let s_0_9: u32 = 4;
        // D s_0_10: cmp-eq s_0_8 s_0_9
        let s_0_10: bool = ((s_0_8) == (s_0_9));
        // N s_0_11: branch s_0_10 b61 b1
        if s_0_10 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckFPEnabled64(s_1_0)
        let s_1_1: () = CheckFPEnabled64(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call FPCR_read(s_2_0)
        let s_2_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_2_0);
        // D s_2_2: write-var fpcr <= s_2_1
        fn_state.fpcr = s_2_1;
        // D s_2_3: read-var fpcr:struct
        let s_2_3: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_2_4: call IsMerging(s_2_3)
        let s_2_4: bool = IsMerging(state, tracer, s_2_3);
        // D s_2_5: write-var merge <= s_2_4
        fn_state.merge = s_2_4;
        // D s_2_6: read-var op:u32
        let s_2_6: u32 = fn_state.op;
        // C s_2_7: const #1u : u32
        let s_2_7: u32 = 1;
        // D s_2_8: cmp-eq s_2_6 s_2_7
        let s_2_8: bool = ((s_2_6) == (s_2_7));
        // N s_2_9: branch s_2_8 b60 b3
        if s_2_8 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#150056 <= s_3_0
        fn_state.gs_150056 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#150056:u8
        let s_4_0: bool = fn_state.gs_150056;
        // N s_4_1: branch s_4_0 b59 b5
        if s_4_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var fltsizeshadow#1297:i64
        let s_5_0: i64 = fn_state.fltsizeshadow_1297;
        // D s_5_1: write-var ga#253718 <= s_5_0
        fn_state.ga_253718 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#253718:i64
        let s_6_0: i64 = fn_state.ga_253718;
        // D s_6_1: write-var fsize <= s_6_0
        fn_state.fsize = s_6_0;
        // C s_6_2: const #0u : u32
        let s_6_2: u32 = 0;
        // D s_6_3: read-var op:u32
        let s_6_3: u32 = fn_state.op;
        // D s_6_4: cmp-eq s_6_2 s_6_3
        let s_6_4: bool = ((s_6_2) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b24 b7
        if s_6_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var fsize:i64
        let s_7_0: i64 = fn_state.fsize;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var n:i64
        let s_7_3: i64 = fn_state.n;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: call V_read(s_7_4, s_7_2)
        let s_7_5: Bits = V_read(state, tracer, s_7_4, s_7_2);
        // D s_7_6: write-var fltvalshadow#1298 <= s_7_5
        fn_state.fltvalshadow_1298 = s_7_5;
        // D s_7_7: read-var fsize:i64
        let s_7_7: i64 = fn_state.fsize;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: call __id(s_7_8)
        let s_7_9: i128 = u__id(state, tracer, s_7_8);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // C s_7_11: const #16s : i
        let s_7_11: i128 = 16;
        // D s_7_12: cast zx s_7_10 -> i
        let s_7_12: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_13: cmp-eq s_7_12 s_7_11
        let s_7_13: bool = ((s_7_12) == (s_7_11));
        // N s_7_14: branch s_7_13 b23 b8
        if s_7_13 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var fsize:i64
        let s_8_0: i64 = fn_state.fsize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #32s : i
        let s_8_4: i128 = 32;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: cmp-eq s_8_5 s_8_4
        let s_8_6: bool = ((s_8_5) == (s_8_4));
        // D s_8_7: write-var gs#150062 <= s_8_6
        fn_state.gs_150062 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#150062:u8
        let s_9_0: bool = fn_state.gs_150062;
        // N s_9_1: branch s_9_0 b22 b10
        if s_9_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var fsize:i64
        let s_10_0: i64 = fn_state.fsize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #64s : i
        let s_10_4: i128 = 64;
        // D s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // D s_10_7: write-var gs#150064 <= s_10_6
        fn_state.gs_150064 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#150064:u8
        let s_11_0: bool = fn_state.gs_150064;
        // N s_11_1: branch s_11_0 b15 b12
        if s_11_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#150070 <= s_12_0
        fn_state.gs_150070 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#150070:u8
        let s_13_0: bool = fn_state.gs_150070;
        // N s_13_1: assert s_13_0
        let s_13_1: () = assert!(s_13_0);
        // D s_13_2: read-var intsizeshadow#1296:i64
        let s_13_2: i64 = fn_state.intsizeshadow_1296;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: cast reint s_13_3 -> i64
        let s_13_4: i64 = (s_13_3 as i64);
        // C s_13_5: const #0s : i
        let s_13_5: i128 = 0;
        // D s_13_6: read-var fltvalshadow#1298:bv
        let s_13_6: Bits = fn_state.fltvalshadow_1298;
        // D s_13_7: read-var is_unsigned:u8
        let s_13_7: bool = fn_state.is_unsigned;
        // D s_13_8: read-var fpcr:struct
        let s_13_8: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_13_9: read-var rounding:u32
        let s_13_9: u32 = fn_state.rounding;
        // D s_13_10: call FPToFixed(s_13_6, s_13_5, s_13_7, s_13_8, s_13_9, s_13_4)
        let s_13_10: Bits = FPToFixed(
            state,
            tracer,
            s_13_6,
            s_13_5,
            s_13_7,
            s_13_8,
            s_13_9,
            s_13_4,
        );
        // D s_13_11: write-var intvalshadow#1299 <= s_13_10
        fn_state.intvalshadow_1299 = s_13_10;
        // N s_13_12: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var intsizeshadow#1296:i64
        let s_14_0: i64 = fn_state.intsizeshadow_1296;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: read-var d:i64
        let s_14_3: i64 = fn_state.d;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: read-var intvalshadow#1299:bv
        let s_14_5: Bits = fn_state.intvalshadow_1299;
        // D s_14_6: call X_set(s_14_4, s_14_2, s_14_5)
        let s_14_6: () = X_set(state, tracer, s_14_4, s_14_2, s_14_5);
        // N s_14_7: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var intsizeshadow#1296:i64
        let s_15_0: i64 = fn_state.intsizeshadow_1296;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #16s : i
        let s_15_4: i128 = 16;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // N s_15_7: branch s_15_6 b21 b16
        if s_15_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var intsizeshadow#1296:i64
        let s_16_0: i64 = fn_state.intsizeshadow_1296;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #32s : i
        let s_16_4: i128 = 32;
        // D s_16_5: cast zx s_16_3 -> i
        let s_16_5: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_6: cmp-eq s_16_5 s_16_4
        let s_16_6: bool = ((s_16_5) == (s_16_4));
        // D s_16_7: write-var gs#150067 <= s_16_6
        fn_state.gs_150067 = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#150067:u8
        let s_17_0: bool = fn_state.gs_150067;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var intsizeshadow#1296:i64
        let s_18_0: i64 = fn_state.intsizeshadow_1296;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #64s : i
        let s_18_4: i128 = 64;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: cmp-eq s_18_5 s_18_4
        let s_18_6: bool = ((s_18_5) == (s_18_4));
        // D s_18_7: write-var gs#150069 <= s_18_6
        fn_state.gs_150069 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#150069:u8
        let s_19_0: bool = fn_state.gs_150069;
        // D s_19_1: write-var gs#150070 <= s_19_0
        fn_state.gs_150070 = s_19_0;
        // N s_19_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#150069 <= s_20_0
        fn_state.gs_150069 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#150067 <= s_21_0
        fn_state.gs_150067 = s_21_0;
        // N s_21_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#150064 <= s_22_0
        fn_state.gs_150064 = s_22_0;
        // N s_22_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#150062 <= s_23_0
        fn_state.gs_150062 = s_23_0;
        // N s_23_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u32
        let s_24_0: u32 = 1;
        // D s_24_1: read-var op:u32
        let s_24_1: u32 = fn_state.op;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // N s_24_4: branch s_24_3 b45 b25
        if s_24_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var intsizeshadow#1296:i64
        let s_25_0: i64 = fn_state.intsizeshadow_1296;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: cast reint s_25_1 -> i64
        let s_25_2: i64 = (s_25_1 as i64);
        // D s_25_3: read-var n:i64
        let s_25_3: i64 = fn_state.n;
        // D s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_5: call X_read(s_25_4, s_25_2)
        let s_25_5: Bits = X_read(state, tracer, s_25_4, s_25_2);
        // D s_25_6: write-var intvalshadow#1300 <= s_25_5
        fn_state.intvalshadow_1300 = s_25_5;
        // D s_25_7: read-var merge:u8
        let s_25_7: bool = fn_state.merge;
        // N s_25_8: branch s_25_7 b44 b26
        if s_25_7 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var fsize:i64
        let s_26_0: i64 = fn_state.fsize;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: call Zeros(s_26_1)
        let s_26_2: Bits = Zeros(state, tracer, s_26_1);
        // D s_26_3: write-var fltval <= s_26_2
        fn_state.fltval = s_26_2;
        // N s_26_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var fltsizeshadow#1297:i64
        let s_27_0: i64 = fn_state.fltsizeshadow_1297;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #16s : i
        let s_27_4: i128 = 16;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // N s_27_7: branch s_27_6 b43 b28
        if s_27_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fltsizeshadow#1297:i64
        let s_28_0: i64 = fn_state.fltsizeshadow_1297;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: call __id(s_28_1)
        let s_28_2: i128 = u__id(state, tracer, s_28_1);
        // D s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: const #32s : i
        let s_28_4: i128 = 32;
        // D s_28_5: cast zx s_28_3 -> i
        let s_28_5: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_6: cmp-eq s_28_5 s_28_4
        let s_28_6: bool = ((s_28_5) == (s_28_4));
        // D s_28_7: write-var gs#150076 <= s_28_6
        fn_state.gs_150076 = s_28_6;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#150076:u8
        let s_29_0: bool = fn_state.gs_150076;
        // N s_29_1: branch s_29_0 b42 b30
        if s_29_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var fltsizeshadow#1297:i64
        let s_30_0: i64 = fn_state.fltsizeshadow_1297;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #64s : i
        let s_30_4: i128 = 64;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // D s_30_7: write-var gs#150078 <= s_30_6
        fn_state.gs_150078 = s_30_6;
        // N s_30_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#150078:u8
        let s_31_0: bool = fn_state.gs_150078;
        // N s_31_1: branch s_31_0 b35 b32
        if s_31_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#150084 <= s_32_0
        fn_state.gs_150084 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#150084:u8
        let s_33_0: bool = fn_state.gs_150084;
        // N s_33_1: assert s_33_0
        let s_33_1: () = assert!(s_33_0);
        // D s_33_2: read-var fltsizeshadow#1297:i64
        let s_33_2: i64 = fn_state.fltsizeshadow_1297;
        // D s_33_3: cast zx s_33_2 -> i
        let s_33_3: i128 = (i128::try_from(s_33_2).unwrap());
        // D s_33_4: cast reint s_33_3 -> i64
        let s_33_4: i64 = (s_33_3 as i64);
        // D s_33_5: write-var ga#253748 <= s_33_4
        fn_state.ga_253748 = s_33_4;
        // D s_33_6: read-var fltsizeshadow#1297:i64
        let s_33_6: i64 = fn_state.fltsizeshadow_1297;
        // D s_33_7: cast zx s_33_6 -> i
        let s_33_7: i128 = (i128::try_from(s_33_6).unwrap());
        // D s_33_8: cast reint s_33_7 -> i64
        let s_33_8: i64 = (s_33_7 as i64);
        // C s_33_9: const #0s : i
        let s_33_9: i128 = 0;
        // D s_33_10: read-var intvalshadow#1300:bv
        let s_33_10: Bits = fn_state.intvalshadow_1300;
        // D s_33_11: read-var is_unsigned:u8
        let s_33_11: bool = fn_state.is_unsigned;
        // D s_33_12: read-var fpcr:struct
        let s_33_12: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_33_13: read-var rounding:u32
        let s_33_13: u32 = fn_state.rounding;
        // D s_33_14: call FixedToFP(s_33_10, s_33_9, s_33_11, s_33_12, s_33_13, s_33_8)
        let s_33_14: Bits = FixedToFP(
            state,
            tracer,
            s_33_10,
            s_33_9,
            s_33_11,
            s_33_12,
            s_33_13,
            s_33_8,
        );
        // D s_33_15: write-var ga#253749 <= s_33_14
        fn_state.ga_253749 = s_33_14;
        // N s_33_16: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0s : i
        let s_34_0: i128 = 0;
        // D s_34_1: read-var ga#253748:i64
        let s_34_1: i64 = fn_state.ga_253748;
        // D s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (i128::try_from(s_34_1).unwrap());
        // D s_34_3: read-var fltval:bv
        let s_34_3: Bits = fn_state.fltval;
        // D s_34_4: read-var ga#253749:bv
        let s_34_4: Bits = fn_state.ga_253749;
        // D s_34_5: call Elem_set(s_34_3, s_34_0, s_34_2, s_34_4)
        let s_34_5: Bits = Elem_set(state, tracer, s_34_3, s_34_0, s_34_2, s_34_4);
        // D s_34_6: write-var fltval <= s_34_5
        fn_state.fltval = s_34_5;
        // D s_34_7: read-var fsize:i64
        let s_34_7: i64 = fn_state.fsize;
        // D s_34_8: cast zx s_34_7 -> i
        let s_34_8: i128 = (i128::try_from(s_34_7).unwrap());
        // D s_34_9: cast reint s_34_8 -> i64
        let s_34_9: i64 = (s_34_8 as i64);
        // D s_34_10: read-var d:i64
        let s_34_10: i64 = fn_state.d;
        // D s_34_11: cast zx s_34_10 -> i
        let s_34_11: i128 = (i128::try_from(s_34_10).unwrap());
        // D s_34_12: read-var fltval:bv
        let s_34_12: Bits = fn_state.fltval;
        // D s_34_13: call V_set(s_34_11, s_34_9, s_34_12)
        let s_34_13: () = V_set(state, tracer, s_34_11, s_34_9, s_34_12);
        // N s_34_14: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var intsizeshadow#1296:i64
        let s_35_0: i64 = fn_state.intsizeshadow_1296;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call __id(s_35_1)
        let s_35_2: i128 = u__id(state, tracer, s_35_1);
        // D s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: const #16s : i
        let s_35_4: i128 = 16;
        // D s_35_5: cast zx s_35_3 -> i
        let s_35_5: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_6: cmp-eq s_35_5 s_35_4
        let s_35_6: bool = ((s_35_5) == (s_35_4));
        // N s_35_7: branch s_35_6 b41 b36
        if s_35_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var intsizeshadow#1296:i64
        let s_36_0: i64 = fn_state.intsizeshadow_1296;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #32s : i
        let s_36_4: i128 = 32;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-eq s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) == (s_36_4));
        // D s_36_7: write-var gs#150081 <= s_36_6
        fn_state.gs_150081 = s_36_6;
        // N s_36_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#150081:u8
        let s_37_0: bool = fn_state.gs_150081;
        // N s_37_1: branch s_37_0 b40 b38
        if s_37_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var intsizeshadow#1296:i64
        let s_38_0: i64 = fn_state.intsizeshadow_1296;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: call __id(s_38_1)
        let s_38_2: i128 = u__id(state, tracer, s_38_1);
        // D s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: const #64s : i
        let s_38_4: i128 = 64;
        // D s_38_5: cast zx s_38_3 -> i
        let s_38_5: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_6: cmp-eq s_38_5 s_38_4
        let s_38_6: bool = ((s_38_5) == (s_38_4));
        // D s_38_7: write-var gs#150083 <= s_38_6
        fn_state.gs_150083 = s_38_6;
        // N s_38_8: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#150083:u8
        let s_39_0: bool = fn_state.gs_150083;
        // D s_39_1: write-var gs#150084 <= s_39_0
        fn_state.gs_150084 = s_39_0;
        // N s_39_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#150083 <= s_40_0
        fn_state.gs_150083 = s_40_0;
        // N s_40_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#150081 <= s_41_0
        fn_state.gs_150081 = s_41_0;
        // N s_41_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#150078 <= s_42_0
        fn_state.gs_150078 = s_42_0;
        // N s_42_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#150076 <= s_43_0
        fn_state.gs_150076 = s_43_0;
        // N s_43_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var fsize:i64
        let s_44_0: i64 = fn_state.fsize;
        // D s_44_1: cast zx s_44_0 -> i
        let s_44_1: i128 = (i128::try_from(s_44_0).unwrap());
        // D s_44_2: cast reint s_44_1 -> i64
        let s_44_2: i64 = (s_44_1 as i64);
        // D s_44_3: read-var d:i64
        let s_44_3: i64 = fn_state.d;
        // D s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // D s_44_5: call V_read(s_44_4, s_44_2)
        let s_44_5: Bits = V_read(state, tracer, s_44_4, s_44_2);
        // D s_44_6: write-var fltval <= s_44_5
        fn_state.fltval = s_44_5;
        // N s_44_7: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #2u : u32
        let s_45_0: u32 = 2;
        // D s_45_1: read-var op:u32
        let s_45_1: u32 = fn_state.op;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // D s_45_3: not s_45_2
        let s_45_3: bool = !s_45_2;
        // N s_45_4: branch s_45_3 b50 b46
        if s_45_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var fsize:i64
        let s_46_0: i64 = fn_state.fsize;
        // D s_46_1: cast zx s_46_0 -> i
        let s_46_1: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_2: cast reint s_46_1 -> i64
        let s_46_2: i64 = (s_46_1 as i64);
        // D s_46_3: read-var n:i64
        let s_46_3: i64 = fn_state.n;
        // D s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // D s_46_5: read-var part:i64
        let s_46_5: i64 = fn_state.part;
        // D s_46_6: cast zx s_46_5 -> i
        let s_46_6: i128 = (i128::try_from(s_46_5).unwrap());
        // D s_46_7: cast zx s_46_2 -> i
        let s_46_7: i128 = (i128::try_from(s_46_2).unwrap());
        // D s_46_8: call Vpart_read(s_46_4, s_46_6, s_46_7)
        let s_46_8: Bits = Vpart_read(state, tracer, s_46_4, s_46_6, s_46_7);
        // D s_46_9: write-var fltvalshadow#1301 <= s_46_8
        fn_state.fltvalshadow_1301 = s_46_8;
        // D s_46_10: read-var fsize:i64
        let s_46_10: i64 = fn_state.fsize;
        // D s_46_11: cast zx s_46_10 -> i
        let s_46_11: i128 = (i128::try_from(s_46_10).unwrap());
        // D s_46_12: call __id(s_46_11)
        let s_46_12: i128 = u__id(state, tracer, s_46_11);
        // D s_46_13: cast reint s_46_12 -> i64
        let s_46_13: i64 = (s_46_12 as i64);
        // C s_46_14: const #0s : i
        let s_46_14: i128 = 0;
        // D s_46_15: cast zx s_46_13 -> i
        let s_46_15: i128 = (i128::try_from(s_46_13).unwrap());
        // D s_46_16: cmp-ge s_46_15 s_46_14
        let s_46_16: bool = ((s_46_15) >= (s_46_14));
        // N s_46_17: branch s_46_16 b49 b47
        if s_46_16 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#150092 <= s_47_0
        fn_state.gs_150092 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#150092:u8
        let s_48_0: bool = fn_state.gs_150092;
        // N s_48_1: assert s_48_0
        let s_48_1: () = assert!(s_48_0);
        // D s_48_2: read-var intsizeshadow#1296:i64
        let s_48_2: i64 = fn_state.intsizeshadow_1296;
        // D s_48_3: cast zx s_48_2 -> i
        let s_48_3: i128 = (i128::try_from(s_48_2).unwrap());
        // D s_48_4: read-var fltvalshadow#1301:bv
        let s_48_4: Bits = fn_state.fltvalshadow_1301;
        // D s_48_5: bits-cast zx s_48_4 -> bv length s_48_3
        let s_48_5: Bits = s_48_4.zero_extend(s_48_3);
        // D s_48_6: read-var intsizeshadow#1296:i64
        let s_48_6: i64 = fn_state.intsizeshadow_1296;
        // D s_48_7: cast zx s_48_6 -> i
        let s_48_7: i128 = (i128::try_from(s_48_6).unwrap());
        // D s_48_8: cast reint s_48_7 -> i64
        let s_48_8: i64 = (s_48_7 as i64);
        // D s_48_9: read-var d:i64
        let s_48_9: i64 = fn_state.d;
        // D s_48_10: cast zx s_48_9 -> i
        let s_48_10: i128 = (i128::try_from(s_48_9).unwrap());
        // D s_48_11: call X_set(s_48_10, s_48_8, s_48_5)
        let s_48_11: () = X_set(state, tracer, s_48_10, s_48_8, s_48_5);
        // N s_48_12: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var intsizeshadow#1296:i64
        let s_49_0: i64 = fn_state.intsizeshadow_1296;
        // D s_49_1: cast zx s_49_0 -> i
        let s_49_1: i128 = (i128::try_from(s_49_0).unwrap());
        // D s_49_2: call __id(s_49_1)
        let s_49_2: i128 = u__id(state, tracer, s_49_1);
        // D s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // D s_49_4: read-var fsize:i64
        let s_49_4: i64 = fn_state.fsize;
        // D s_49_5: cast zx s_49_4 -> i
        let s_49_5: i128 = (i128::try_from(s_49_4).unwrap());
        // D s_49_6: call __id(s_49_5)
        let s_49_6: i128 = u__id(state, tracer, s_49_5);
        // D s_49_7: cast reint s_49_6 -> i64
        let s_49_7: i64 = (s_49_6 as i64);
        // D s_49_8: cast zx s_49_3 -> i
        let s_49_8: i128 = (i128::try_from(s_49_3).unwrap());
        // D s_49_9: cast zx s_49_7 -> i
        let s_49_9: i128 = (i128::try_from(s_49_7).unwrap());
        // D s_49_10: cmp-ge s_49_8 s_49_9
        let s_49_10: bool = ((s_49_8) >= (s_49_9));
        // D s_49_11: write-var gs#150092 <= s_49_10
        fn_state.gs_150092 = s_49_10;
        // N s_49_12: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #3u : u32
        let s_50_0: u32 = 3;
        // D s_50_1: read-var op:u32
        let s_50_1: u32 = fn_state.op;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: branch s_50_3 b55 b51
        if s_50_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var intsizeshadow#1296:i64
        let s_51_0: i64 = fn_state.intsizeshadow_1296;
        // D s_51_1: cast zx s_51_0 -> i
        let s_51_1: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_2: cast reint s_51_1 -> i64
        let s_51_2: i64 = (s_51_1 as i64);
        // D s_51_3: read-var n:i64
        let s_51_3: i64 = fn_state.n;
        // D s_51_4: cast zx s_51_3 -> i
        let s_51_4: i128 = (i128::try_from(s_51_3).unwrap());
        // D s_51_5: call X_read(s_51_4, s_51_2)
        let s_51_5: Bits = X_read(state, tracer, s_51_4, s_51_2);
        // D s_51_6: write-var intvalshadow#1303 <= s_51_5
        fn_state.intvalshadow_1303 = s_51_5;
        // D s_51_7: read-var fsize:i64
        let s_51_7: i64 = fn_state.fsize;
        // D s_51_8: cast zx s_51_7 -> i
        let s_51_8: i128 = (i128::try_from(s_51_7).unwrap());
        // D s_51_9: call __id(s_51_8)
        let s_51_9: i128 = u__id(state, tracer, s_51_8);
        // D s_51_10: cast reint s_51_9 -> i64
        let s_51_10: i64 = (s_51_9 as i64);
        // C s_51_11: const #1s : i
        let s_51_11: i128 = 1;
        // D s_51_12: cast zx s_51_10 -> i
        let s_51_12: i128 = (i128::try_from(s_51_10).unwrap());
        // D s_51_13: sub s_51_12 s_51_11
        let s_51_13: i128 = ((s_51_12) - (s_51_11));
        // D s_51_14: cast reint s_51_13 -> i64
        let s_51_14: i64 = (s_51_13 as i64);
        // C s_51_15: const #0s : i
        let s_51_15: i128 = 0;
        // D s_51_16: cast zx s_51_14 -> i
        let s_51_16: i128 = (i128::try_from(s_51_14).unwrap());
        // D s_51_17: cmp-le s_51_15 s_51_16
        let s_51_17: bool = ((s_51_15) <= (s_51_16));
        // N s_51_18: branch s_51_17 b54 b52
        if s_51_17 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#150098 <= s_52_0
        fn_state.gs_150098 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#150098:u8
        let s_53_0: bool = fn_state.gs_150098;
        // N s_53_1: assert s_53_0
        let s_53_1: () = assert!(s_53_0);
        // D s_53_2: read-var fsize:i64
        let s_53_2: i64 = fn_state.fsize;
        // D s_53_3: cast zx s_53_2 -> i
        let s_53_3: i128 = (i128::try_from(s_53_2).unwrap());
        // D s_53_4: cast reint s_53_3 -> i64
        let s_53_4: i64 = (s_53_3 as i64);
        // C s_53_5: const #1s : i
        let s_53_5: i128 = 1;
        // D s_53_6: read-var fsize:i64
        let s_53_6: i64 = fn_state.fsize;
        // D s_53_7: cast zx s_53_6 -> i
        let s_53_7: i128 = (i128::try_from(s_53_6).unwrap());
        // D s_53_8: sub s_53_7 s_53_5
        let s_53_8: i128 = ((s_53_7) - (s_53_5));
        // D s_53_9: cast reint s_53_8 -> i64
        let s_53_9: i64 = (s_53_8 as i64);
        // C s_53_10: const #0s : i
        let s_53_10: i128 = 0;
        // D s_53_11: cast zx s_53_9 -> i
        let s_53_11: i128 = (i128::try_from(s_53_9).unwrap());
        // D s_53_12: read-var intvalshadow#1303:bv
        let s_53_12: Bits = fn_state.intvalshadow_1303;
        // C s_53_13: const #1s : i64
        let s_53_13: i64 = 1;
        // C s_53_14: cast zx s_53_13 -> i
        let s_53_14: i128 = (i128::try_from(s_53_13).unwrap());
        // D s_53_15: sub s_53_11 s_53_10
        let s_53_15: i128 = ((s_53_11) - (s_53_10));
        // D s_53_16: add s_53_15 s_53_14
        let s_53_16: i128 = (s_53_15 + s_53_14);
        // D s_53_17: bit-extract s_53_12 s_53_10 s_53_16
        let s_53_17: Bits = (Bits::new(
            ((s_53_12) >> (s_53_10)).value(),
            u16::try_from(s_53_16).unwrap(),
        ));
        // D s_53_18: read-var d:i64
        let s_53_18: i64 = fn_state.d;
        // D s_53_19: cast zx s_53_18 -> i
        let s_53_19: i128 = (i128::try_from(s_53_18).unwrap());
        // D s_53_20: read-var part:i64
        let s_53_20: i64 = fn_state.part;
        // D s_53_21: cast zx s_53_20 -> i
        let s_53_21: i128 = (i128::try_from(s_53_20).unwrap());
        // D s_53_22: cast zx s_53_4 -> i
        let s_53_22: i128 = (i128::try_from(s_53_4).unwrap());
        // D s_53_23: call Vpart_set(s_53_19, s_53_21, s_53_22, s_53_17)
        let s_53_23: () = Vpart_set(state, tracer, s_53_19, s_53_21, s_53_22, s_53_17);
        // N s_53_24: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var fsize:i64
        let s_54_0: i64 = fn_state.fsize;
        // D s_54_1: cast zx s_54_0 -> i
        let s_54_1: i128 = (i128::try_from(s_54_0).unwrap());
        // D s_54_2: call __id(s_54_1)
        let s_54_2: i128 = u__id(state, tracer, s_54_1);
        // D s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: const #1s : i
        let s_54_4: i128 = 1;
        // D s_54_5: cast zx s_54_3 -> i
        let s_54_5: i128 = (i128::try_from(s_54_3).unwrap());
        // D s_54_6: sub s_54_5 s_54_4
        let s_54_6: i128 = ((s_54_5) - (s_54_4));
        // D s_54_7: cast reint s_54_6 -> i64
        let s_54_7: i64 = (s_54_6 as i64);
        // D s_54_8: read-var intsizeshadow#1296:i64
        let s_54_8: i64 = fn_state.intsizeshadow_1296;
        // D s_54_9: cast zx s_54_8 -> i
        let s_54_9: i128 = (i128::try_from(s_54_8).unwrap());
        // D s_54_10: call __id(s_54_9)
        let s_54_10: i128 = u__id(state, tracer, s_54_9);
        // D s_54_11: cast reint s_54_10 -> i64
        let s_54_11: i64 = (s_54_10 as i64);
        // D s_54_12: cast zx s_54_7 -> i
        let s_54_12: i128 = (i128::try_from(s_54_7).unwrap());
        // D s_54_13: cast zx s_54_11 -> i
        let s_54_13: i128 = (i128::try_from(s_54_11).unwrap());
        // D s_54_14: cmp-lt s_54_12 s_54_13
        let s_54_14: bool = ((s_54_12) < (s_54_13));
        // D s_54_15: write-var gs#150098 <= s_54_14
        fn_state.gs_150098 = s_54_14;
        // N s_54_16: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #4u : u32
        let s_55_0: u32 = 4;
        // D s_55_1: read-var op:u32
        let s_55_1: u32 = fn_state.op;
        // D s_55_2: cmp-eq s_55_0 s_55_1
        let s_55_2: bool = ((s_55_0) == (s_55_1));
        // D s_55_3: not s_55_2
        let s_55_3: bool = !s_55_2;
        // N s_55_4: branch s_55_3 b58 b56
        if s_55_3 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var fsize:i64
        let s_56_0: i64 = fn_state.fsize;
        // D s_56_1: cast zx s_56_0 -> i
        let s_56_1: i128 = (i128::try_from(s_56_0).unwrap());
        // D s_56_2: cast reint s_56_1 -> i64
        let s_56_2: i64 = (s_56_1 as i64);
        // D s_56_3: read-var n:i64
        let s_56_3: i64 = fn_state.n;
        // D s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // D s_56_5: call V_read(s_56_4, s_56_2)
        let s_56_5: Bits = V_read(state, tracer, s_56_4, s_56_2);
        // D s_56_6: read-var fsize:i64
        let s_56_6: i64 = fn_state.fsize;
        // D s_56_7: cast zx s_56_6 -> i
        let s_56_7: i128 = (i128::try_from(s_56_6).unwrap());
        // D s_56_8: call __id(s_56_7)
        let s_56_8: i128 = u__id(state, tracer, s_56_7);
        // D s_56_9: cast reint s_56_8 -> i64
        let s_56_9: i64 = (s_56_8 as i64);
        // C s_56_10: const #64s : i
        let s_56_10: i128 = 64;
        // D s_56_11: cast zx s_56_9 -> i
        let s_56_11: i128 = (i128::try_from(s_56_9).unwrap());
        // D s_56_12: cmp-eq s_56_11 s_56_10
        let s_56_12: bool = ((s_56_11) == (s_56_10));
        // N s_56_13: assert s_56_12
        let s_56_13: () = assert!(s_56_12);
        // D s_56_14: read-var intsizeshadow#1296:i64
        let s_56_14: i64 = fn_state.intsizeshadow_1296;
        // D s_56_15: cast zx s_56_14 -> i
        let s_56_15: i128 = (i128::try_from(s_56_14).unwrap());
        // D s_56_16: call __id(s_56_15)
        let s_56_16: i128 = u__id(state, tracer, s_56_15);
        // D s_56_17: cast reint s_56_16 -> i64
        let s_56_17: i64 = (s_56_16 as i64);
        // C s_56_18: const #32s : i
        let s_56_18: i128 = 32;
        // D s_56_19: cast zx s_56_17 -> i
        let s_56_19: i128 = (i128::try_from(s_56_17).unwrap());
        // D s_56_20: cmp-eq s_56_19 s_56_18
        let s_56_20: bool = ((s_56_19) == (s_56_18));
        // N s_56_21: assert s_56_20
        let s_56_21: () = assert!(s_56_20);
        // D s_56_22: read-var fpcr:struct
        let s_56_22: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // C s_56_23: const #1u : u8
        let s_56_23: bool = true;
        // D s_56_24: read-var intsizeshadow#1296:i64
        let s_56_24: i64 = fn_state.intsizeshadow_1296;
        // D s_56_25: call FPToFixedJS(s_56_5, s_56_22, s_56_23, s_56_24)
        let s_56_25: ProductTypef506aa96a892fe52 = FPToFixedJS(
            state,
            tracer,
            s_56_5,
            s_56_22,
            s_56_23,
            s_56_24,
        );
        // D s_56_26: write-var gs#691236 <= s_56_25
        fn_state.gs_691236 = s_56_25;
        // N s_56_27: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#691236.0:struct
        let s_57_0: Bits = fn_state.gs_691236._0;
        // D s_57_1: cast reint s_57_0 -> u32
        let s_57_1: u32 = (s_57_0.value() as u32);
        // D s_57_2: read-var gs#691236.1:struct
        let s_57_2: bool = fn_state.gs_691236._1;
        // D s_57_3: cast zx s_57_1 -> bv
        let s_57_3: Bits = Bits::new(s_57_1 as u128, 32u16);
        // C s_57_4: const #0u : u8
        let s_57_4: bool = false;
        // C s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // D s_57_6: cast zx s_57_2 -> bv
        let s_57_6: Bits = Bits::new(s_57_2 as u128, 1u16);
        // C s_57_7: cast reint s_57_5 -> u128
        let s_57_7: u128 = (s_57_5.value() as u128);
        // D s_57_8: size-of s_57_5
        let s_57_8: u16 = s_57_5.length();
        // D s_57_9: cast reint s_57_6 -> u128
        let s_57_9: u128 = (s_57_6.value() as u128);
        // D s_57_10: size-of s_57_6
        let s_57_10: u16 = s_57_6.length();
        // D s_57_11: lsl s_57_7 s_57_10
        let s_57_11: u128 = s_57_7 << s_57_10;
        // D s_57_12: or s_57_11 s_57_9
        let s_57_12: u128 = ((s_57_11) | (s_57_9));
        // D s_57_13: add s_57_8 s_57_10
        let s_57_13: u16 = (s_57_8 + s_57_10);
        // D s_57_14: create-bits s_57_12 s_57_13
        let s_57_14: Bits = Bits::new(s_57_12, s_57_13);
        // D s_57_15: cast reint s_57_14 -> u8
        let s_57_15: u8 = (s_57_14.value() as u8);
        // D s_57_16: cast zx s_57_15 -> bv
        let s_57_16: Bits = Bits::new(s_57_15 as u128, 2u16);
        // C s_57_17: const #0u : u8
        let s_57_17: u8 = 0;
        // C s_57_18: cast zx s_57_17 -> bv
        let s_57_18: Bits = Bits::new(s_57_17 as u128, 2u16);
        // D s_57_19: cast reint s_57_16 -> u128
        let s_57_19: u128 = (s_57_16.value() as u128);
        // D s_57_20: size-of s_57_16
        let s_57_20: u16 = s_57_16.length();
        // C s_57_21: cast reint s_57_18 -> u128
        let s_57_21: u128 = (s_57_18.value() as u128);
        // D s_57_22: size-of s_57_18
        let s_57_22: u16 = s_57_18.length();
        // D s_57_23: lsl s_57_19 s_57_22
        let s_57_23: u128 = s_57_19 << s_57_22;
        // D s_57_24: or s_57_23 s_57_21
        let s_57_24: u128 = ((s_57_23) | (s_57_21));
        // D s_57_25: add s_57_20 s_57_22
        let s_57_25: u16 = (s_57_20 + s_57_22);
        // D s_57_26: create-bits s_57_24 s_57_25
        let s_57_26: Bits = Bits::new(s_57_24, s_57_25);
        // D s_57_27: cast reint s_57_26 -> u8
        let s_57_27: u8 = (s_57_26.value() as u8);
        // C s_57_28: const #3s : i
        let s_57_28: i128 = 3;
        // D s_57_29: cast zx s_57_27 -> bv
        let s_57_29: Bits = Bits::new(s_57_27 as u128, 4u16);
        // C s_57_30: const #1s : i64
        let s_57_30: i64 = 1;
        // C s_57_31: cast zx s_57_30 -> i
        let s_57_31: i128 = (i128::try_from(s_57_30).unwrap());
        // C s_57_32: const #0s : i
        let s_57_32: i128 = 0;
        // C s_57_33: add s_57_32 s_57_31
        let s_57_33: i128 = (s_57_32 + s_57_31);
        // D s_57_34: bit-extract s_57_29 s_57_28 s_57_33
        let s_57_34: Bits = (Bits::new(
            ((s_57_29) >> (s_57_28)).value(),
            u16::try_from(s_57_33).unwrap(),
        ));
        // D s_57_35: cast reint s_57_34 -> u8
        let s_57_35: bool = ((s_57_34.value()) != 0);
        // C s_57_36: const #16984u : u32
        let s_57_36: u32 = 16984;
        // N s_57_37: write-reg s_57_36 <= s_57_35
        let s_57_37: () = {
            state.write_register::<bool>(s_57_36 as isize, s_57_35);
            tracer.write_register(s_57_36 as isize, s_57_35);
        };
        // C s_57_38: const #2s : i
        let s_57_38: i128 = 2;
        // D s_57_39: cast zx s_57_27 -> bv
        let s_57_39: Bits = Bits::new(s_57_27 as u128, 4u16);
        // C s_57_40: const #1s : i64
        let s_57_40: i64 = 1;
        // C s_57_41: cast zx s_57_40 -> i
        let s_57_41: i128 = (i128::try_from(s_57_40).unwrap());
        // C s_57_42: const #0s : i
        let s_57_42: i128 = 0;
        // C s_57_43: add s_57_42 s_57_41
        let s_57_43: i128 = (s_57_42 + s_57_41);
        // D s_57_44: bit-extract s_57_39 s_57_38 s_57_43
        let s_57_44: Bits = (Bits::new(
            ((s_57_39) >> (s_57_38)).value(),
            u16::try_from(s_57_43).unwrap(),
        ));
        // D s_57_45: cast reint s_57_44 -> u8
        let s_57_45: bool = ((s_57_44.value()) != 0);
        // C s_57_46: const #16997u : u32
        let s_57_46: u32 = 16997;
        // N s_57_47: write-reg s_57_46 <= s_57_45
        let s_57_47: () = {
            state.write_register::<bool>(s_57_46 as isize, s_57_45);
            tracer.write_register(s_57_46 as isize, s_57_45);
        };
        // C s_57_48: const #1s : i
        let s_57_48: i128 = 1;
        // D s_57_49: cast zx s_57_27 -> bv
        let s_57_49: Bits = Bits::new(s_57_27 as u128, 4u16);
        // C s_57_50: const #1s : i64
        let s_57_50: i64 = 1;
        // C s_57_51: cast zx s_57_50 -> i
        let s_57_51: i128 = (i128::try_from(s_57_50).unwrap());
        // C s_57_52: const #0s : i
        let s_57_52: i128 = 0;
        // C s_57_53: add s_57_52 s_57_51
        let s_57_53: i128 = (s_57_52 + s_57_51);
        // D s_57_54: bit-extract s_57_49 s_57_48 s_57_53
        let s_57_54: Bits = (Bits::new(
            ((s_57_49) >> (s_57_48)).value(),
            u16::try_from(s_57_53).unwrap(),
        ));
        // D s_57_55: cast reint s_57_54 -> u8
        let s_57_55: bool = ((s_57_54.value()) != 0);
        // C s_57_56: const #16971u : u32
        let s_57_56: u32 = 16971;
        // N s_57_57: write-reg s_57_56 <= s_57_55
        let s_57_57: () = {
            state.write_register::<bool>(s_57_56 as isize, s_57_55);
            tracer.write_register(s_57_56 as isize, s_57_55);
        };
        // C s_57_58: const #0s : i
        let s_57_58: i128 = 0;
        // D s_57_59: cast zx s_57_27 -> bv
        let s_57_59: Bits = Bits::new(s_57_27 as u128, 4u16);
        // C s_57_60: const #1s : i64
        let s_57_60: i64 = 1;
        // C s_57_61: cast zx s_57_60 -> i
        let s_57_61: i128 = (i128::try_from(s_57_60).unwrap());
        // C s_57_62: const #0s : i
        let s_57_62: i128 = 0;
        // C s_57_63: add s_57_62 s_57_61
        let s_57_63: i128 = (s_57_62 + s_57_61);
        // D s_57_64: bit-extract s_57_59 s_57_58 s_57_63
        let s_57_64: Bits = (Bits::new(
            ((s_57_59) >> (s_57_58)).value(),
            u16::try_from(s_57_63).unwrap(),
        ));
        // D s_57_65: cast reint s_57_64 -> u8
        let s_57_65: bool = ((s_57_64.value()) != 0);
        // C s_57_66: const #16996u : u32
        let s_57_66: u32 = 16996;
        // N s_57_67: write-reg s_57_66 <= s_57_65
        let s_57_67: () = {
            state.write_register::<bool>(s_57_66 as isize, s_57_65);
            tracer.write_register(s_57_66 as isize, s_57_65);
        };
        // D s_57_68: read-var intsizeshadow#1296:i64
        let s_57_68: i64 = fn_state.intsizeshadow_1296;
        // D s_57_69: cast zx s_57_68 -> i
        let s_57_69: i128 = (i128::try_from(s_57_68).unwrap());
        // D s_57_70: cast reint s_57_69 -> i64
        let s_57_70: i64 = (s_57_69 as i64);
        // D s_57_71: read-var d:i64
        let s_57_71: i64 = fn_state.d;
        // D s_57_72: cast zx s_57_71 -> i
        let s_57_72: i128 = (i128::try_from(s_57_71).unwrap());
        // D s_57_73: call X_set(s_57_72, s_57_70, s_57_3)
        let s_57_73: () = X_set(state, tracer, s_57_72, s_57_70, s_57_3);
        // N s_57_74: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #128s : i64
        let s_59_0: i64 = 128;
        // D s_59_1: write-var ga#253718 <= s_59_0
        fn_state.ga_253718 = s_59_0;
        // N s_59_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var merge:u8
        let s_60_0: bool = fn_state.merge;
        // D s_60_1: write-var gs#150056 <= s_60_0
        fn_state.gs_150056 = s_60_0;
        // N s_60_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call CheckFPAdvSIMDEnabled64(s_61_0)
        let s_61_1: () = CheckFPAdvSIMDEnabled64(state, tracer, s_61_0);
        // N s_61_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
