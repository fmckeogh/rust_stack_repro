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
use X_set::*;
use CheckFPEnabled64::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use u__id::*;
use IsMerging::*;
use V_set::*;
use FPToFixed::*;
use X_read::*;
use Zeros::*;
use FixedToFP::*;
use common::*;
pub fn execute_aarch64_instrs_float_convert_fix<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    fltsize: i64,
    fracbits: i64,
    intsize: i64,
    n: i64,
    op: u32,
    rounding: u32,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_151891: bool,
        fpcr: ProductType5c790c8ef59cc8b2,
        fsize: i64,
        intsizeshadow_1352: i64,
        fltval: Bits,
        gs_151886: bool,
        intvalshadow_1355: Bits,
        fltvalshadow_1354: Bits,
        ga_254779: Bits,
        gs_151888: bool,
        ga_254778: i64,
        gs_151893: bool,
        merge: bool,
        intvalshadow_1356: Bits,
        fltsizeshadow_1353: i64,
        gs_151880: bool,
        ga_254760: i64,
        gs_151894: bool,
        d: i64,
        fltsize: i64,
        fracbits: i64,
        intsize: i64,
        n: i64,
        op: u32,
        rounding: u32,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        fltsize,
        fracbits,
        intsize,
        n,
        op,
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
        // D s_0_3: write-var intsizeshadow#1352 <= s_0_2
        fn_state.intsizeshadow_1352 = s_0_2;
        // D s_0_4: read-var fltsize:i64
        let s_0_4: i64 = fn_state.fltsize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var fltsizeshadow#1353 <= s_0_6
        fn_state.fltsizeshadow_1353 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPEnabled64(s_0_8)
        let s_0_9: () = CheckFPEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call FPCR_read(s_1_0)
        let s_1_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_0);
        // D s_1_2: write-var fpcr <= s_1_1
        fn_state.fpcr = s_1_1;
        // D s_1_3: read-var fpcr:struct
        let s_1_3: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_1_4: call IsMerging(s_1_3)
        let s_1_4: bool = IsMerging(state, tracer, s_1_3);
        // D s_1_5: write-var merge <= s_1_4
        fn_state.merge = s_1_4;
        // D s_1_6: read-var op:u32
        let s_1_6: u32 = fn_state.op;
        // C s_1_7: const #1u : u32
        let s_1_7: u32 = 1;
        // D s_1_8: cmp-eq s_1_6 s_1_7
        let s_1_8: bool = ((s_1_6) == (s_1_7));
        // N s_1_9: branch s_1_8 b31 b2
        if s_1_8 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#151880 <= s_2_0
        fn_state.gs_151880 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#151880:u8
        let s_3_0: bool = fn_state.gs_151880;
        // N s_3_1: branch s_3_0 b30 b4
        if s_3_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var fltsizeshadow#1353:i64
        let s_4_0: i64 = fn_state.fltsizeshadow_1353;
        // D s_4_1: write-var ga#254760 <= s_4_0
        fn_state.ga_254760 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#254760:i64
        let s_5_0: i64 = fn_state.ga_254760;
        // D s_5_1: write-var fsize <= s_5_0
        fn_state.fsize = s_5_0;
        // C s_5_2: const #0u : u32
        let s_5_2: u32 = 0;
        // D s_5_3: read-var op:u32
        let s_5_3: u32 = fn_state.op;
        // D s_5_4: cmp-eq s_5_2 s_5_3
        let s_5_4: bool = ((s_5_2) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b23 b6
        if s_5_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var fsize:i64
        let s_6_0: i64 = fn_state.fsize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var n:i64
        let s_6_3: i64 = fn_state.n;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: call V_read(s_6_4, s_6_2)
        let s_6_5: Bits = V_read(state, tracer, s_6_4, s_6_2);
        // D s_6_6: write-var fltvalshadow#1354 <= s_6_5
        fn_state.fltvalshadow_1354 = s_6_5;
        // D s_6_7: read-var fsize:i64
        let s_6_7: i64 = fn_state.fsize;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: call __id(s_6_8)
        let s_6_9: i128 = u__id(state, tracer, s_6_8);
        // D s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // C s_6_11: const #16s : i
        let s_6_11: i128 = 16;
        // D s_6_12: cast zx s_6_10 -> i
        let s_6_12: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_13: cmp-eq s_6_12 s_6_11
        let s_6_13: bool = ((s_6_12) == (s_6_11));
        // N s_6_14: branch s_6_13 b22 b7
        if s_6_13 {
            return block_22(state, tracer, fn_state);
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
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #32s : i
        let s_7_4: i128 = 32;
        // D s_7_5: cast zx s_7_3 -> i
        let s_7_5: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_6: cmp-eq s_7_5 s_7_4
        let s_7_6: bool = ((s_7_5) == (s_7_4));
        // D s_7_7: write-var gs#151886 <= s_7_6
        fn_state.gs_151886 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#151886:u8
        let s_8_0: bool = fn_state.gs_151886;
        // N s_8_1: branch s_8_0 b21 b9
        if s_8_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var fsize:i64
        let s_9_0: i64 = fn_state.fsize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #64s : i
        let s_9_4: i128 = 64;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // D s_9_7: write-var gs#151888 <= s_9_6
        fn_state.gs_151888 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#151888:u8
        let s_10_0: bool = fn_state.gs_151888;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#151894 <= s_11_0
        fn_state.gs_151894 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#151894:u8
        let s_12_0: bool = fn_state.gs_151894;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var intsizeshadow#1352:i64
        let s_12_2: i64 = fn_state.intsizeshadow_1352;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: read-var fracbits:i64
        let s_12_5: i64 = fn_state.fracbits;
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: read-var fltvalshadow#1354:bv
        let s_12_7: Bits = fn_state.fltvalshadow_1354;
        // D s_12_8: read-var is_unsigned:u8
        let s_12_8: bool = fn_state.is_unsigned;
        // D s_12_9: read-var fpcr:struct
        let s_12_9: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_12_10: read-var rounding:u32
        let s_12_10: u32 = fn_state.rounding;
        // D s_12_11: call FPToFixed(s_12_7, s_12_6, s_12_8, s_12_9, s_12_10, s_12_4)
        let s_12_11: Bits = FPToFixed(
            state,
            tracer,
            s_12_7,
            s_12_6,
            s_12_8,
            s_12_9,
            s_12_10,
            s_12_4,
        );
        // D s_12_12: write-var intvalshadow#1355 <= s_12_11
        fn_state.intvalshadow_1355 = s_12_11;
        // N s_12_13: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var intsizeshadow#1352:i64
        let s_13_0: i64 = fn_state.intsizeshadow_1352;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var d:i64
        let s_13_3: i64 = fn_state.d;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: read-var intvalshadow#1355:bv
        let s_13_5: Bits = fn_state.intvalshadow_1355;
        // D s_13_6: call X_set(s_13_4, s_13_2, s_13_5)
        let s_13_6: () = X_set(state, tracer, s_13_4, s_13_2, s_13_5);
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var intsizeshadow#1352:i64
        let s_14_0: i64 = fn_state.intsizeshadow_1352;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #16s : i
        let s_14_4: i128 = 16;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // N s_14_7: branch s_14_6 b20 b15
        if s_14_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var intsizeshadow#1352:i64
        let s_15_0: i64 = fn_state.intsizeshadow_1352;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #32s : i
        let s_15_4: i128 = 32;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // D s_15_7: write-var gs#151891 <= s_15_6
        fn_state.gs_151891 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#151891:u8
        let s_16_0: bool = fn_state.gs_151891;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var intsizeshadow#1352:i64
        let s_17_0: i64 = fn_state.intsizeshadow_1352;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call __id(s_17_1)
        let s_17_2: i128 = u__id(state, tracer, s_17_1);
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: const #64s : i
        let s_17_4: i128 = 64;
        // D s_17_5: cast zx s_17_3 -> i
        let s_17_5: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_6: cmp-eq s_17_5 s_17_4
        let s_17_6: bool = ((s_17_5) == (s_17_4));
        // D s_17_7: write-var gs#151893 <= s_17_6
        fn_state.gs_151893 = s_17_6;
        // N s_17_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#151893:u8
        let s_18_0: bool = fn_state.gs_151893;
        // D s_18_1: write-var gs#151894 <= s_18_0
        fn_state.gs_151894 = s_18_0;
        // N s_18_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#151893 <= s_19_0
        fn_state.gs_151893 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#151891 <= s_20_0
        fn_state.gs_151891 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#151888 <= s_21_0
        fn_state.gs_151888 = s_21_0;
        // N s_21_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#151886 <= s_22_0
        fn_state.gs_151886 = s_22_0;
        // N s_22_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u32
        let s_23_0: u32 = 1;
        // D s_23_1: read-var op:u32
        let s_23_1: u32 = fn_state.op;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // N s_23_4: branch s_23_3 b29 b24
        if s_23_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var intsizeshadow#1352:i64
        let s_24_0: i64 = fn_state.intsizeshadow_1352;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: cast reint s_24_1 -> i64
        let s_24_2: i64 = (s_24_1 as i64);
        // D s_24_3: read-var n:i64
        let s_24_3: i64 = fn_state.n;
        // D s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_5: call X_read(s_24_4, s_24_2)
        let s_24_5: Bits = X_read(state, tracer, s_24_4, s_24_2);
        // D s_24_6: write-var intvalshadow#1356 <= s_24_5
        fn_state.intvalshadow_1356 = s_24_5;
        // D s_24_7: read-var merge:u8
        let s_24_7: bool = fn_state.merge;
        // N s_24_8: branch s_24_7 b28 b25
        if s_24_7 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var fsize:i64
        let s_25_0: i64 = fn_state.fsize;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call Zeros(s_25_1)
        let s_25_2: Bits = Zeros(state, tracer, s_25_1);
        // D s_25_3: write-var fltval <= s_25_2
        fn_state.fltval = s_25_2;
        // N s_25_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var fltsizeshadow#1353:i64
        let s_26_0: i64 = fn_state.fltsizeshadow_1353;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: cast reint s_26_1 -> i64
        let s_26_2: i64 = (s_26_1 as i64);
        // D s_26_3: write-var ga#254778 <= s_26_2
        fn_state.ga_254778 = s_26_2;
        // D s_26_4: read-var fltsizeshadow#1353:i64
        let s_26_4: i64 = fn_state.fltsizeshadow_1353;
        // D s_26_5: cast zx s_26_4 -> i
        let s_26_5: i128 = (i128::try_from(s_26_4).unwrap());
        // D s_26_6: cast reint s_26_5 -> i64
        let s_26_6: i64 = (s_26_5 as i64);
        // D s_26_7: read-var fracbits:i64
        let s_26_7: i64 = fn_state.fracbits;
        // D s_26_8: cast zx s_26_7 -> i
        let s_26_8: i128 = (i128::try_from(s_26_7).unwrap());
        // D s_26_9: read-var intvalshadow#1356:bv
        let s_26_9: Bits = fn_state.intvalshadow_1356;
        // D s_26_10: read-var is_unsigned:u8
        let s_26_10: bool = fn_state.is_unsigned;
        // D s_26_11: read-var fpcr:struct
        let s_26_11: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_26_12: read-var rounding:u32
        let s_26_12: u32 = fn_state.rounding;
        // D s_26_13: call FixedToFP(s_26_9, s_26_8, s_26_10, s_26_11, s_26_12, s_26_6)
        let s_26_13: Bits = FixedToFP(
            state,
            tracer,
            s_26_9,
            s_26_8,
            s_26_10,
            s_26_11,
            s_26_12,
            s_26_6,
        );
        // D s_26_14: write-var ga#254779 <= s_26_13
        fn_state.ga_254779 = s_26_13;
        // N s_26_15: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0s : i
        let s_27_0: i128 = 0;
        // D s_27_1: read-var ga#254778:i64
        let s_27_1: i64 = fn_state.ga_254778;
        // D s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (i128::try_from(s_27_1).unwrap());
        // D s_27_3: read-var fltval:bv
        let s_27_3: Bits = fn_state.fltval;
        // D s_27_4: read-var ga#254779:bv
        let s_27_4: Bits = fn_state.ga_254779;
        // D s_27_5: call Elem_set(s_27_3, s_27_0, s_27_2, s_27_4)
        let s_27_5: Bits = Elem_set(state, tracer, s_27_3, s_27_0, s_27_2, s_27_4);
        // D s_27_6: write-var fltval <= s_27_5
        fn_state.fltval = s_27_5;
        // D s_27_7: read-var fsize:i64
        let s_27_7: i64 = fn_state.fsize;
        // D s_27_8: cast zx s_27_7 -> i
        let s_27_8: i128 = (i128::try_from(s_27_7).unwrap());
        // D s_27_9: cast reint s_27_8 -> i64
        let s_27_9: i64 = (s_27_8 as i64);
        // D s_27_10: read-var d:i64
        let s_27_10: i64 = fn_state.d;
        // D s_27_11: cast zx s_27_10 -> i
        let s_27_11: i128 = (i128::try_from(s_27_10).unwrap());
        // D s_27_12: read-var fltval:bv
        let s_27_12: Bits = fn_state.fltval;
        // D s_27_13: call V_set(s_27_11, s_27_9, s_27_12)
        let s_27_13: () = V_set(state, tracer, s_27_11, s_27_9, s_27_12);
        // N s_27_14: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fsize:i64
        let s_28_0: i64 = fn_state.fsize;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: cast reint s_28_1 -> i64
        let s_28_2: i64 = (s_28_1 as i64);
        // D s_28_3: read-var d:i64
        let s_28_3: i64 = fn_state.d;
        // D s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_5: call V_read(s_28_4, s_28_2)
        let s_28_5: Bits = V_read(state, tracer, s_28_4, s_28_2);
        // D s_28_6: write-var fltval <= s_28_5
        fn_state.fltval = s_28_5;
        // N s_28_7: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #128s : i64
        let s_30_0: i64 = 128;
        // D s_30_1: write-var ga#254760 <= s_30_0
        fn_state.ga_254760 = s_30_0;
        // N s_30_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var merge:u8
        let s_31_0: bool = fn_state.merge;
        // D s_31_1: write-var gs#151880 <= s_31_0
        fn_state.gs_151880 = s_31_0;
        // N s_31_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
