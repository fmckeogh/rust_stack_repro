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
use u__id::*;
use ELUsingAArch32::*;
use AArch64_BreakpointMatch::*;
use Halt::*;
use S1TranslationRegime__1::*;
use UsingAArch32::*;
use HaltOnBreakpointOrWatchpoint::*;
use NumBreakpointsImplemented::*;
use common::*;
pub fn AArch64_CheckBreakpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    vaddress: u64,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        gs_16545: bool,
        gs_16570: bool,
        val_match: bool,
        mismatch_bp: bool,
        gs_16548: bool,
        fault: ProductType1d757adad216cdef,
        gs_16574: bool,
        gs_16563: bool,
        match_i: bool,
        gs_16572: bool,
        gs_16560: bool,
        gs_16546: bool,
        gs_16562: bool,
        ga_12355: ProductType8b847afc727d5818,
        i: i64,
        mismatch: bool,
        gs_16554: i64,
        gs_16573: bool,
        fault_in: ProductType1d757adad216cdef,
        vaddress: u64,
        accdesc: ProductType9878976b5bcce9c9,
        size: i128,
    }
    let fn_state = FunctionState {
        fault_in,
        vaddress,
        accdesc,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call S1TranslationRegime__1(s_0_0)
        let s_0_1: u8 = S1TranslationRegime__1(state, tracer, s_0_0);
        // S s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // S s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call UsingAArch32(s_0_5)
        let s_0_6: bool = UsingAArch32(state, tracer, s_0_5);
        // N s_0_7: branch s_0_6 b39 b1
        if s_0_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#16546 <= s_1_0
        fn_state.gs_16546 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var gs#16546:u8
        let s_2_0: bool = fn_state.gs_16546;
        // N s_2_1: branch s_2_0 b38 b3
        if s_2_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_3_0: const #4s : i
        let s_3_0: i128 = 4;
        // D s_3_1: read-var size:i
        let s_3_1: i128 = fn_state.size;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // D s_3_3: write-var gs#16548 <= s_3_2
        fn_state.gs_16548 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_4_0: read-var gs#16548:u8
        let s_4_0: bool = fn_state.gs_16548;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var fault_in:struct
        let s_4_2: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_4_3: write-var fault <= s_4_2
        fn_state.fault = s_4_2;
        // C s_4_4: const #0u : u8
        let s_4_4: bool = false;
        // D s_4_5: write-var val_match <= s_4_4
        fn_state.val_match = s_4_4;
        // C s_4_6: const #1u : u8
        let s_4_6: bool = true;
        // D s_4_7: write-var mismatch <= s_4_6
        fn_state.mismatch = s_4_6;
        // C s_4_8: const #0u : u8
        let s_4_8: bool = false;
        // D s_4_9: write-var mismatch_bp <= s_4_8
        fn_state.mismatch_bp = s_4_8;
        // C s_4_10: const #0s : i64
        let s_4_10: i64 = 0;
        // C s_4_11: const #() : ()
        let s_4_11: () = ();
        // S s_4_12: call NumBreakpointsImplemented(s_4_11)
        let s_4_12: i128 = NumBreakpointsImplemented(state, tracer, s_4_11);
        // C s_4_13: const #1s : i
        let s_4_13: i128 = 1;
        // S s_4_14: sub s_4_12 s_4_13
        let s_4_14: i128 = ((s_4_12) - (s_4_13));
        // S s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var gs#16554 <= s_4_15
        fn_state.gs_16554 = s_4_15;
        // D s_4_17: write-var i <= s_4_10
        fn_state.i = s_4_10;
        // N s_4_18: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // D s_5_1: read-var gs#16554:i64
        let s_5_1: i64 = fn_state.gs_16554;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b25 b6
        if s_5_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_6_0: read-var i:i64
        let s_6_0: i64 = fn_state.i;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // C s_6_3: const #64s : i
        let s_6_3: i128 = 64;
        // D s_6_4: cmp-lt s_6_2 s_6_3
        let s_6_4: bool = ((s_6_2) < (s_6_3));
        // N s_6_5: branch s_6_4 b24 b7
        if s_6_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_7_0: read-var size:i
        let s_7_0: i128 = fn_state.size;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // C s_7_3: const #2s : i
        let s_7_3: i128 = 2;
        // D s_7_4: cast zx s_7_2 -> i
        let s_7_4: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_5: cmp-eq s_7_4 s_7_3
        let s_7_5: bool = ((s_7_4) == (s_7_3));
        // N s_7_6: branch s_7_5 b23 b8
        if s_7_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_8_0: read-var size:i
        let s_8_0: i128 = fn_state.size;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // C s_8_3: const #4s : i
        let s_8_3: i128 = 4;
        // D s_8_4: cast zx s_8_2 -> i
        let s_8_4: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_5: cmp-eq s_8_4 s_8_3
        let s_8_5: bool = ((s_8_4) == (s_8_3));
        // D s_8_6: write-var gs#16560 <= s_8_5
        fn_state.gs_16560 = s_8_5;
        // N s_8_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_9_0: read-var gs#16560:u8
        let s_9_0: bool = fn_state.gs_16560;
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
    ) -> ProductType1d757adad216cdef {
        // D s_10_0: read-var size:i
        let s_10_0: i128 = fn_state.size;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // C s_10_3: const #4s : i
        let s_10_3: i128 = 4;
        // D s_10_4: cast zx s_10_2 -> i
        let s_10_4: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_5: cmp-eq s_10_4 s_10_3
        let s_10_5: bool = ((s_10_4) == (s_10_3));
        // D s_10_6: write-var gs#16562 <= s_10_5
        fn_state.gs_16562 = s_10_5;
        // N s_10_7: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_11_0: read-var gs#16562:u8
        let s_11_0: bool = fn_state.gs_16562;
        // D s_11_1: not s_11_0
        let s_11_1: bool = !s_11_0;
        // D s_11_2: write-var gs#16563 <= s_11_1
        fn_state.gs_16563 = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_12_0: read-var gs#16563:u8
        let s_12_0: bool = fn_state.gs_16563;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var i:i64
        let s_12_2: i64 = fn_state.i;
        // D s_12_3: read-var vaddress:u64
        let s_12_3: u64 = fn_state.vaddress;
        // D s_12_4: read-var accdesc:struct
        let s_12_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_5: read-var size:i
        let s_12_5: i128 = fn_state.size;
        // D s_12_6: call AArch64_BreakpointMatch(s_12_2, s_12_3, s_12_4, s_12_5)
        let s_12_6: ProductType8b847afc727d5818 = AArch64_BreakpointMatch(
            state,
            tracer,
            s_12_2,
            s_12_3,
            s_12_4,
            s_12_5,
        );
        // D s_12_7: write-var ga#12355 <= s_12_6
        fn_state.ga_12355 = s_12_6;
        // D s_12_8: read-var ga#12355.0:struct
        let s_12_8: bool = fn_state.ga_12355._0;
        // D s_12_9: read-var ga#12355.1:struct
        let s_12_9: bool = fn_state.ga_12355._1;
        // D s_12_10: write-var match_i <= s_12_8
        fn_state.match_i = s_12_8;
        // N s_12_11: branch s_12_9 b18 b13
        if s_12_9 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_13_0: read-var val_match:u8
        let s_13_0: bool = fn_state.val_match;
        // N s_13_1: branch s_13_0 b17 b14
        if s_13_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_14_0: read-var match_i:u8
        let s_14_0: bool = fn_state.match_i;
        // D s_14_1: write-var gs#16570 <= s_14_0
        fn_state.gs_16570 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_15_0: read-var gs#16570:u8
        let s_15_0: bool = fn_state.gs_16570;
        // D s_15_1: write-var val_match <= s_15_0
        fn_state.val_match = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_16_0: read-var i:i64
        let s_16_0: i64 = fn_state.i;
        // C s_16_1: const #1s : i64
        let s_16_1: i64 = 1;
        // D s_16_2: add s_16_0 s_16_1
        let s_16_2: i64 = (s_16_0 + s_16_1);
        // D s_16_3: write-var i <= s_16_2
        fn_state.i = s_16_2;
        // N s_16_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#16570 <= s_17_0
        fn_state.gs_16570 = s_17_0;
        // N s_17_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var mismatch_bp <= s_18_0
        fn_state.mismatch_bp = s_18_0;
        // D s_18_2: read-var mismatch:u8
        let s_18_2: bool = fn_state.mismatch;
        // N s_18_3: branch s_18_2 b21 b19
        if s_18_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#16572 <= s_19_0
        fn_state.gs_16572 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_20_0: read-var gs#16572:u8
        let s_20_0: bool = fn_state.gs_16572;
        // D s_20_1: write-var mismatch <= s_20_0
        fn_state.mismatch = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_21_0: read-var match_i:u8
        let s_21_0: bool = fn_state.match_i;
        // D s_21_1: not s_21_0
        let s_21_1: bool = !s_21_0;
        // D s_21_2: write-var gs#16572 <= s_21_1
        fn_state.gs_16572 = s_21_1;
        // N s_21_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#16562 <= s_22_0
        fn_state.gs_16562 = s_22_0;
        // N s_22_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#16560 <= s_23_0
        fn_state.gs_16560 = s_23_0;
        // N s_23_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#16563 <= s_24_0
        fn_state.gs_16563 = s_24_0;
        // N s_24_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_25_0: read-var val_match:u8
        let s_25_0: bool = fn_state.val_match;
        // N s_25_1: branch s_25_0 b37 b26
        if s_25_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_26_0: read-var mismatch:u8
        let s_26_0: bool = fn_state.mismatch;
        // N s_26_1: branch s_26_0 b36 b27
        if s_26_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#16573 <= s_27_0
        fn_state.gs_16573 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_28_0: read-var gs#16573:u8
        let s_28_0: bool = fn_state.gs_16573;
        // D s_28_1: write-var gs#16574 <= s_28_0
        fn_state.gs_16574 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_29_0: read-var gs#16574:u8
        let s_29_0: bool = fn_state.gs_16574;
        // N s_29_1: branch s_29_0 b32 b30
        if s_29_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_31_0: read-var fault:struct
        let s_31_0: ProductType1d757adad216cdef = fn_state.fault;
        // N s_31_1: return s_31_0
        return s_31_0;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_32_0: const #17u : u32
        let s_32_0: u32 = 17;
        // D s_32_1: write-var fault.16 <= s_32_0
        fn_state.fault._16 = s_32_0;
        // C s_32_2: const #() : ()
        let s_32_2: () = ();
        // S s_32_3: call HaltOnBreakpointOrWatchpoint(s_32_2)
        let s_32_3: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_32_2);
        // N s_32_4: branch s_32_3 b35 b33
        if s_32_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_34_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_35_0: const #1096u : u32
        let s_35_0: u32 = 1096;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call Halt(s_35_1)
        let s_35_2: () = Halt(state, tracer, s_35_1);
        // N s_35_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_36_0: read-var mismatch_bp:u8
        let s_36_0: bool = fn_state.mismatch_bp;
        // D s_36_1: write-var gs#16573 <= s_36_0
        fn_state.gs_16573 = s_36_0;
        // N s_36_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#16574 <= s_37_0
        fn_state.gs_16574 = s_37_0;
        // N s_37_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#16548 <= s_38_0
        fn_state.gs_16548 = s_38_0;
        // N s_38_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_39_0: const #2s : i
        let s_39_0: i128 = 2;
        // D s_39_1: read-var size:i
        let s_39_1: i128 = fn_state.size;
        // D s_39_2: cmp-eq s_39_1 s_39_0
        let s_39_2: bool = ((s_39_1) == (s_39_0));
        // N s_39_3: branch s_39_2 b42 b40
        if s_39_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_40_0: const #4s : i
        let s_40_0: i128 = 4;
        // D s_40_1: read-var size:i
        let s_40_1: i128 = fn_state.size;
        // D s_40_2: cmp-eq s_40_1 s_40_0
        let s_40_2: bool = ((s_40_1) == (s_40_0));
        // D s_40_3: write-var gs#16545 <= s_40_2
        fn_state.gs_16545 = s_40_2;
        // N s_40_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_41_0: read-var gs#16545:u8
        let s_41_0: bool = fn_state.gs_16545;
        // D s_41_1: write-var gs#16546 <= s_41_0
        fn_state.gs_16546 = s_41_0;
        // N s_41_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#16545 <= s_42_0
        fn_state.gs_16545 = s_42_0;
        // N s_42_2: jump b41
        return block_41(state, tracer, fn_state);
    }
}
