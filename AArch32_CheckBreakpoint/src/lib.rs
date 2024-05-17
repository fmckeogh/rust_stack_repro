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
use Halt::*;
use S1TranslationRegime__1::*;
use u__id::*;
use AArch32_BreakpointMatch::*;
use ELUsingAArch32::*;
use HaltOnBreakpointOrWatchpoint::*;
use NumBreakpointsImplemented::*;
use common::*;
pub fn AArch32_CheckBreakpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    vaddress: u32,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        gs_30028: bool,
        gs_30033: bool,
        gs_30016: bool,
        val_match: bool,
        ga_23226: ProductType8b847afc727d5818,
        fault: ProductType1d757adad216cdef,
        match_i: bool,
        gs_30039: bool,
        mismatch_i: bool,
        gs_30022: i64,
        gs_30037: bool,
        i: i64,
        gs_30041: bool,
        mismatch: bool,
        gs_30029: bool,
        fault_in: ProductType1d757adad216cdef,
        vaddress: u32,
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
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // C s_0_4: const #2s : i
        let s_0_4: i128 = 2;
        // D s_0_5: read-var size:i
        let s_0_5: i128 = fn_state.size;
        // D s_0_6: cmp-eq s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) == (s_0_4));
        // N s_0_7: branch s_0_6 b29 b1
        if s_0_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_1_0: const #4s : i
        let s_1_0: i128 = 4;
        // D s_1_1: read-var size:i
        let s_1_1: i128 = fn_state.size;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // D s_1_3: write-var gs#30016 <= s_1_2
        fn_state.gs_30016 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var gs#30016:u8
        let s_2_0: bool = fn_state.gs_30016;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var fault_in:struct
        let s_2_2: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_2_3: write-var fault <= s_2_2
        fn_state.fault = s_2_2;
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // D s_2_5: write-var val_match <= s_2_4
        fn_state.val_match = s_2_4;
        // C s_2_6: const #0u : u8
        let s_2_6: bool = false;
        // D s_2_7: write-var mismatch <= s_2_6
        fn_state.mismatch = s_2_6;
        // C s_2_8: const #0s : i64
        let s_2_8: i64 = 0;
        // C s_2_9: const #() : ()
        let s_2_9: () = ();
        // S s_2_10: call NumBreakpointsImplemented(s_2_9)
        let s_2_10: i128 = NumBreakpointsImplemented(state, tracer, s_2_9);
        // C s_2_11: const #1s : i
        let s_2_11: i128 = 1;
        // S s_2_12: sub s_2_10 s_2_11
        let s_2_12: i128 = ((s_2_10) - (s_2_11));
        // S s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: write-var gs#30022 <= s_2_13
        fn_state.gs_30022 = s_2_13;
        // D s_2_15: write-var i <= s_2_8
        fn_state.i = s_2_8;
        // N s_2_16: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_3_0: read-var i:i64
        let s_3_0: i64 = fn_state.i;
        // D s_3_1: read-var gs#30022:i64
        let s_3_1: i64 = fn_state.gs_30022;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b17 b4
        if s_3_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // C s_4_3: const #16s : i
        let s_4_3: i128 = 16;
        // D s_4_4: cmp-lt s_4_2 s_4_3
        let s_4_4: bool = ((s_4_2) < (s_4_3));
        // N s_4_5: branch s_4_4 b16 b5
        if s_4_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_5_0: read-var size:i
        let s_5_0: i128 = fn_state.size;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // C s_5_3: const #2s : i
        let s_5_3: i128 = 2;
        // D s_5_4: cast zx s_5_2 -> i
        let s_5_4: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_5: cmp-eq s_5_4 s_5_3
        let s_5_5: bool = ((s_5_4) == (s_5_3));
        // N s_5_6: branch s_5_5 b15 b6
        if s_5_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_6_0: read-var size:i
        let s_6_0: i128 = fn_state.size;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #4s : i
        let s_6_3: i128 = 4;
        // D s_6_4: cast zx s_6_2 -> i
        let s_6_4: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_5: cmp-eq s_6_4 s_6_3
        let s_6_5: bool = ((s_6_4) == (s_6_3));
        // D s_6_6: write-var gs#30028 <= s_6_5
        fn_state.gs_30028 = s_6_5;
        // N s_6_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_7_0: read-var gs#30028:u8
        let s_7_0: bool = fn_state.gs_30028;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // D s_7_2: write-var gs#30029 <= s_7_1
        fn_state.gs_30029 = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_8_0: read-var gs#30029:u8
        let s_8_0: bool = fn_state.gs_30029;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var i:i64
        let s_8_2: i64 = fn_state.i;
        // D s_8_3: read-var vaddress:u32
        let s_8_3: u32 = fn_state.vaddress;
        // D s_8_4: read-var accdesc:struct
        let s_8_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_5: read-var size:i
        let s_8_5: i128 = fn_state.size;
        // D s_8_6: call AArch32_BreakpointMatch(s_8_2, s_8_3, s_8_4, s_8_5)
        let s_8_6: ProductType8b847afc727d5818 = AArch32_BreakpointMatch(
            state,
            tracer,
            s_8_2,
            s_8_3,
            s_8_4,
            s_8_5,
        );
        // D s_8_7: write-var ga#23226 <= s_8_6
        fn_state.ga_23226 = s_8_6;
        // D s_8_8: read-var ga#23226.0:struct
        let s_8_8: bool = fn_state.ga_23226._0;
        // D s_8_9: read-var ga#23226.1:struct
        let s_8_9: bool = fn_state.ga_23226._1;
        // D s_8_10: write-var match_i <= s_8_8
        fn_state.match_i = s_8_8;
        // D s_8_11: write-var mismatch_i <= s_8_9
        fn_state.mismatch_i = s_8_9;
        // D s_8_12: read-var val_match:u8
        let s_8_12: bool = fn_state.val_match;
        // N s_8_13: branch s_8_12 b14 b9
        if s_8_12 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_9_0: read-var match_i:u8
        let s_9_0: bool = fn_state.match_i;
        // D s_9_1: write-var gs#30033 <= s_9_0
        fn_state.gs_30033 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_10_0: read-var gs#30033:u8
        let s_10_0: bool = fn_state.gs_30033;
        // D s_10_1: write-var val_match <= s_10_0
        fn_state.val_match = s_10_0;
        // D s_10_2: read-var mismatch:u8
        let s_10_2: bool = fn_state.mismatch;
        // N s_10_3: branch s_10_2 b13 b11
        if s_10_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_11_0: read-var mismatch_i:u8
        let s_11_0: bool = fn_state.mismatch_i;
        // D s_11_1: write-var gs#30037 <= s_11_0
        fn_state.gs_30037 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_12_0: read-var gs#30037:u8
        let s_12_0: bool = fn_state.gs_30037;
        // D s_12_1: write-var mismatch <= s_12_0
        fn_state.mismatch = s_12_0;
        // D s_12_2: read-var i:i64
        let s_12_2: i64 = fn_state.i;
        // C s_12_3: const #1s : i64
        let s_12_3: i64 = 1;
        // D s_12_4: add s_12_2 s_12_3
        let s_12_4: i64 = (s_12_2 + s_12_3);
        // D s_12_5: write-var i <= s_12_4
        fn_state.i = s_12_4;
        // N s_12_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#30037 <= s_13_0
        fn_state.gs_30037 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#30033 <= s_14_0
        fn_state.gs_30033 = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#30028 <= s_15_0
        fn_state.gs_30028 = s_15_0;
        // N s_15_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#30029 <= s_16_0
        fn_state.gs_30029 = s_16_0;
        // N s_16_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_17_0: read-var val_match:u8
        let s_17_0: bool = fn_state.val_match;
        // N s_17_1: branch s_17_0 b28 b18
        if s_17_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#30039 <= s_18_0
        fn_state.gs_30039 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_19_0: read-var gs#30039:u8
        let s_19_0: bool = fn_state.gs_30039;
        // N s_19_1: branch s_19_0 b27 b20
        if s_19_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_20_0: read-var val_match:u8
        let s_20_0: bool = fn_state.val_match;
        // N s_20_1: branch s_20_0 b26 b21
        if s_20_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_21_0: read-var mismatch:u8
        let s_21_0: bool = fn_state.mismatch;
        // D s_21_1: write-var gs#30041 <= s_21_0
        fn_state.gs_30041 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_22_0: read-var gs#30041:u8
        let s_22_0: bool = fn_state.gs_30041;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_24_0: read-var fault:struct
        let s_24_0: ProductType1d757adad216cdef = fn_state.fault;
        // N s_24_1: return s_24_0
        return s_24_0;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_25_0: const #17u : u32
        let s_25_0: u32 = 17;
        // D s_25_1: write-var fault.16 <= s_25_0
        fn_state.fault._16 = s_25_0;
        // C s_25_2: const #1312u : u32
        let s_25_2: u32 = 1312;
        // D s_25_3: read-reg s_25_2:u8
        let s_25_3: u8 = {
            let value = state.read_register::<u8>(s_25_2 as isize);
            tracer.read_register(s_25_2 as isize, value);
            value
        };
        // D s_25_4: write-var fault.2 <= s_25_3
        fn_state.fault._2 = s_25_3;
        // N s_25_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#30041 <= s_26_0
        fn_state.gs_30041 = s_26_0;
        // N s_26_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_27_0: const #1096u : u32
        let s_27_0: u32 = 1096;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call Halt(s_27_1)
        let s_27_2: () = Halt(state, tracer, s_27_1);
        // N s_27_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaltOnBreakpointOrWatchpoint(s_28_0)
        let s_28_1: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_28_0);
        // D s_28_2: write-var gs#30039 <= s_28_1
        fn_state.gs_30039 = s_28_1;
        // N s_28_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#30016 <= s_29_0
        fn_state.gs_30016 = s_29_0;
        // N s_29_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
