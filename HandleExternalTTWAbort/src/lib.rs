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
use IsExternalAbortTakenSynchronously::*;
use IsExternalSyncAbort__1::*;
use HaveRASExt::*;
use common::*;
pub fn HandleExternalTTWAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memretstatus: ProductTypef8c3639b88223255,
    iswrite: bool,
    memaddrdesc: ProductTypece7c66ccb2cab13e,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
    input_fault: ProductType1d757adad216cdef,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        gs_8607: bool,
        output_fault: ProductType1d757adad216cdef,
        memretstatus: ProductTypef8c3639b88223255,
        iswrite: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        accdesc: ProductType9878976b5bcce9c9,
        size: i128,
        input_fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        memretstatus,
        iswrite,
        memaddrdesc,
        accdesc,
        size,
        input_fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_0_0: read-var input_fault:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.input_fault;
        // D s_0_1: write-var output_fault <= s_0_0
        fn_state.output_fault = s_0_0;
        // D s_0_2: read-var memretstatus.0:struct
        let s_0_2: bool = fn_state.memretstatus._0;
        // D s_0_3: write-var output_fault.5 <= s_0_2
        fn_state.output_fault._5 = s_0_2;
        // D s_0_4: read-var memretstatus.2:struct
        let s_0_4: u32 = fn_state.memretstatus._2;
        // D s_0_5: write-var output_fault.16 <= s_0_4
        fn_state.output_fault._16 = s_0_4;
        // D s_0_6: read-var output_fault:struct
        let s_0_6: ProductType1d757adad216cdef = fn_state.output_fault;
        // D s_0_7: call IsExternalSyncAbort__1(s_0_6)
        let s_0_7: bool = IsExternalSyncAbort__1(state, tracer, s_0_6);
        // N s_0_8: branch s_0_7 b19 b1
        if s_0_7 {
            return block_19(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#8607 <= s_1_0
        fn_state.gs_8607 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var gs#8607:u8
        let s_2_0: bool = fn_state.gs_8607;
        // N s_2_1: branch s_2_0 b16 b3
        if s_2_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_4_0: read-var output_fault:struct
        let s_4_0: ProductType1d757adad216cdef = fn_state.output_fault;
        // D s_4_1: call IsExternalSyncAbort__1(s_4_0)
        let s_4_1: bool = IsExternalSyncAbort__1(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b13 b5
        if s_4_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveRASExt(s_6_0)
        let s_6_1: bool = HaveRASExt(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b12 b7
        if s_6_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_8_0: read-var output_fault:struct
        let s_8_0: ProductType1d757adad216cdef = fn_state.output_fault;
        // D s_8_1: call IsExternalSyncAbort__1(s_8_0)
        let s_8_1: bool = IsExternalSyncAbort__1(state, tracer, s_8_0);
        // D s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // N s_8_3: branch s_8_2 b11 b9
        if s_8_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_10_0: read-var output_fault:struct
        let s_10_0: ProductType1d757adad216cdef = fn_state.output_fault;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_11_0: read-var output_fault:struct
        let s_11_0: ProductType1d757adad216cdef = fn_state.output_fault;
        // D s_11_1: call PendSErrorInterrupt(s_11_0)
        let s_11_1: () = PendSErrorInterrupt(state, tracer, s_11_0);
        // C s_11_2: const #0u : u32
        let s_11_2: u32 = 0;
        // D s_11_3: write-var output_fault.16 <= s_11_2
        fn_state.output_fault._16 = s_11_2;
        // N s_11_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_12_0: read-var memretstatus.1:struct
        let s_12_0: u32 = fn_state.memretstatus._1;
        // D s_12_1: write-var output_fault.10 <= s_12_0
        fn_state.output_fault._10 = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_13_0: read-var output_fault.16:struct
        let s_13_0: u32 = fn_state.output_fault._16;
        // C s_13_1: const #10u : u32
        let s_13_1: u32 = 10;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b15 b14
        if s_13_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_14_0: const #9u : u32
        let s_14_0: u32 = 9;
        // D s_14_1: write-var output_fault.16 <= s_14_0
        fn_state.output_fault._16 = s_14_0;
        // N s_14_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_15_0: const #11u : u32
        let s_15_0: u32 = 11;
        // D s_15_1: write-var output_fault.16 <= s_15_0
        fn_state.output_fault._16 = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_16_0: read-var output_fault.16:struct
        let s_16_0: u32 = fn_state.output_fault._16;
        // C s_16_1: const #10u : u32
        let s_16_1: u32 = 10;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b18 b17
        if s_16_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_17_0: const #15u : u32
        let s_17_0: u32 = 15;
        // D s_17_1: write-var output_fault.16 <= s_17_0
        fn_state.output_fault._16 = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_18_0: const #14u : u32
        let s_18_0: u32 = 14;
        // D s_18_1: write-var output_fault.16 <= s_18_0
        fn_state.output_fault._16 = s_18_0;
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_19_0: read-var memretstatus:struct
        let s_19_0: ProductTypef8c3639b88223255 = fn_state.memretstatus;
        // D s_19_1: read-var iswrite:u8
        let s_19_1: bool = fn_state.iswrite;
        // D s_19_2: read-var memaddrdesc:struct
        let s_19_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_19_3: read-var size:i
        let s_19_3: i128 = fn_state.size;
        // D s_19_4: read-var accdesc:struct
        let s_19_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_19_5: call IsExternalAbortTakenSynchronously(s_19_0, s_19_1, s_19_2, s_19_3, s_19_4)
        let s_19_5: bool = IsExternalAbortTakenSynchronously(
            state,
            tracer,
            s_19_0,
            s_19_1,
            s_19_2,
            s_19_3,
            s_19_4,
        );
        // D s_19_6: not s_19_5
        let s_19_6: bool = !s_19_5;
        // D s_19_7: write-var gs#8607 <= s_19_6
        fn_state.gs_8607 = s_19_6;
        // N s_19_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
