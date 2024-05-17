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
use PendSErrorInterrupt::*;
use AArch32_Abort::*;
use NoFault__1::*;
use IsExternalSyncAbort__1::*;
use AArch64_Abort::*;
use IsExternalAbortTakenSynchronously::*;
use UsingAArch32::*;
use HaveRASExt::*;
use common::*;
pub fn HandleExternalAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memretstatus: ProductTypef8c3639b88223255,
    iswrite: bool,
    memaddrdesc: ProductTypece7c66ccb2cab13e,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_10038: bool,
        gs_10039: bool,
        gs_10037: bool,
        fault: ProductType1d757adad216cdef,
        gs_10036: bool,
        gs_10041: bool,
        memretstatus: ProductTypef8c3639b88223255,
        iswrite: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        memretstatus,
        iswrite,
        memaddrdesc,
        size,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var memretstatus.2:struct
        let s_0_0: u32 = fn_state.memretstatus._2;
        // C s_0_1: const #8u : u32
        let s_0_1: u32 = 8;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b27 b1
        if s_0_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var memretstatus.2:struct
        let s_1_0: u32 = fn_state.memretstatus._2;
        // C s_1_1: const #15u : u32
        let s_1_1: u32 = 15;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var gs#10036 <= s_1_2
        fn_state.gs_10036 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#10036:u8
        let s_2_0: bool = fn_state.gs_10036;
        // N s_2_1: branch s_2_0 b26 b3
        if s_2_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveRASExt(s_3_0)
        let s_3_1: bool = HaveRASExt(state, tracer, s_3_0);
        // S s_3_2: not s_3_1
        let s_3_2: bool = !s_3_1;
        // N s_3_3: branch s_3_2 b22 b4
        if s_3_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#10038 <= s_4_0
        fn_state.gs_10038 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#10038:u8
        let s_5_0: bool = fn_state.gs_10038;
        // D s_5_1: write-var gs#10039 <= s_5_0
        fn_state.gs_10039 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#10039:u8
        let s_6_0: bool = fn_state.gs_10039;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var accdesc:struct
        let s_6_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_3: call NoFault__1(s_6_2)
        let s_6_3: ProductType1d757adad216cdef = NoFault__1(state, tracer, s_6_2);
        // D s_6_4: write-var fault <= s_6_3
        fn_state.fault = s_6_3;
        // D s_6_5: read-var memretstatus.2:struct
        let s_6_5: u32 = fn_state.memretstatus._2;
        // D s_6_6: write-var fault.16 <= s_6_5
        fn_state.fault._16 = s_6_5;
        // D s_6_7: read-var iswrite:u8
        let s_6_7: bool = fn_state.iswrite;
        // D s_6_8: write-var fault.19 <= s_6_7
        fn_state.fault._19 = s_6_7;
        // D s_6_9: read-var memretstatus.0:struct
        let s_6_9: bool = fn_state.memretstatus._0;
        // D s_6_10: write-var fault.5 <= s_6_9
        fn_state.fault._5 = s_6_9;
        // D s_6_11: read-var fault:struct
        let s_6_11: ProductType1d757adad216cdef = fn_state.fault;
        // D s_6_12: call IsExternalSyncAbort__1(s_6_11)
        let s_6_12: bool = IsExternalSyncAbort__1(state, tracer, s_6_11);
        // N s_6_13: branch s_6_12 b21 b7
        if s_6_12 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#10041 <= s_7_0
        fn_state.gs_10041 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#10041:u8
        let s_8_0: bool = fn_state.gs_10041;
        // N s_8_1: branch s_8_0 b18 b9
        if s_8_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveRASExt(s_10_0)
        let s_10_1: bool = HaveRASExt(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b17 b11
        if s_10_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var fault:struct
        let s_12_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_12_1: call IsExternalSyncAbort__1(s_12_0)
        let s_12_1: bool = IsExternalSyncAbort__1(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b14 b13
        if s_12_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var fault:struct
        let s_13_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_13_1: call PendSErrorInterrupt(s_13_0)
        let s_13_1: () = PendSErrorInterrupt(state, tracer, s_13_0);
        // N s_13_2: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call UsingAArch32(s_14_0)
        let s_14_1: bool = UsingAArch32(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b16 b15
        if s_14_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var memaddrdesc.7:struct
        let s_15_0: u64 = fn_state.memaddrdesc._7;
        // D s_15_1: read-var fault:struct
        let s_15_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_2: call AArch64_Abort(s_15_0, s_15_1)
        let s_15_2: () = AArch64_Abort(state, tracer, s_15_0, s_15_1);
        // N s_15_3: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var memaddrdesc.7:struct
        let s_16_0: u64 = fn_state.memaddrdesc._7;
        // C s_16_1: const #0s : i
        let s_16_1: i128 = 0;
        // D s_16_2: cast zx s_16_0 -> bv
        let s_16_2: Bits = Bits::new(s_16_0 as u128, 64u16);
        // C s_16_3: const #1s : i64
        let s_16_3: i64 = 1;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #31s : i
        let s_16_5: i128 = 31;
        // C s_16_6: add s_16_5 s_16_4
        let s_16_6: i128 = (s_16_5 + s_16_4);
        // D s_16_7: bit-extract s_16_2 s_16_1 s_16_6
        let s_16_7: Bits = (Bits::new(
            ((s_16_2) >> (s_16_1)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D s_16_8: cast reint s_16_7 -> u32
        let s_16_8: u32 = (s_16_7.value() as u32);
        // D s_16_9: read-var fault:struct
        let s_16_9: ProductType1d757adad216cdef = fn_state.fault;
        // D s_16_10: call AArch32_Abort(s_16_8, s_16_9)
        let s_16_10: () = AArch32_Abort(state, tracer, s_16_8, s_16_9);
        // N s_16_11: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var memretstatus.1:struct
        let s_17_0: u32 = fn_state.memretstatus._1;
        // D s_17_1: write-var fault.10 <= s_17_0
        fn_state.fault._10 = s_17_0;
        // N s_17_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var fault.16:struct
        let s_18_0: u32 = fn_state.fault._16;
        // C s_18_1: const #10u : u32
        let s_18_1: u32 = 10;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b20 b19
        if s_18_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #15u : u32
        let s_19_0: u32 = 15;
        // D s_19_1: write-var fault.16 <= s_19_0
        fn_state.fault._16 = s_19_0;
        // N s_19_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #14u : u32
        let s_20_0: u32 = 14;
        // D s_20_1: write-var fault.16 <= s_20_0
        fn_state.fault._16 = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var memretstatus:struct
        let s_21_0: ProductTypef8c3639b88223255 = fn_state.memretstatus;
        // D s_21_1: read-var iswrite:u8
        let s_21_1: bool = fn_state.iswrite;
        // D s_21_2: read-var memaddrdesc:struct
        let s_21_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_21_3: read-var size:i
        let s_21_3: i128 = fn_state.size;
        // D s_21_4: read-var accdesc:struct
        let s_21_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_21_5: call IsExternalAbortTakenSynchronously(s_21_0, s_21_1, s_21_2, s_21_3, s_21_4)
        let s_21_5: bool = IsExternalAbortTakenSynchronously(
            state,
            tracer,
            s_21_0,
            s_21_1,
            s_21_2,
            s_21_3,
            s_21_4,
        );
        // D s_21_6: not s_21_5
        let s_21_6: bool = !s_21_5;
        // D s_21_7: write-var gs#10041 <= s_21_6
        fn_state.gs_10041 = s_21_6;
        // N s_21_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var memretstatus.2:struct
        let s_22_0: u32 = fn_state.memretstatus._2;
        // C s_22_1: const #10u : u32
        let s_22_1: u32 = 10;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // N s_22_3: branch s_22_2 b25 b23
        if s_22_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var memretstatus.2:struct
        let s_23_0: u32 = fn_state.memretstatus._2;
        // C s_23_1: const #14u : u32
        let s_23_1: u32 = 14;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: write-var gs#10037 <= s_23_2
        fn_state.gs_10037 = s_23_2;
        // N s_23_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#10037:u8
        let s_24_0: bool = fn_state.gs_10037;
        // D s_24_1: write-var gs#10038 <= s_24_0
        fn_state.gs_10038 = s_24_0;
        // N s_24_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#10037 <= s_25_0
        fn_state.gs_10037 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#10039 <= s_26_0
        fn_state.gs_10039 = s_26_0;
        // N s_26_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#10036 <= s_27_0
        fn_state.gs_10036 = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
