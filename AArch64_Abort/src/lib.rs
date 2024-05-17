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
use IsDebugException::*;
use AArch64_BreakpointException::*;
use AArch64_DataAbort::*;
use TakeGPCException::*;
use UsingAArch32::*;
use AArch64_InstructionAbort::*;
use AArch64_VectorCatchException::*;
use ReportAsGPCException::*;
use AArch64_WatchpointException::*;
use common::*;
pub fn AArch64_Abort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_7289: ProductType396b95aa74979079,
        ga_7283: ProductType9878976b5bcce9c9,
        gs_9991: bool,
        gs_9990: bool,
        ga_7293: ProductType9878976b5bcce9c9,
        vaddress: u64,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        vaddress,
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var fault:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_1: call IsDebugException(s_0_0)
        let s_0_1: bool = IsDebugException(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b9 b1
        if s_0_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var fault.6:struct
        let s_1_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_1_1: write-var ga#7289 <= s_1_0
        fn_state.ga_7289 = s_1_0;
        // D s_1_2: read-var ga#7289.0:struct
        let s_1_2: u32 = fn_state.ga_7289._0;
        // C s_1_3: const #0u : u32
        let s_1_3: u32 = 0;
        // D s_1_4: cmp-eq s_1_2 s_1_3
        let s_1_4: bool = ((s_1_2) == (s_1_3));
        // N s_1_5: branch s_1_4 b8 b2
        if s_1_4 {
            return block_8(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#9990 <= s_2_0
        fn_state.gs_9990 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#9990:u8
        let s_3_0: bool = fn_state.gs_9990;
        // N s_3_1: branch s_3_0 b7 b4
        if s_3_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var fault.0:struct
        let s_4_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_4_1: write-var ga#7293 <= s_4_0
        fn_state.ga_7293 = s_4_0;
        // D s_4_2: read-var ga#7293.1:struct
        let s_4_2: u32 = fn_state.ga_7293._1;
        // C s_4_3: const #0u : u32
        let s_4_3: u32 = 0;
        // D s_4_4: cmp-eq s_4_2 s_4_3
        let s_4_4: bool = ((s_4_2) == (s_4_3));
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
        // D s_5_0: read-var vaddress:u64
        let s_5_0: u64 = fn_state.vaddress;
        // D s_5_1: read-var fault:struct
        let s_5_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_5_2: call AArch64_DataAbort(s_5_0, s_5_1)
        let s_5_2: () = AArch64_DataAbort(state, tracer, s_5_0, s_5_1);
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var vaddress:u64
        let s_6_0: u64 = fn_state.vaddress;
        // D s_6_1: read-var fault:struct
        let s_6_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_6_2: call AArch64_InstructionAbort(s_6_0, s_6_1)
        let s_6_2: () = AArch64_InstructionAbort(state, tracer, s_6_0, s_6_1);
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var vaddress:u64
        let s_7_0: u64 = fn_state.vaddress;
        // D s_7_1: read-var fault:struct
        let s_7_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_7_2: call TakeGPCException(s_7_0, s_7_1)
        let s_7_2: () = TakeGPCException(state, tracer, s_7_0, s_7_1);
        // N s_7_3: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var fault:struct
        let s_8_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_8_1: call ReportAsGPCException(s_8_0)
        let s_8_1: bool = ReportAsGPCException(state, tracer, s_8_0);
        // D s_8_2: write-var gs#9990 <= s_8_1
        fn_state.gs_9990 = s_8_1;
        // N s_8_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var fault.0:struct
        let s_9_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_9_1: write-var ga#7283 <= s_9_0
        fn_state.ga_7283 = s_9_0;
        // D s_9_2: read-var ga#7283.1:struct
        let s_9_2: u32 = fn_state.ga_7283._1;
        // C s_9_3: const #0u : u32
        let s_9_3: u32 = 0;
        // D s_9_4: cmp-eq s_9_2 s_9_3
        let s_9_4: bool = ((s_9_2) == (s_9_3));
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
        // D s_10_0: read-var vaddress:u64
        let s_10_0: u64 = fn_state.vaddress;
        // D s_10_1: read-var fault:struct
        let s_10_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_10_2: call AArch64_WatchpointException(s_10_0, s_10_1)
        let s_10_2: () = AArch64_WatchpointException(state, tracer, s_10_0, s_10_1);
        // N s_10_3: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call UsingAArch32(s_11_0)
        let s_11_1: bool = UsingAArch32(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b16 b12
        if s_11_1 {
            return block_16(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#9991 <= s_12_0
        fn_state.gs_9991 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#9991:u8
        let s_13_0: bool = fn_state.gs_9991;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var fault:struct
        let s_14_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_14_1: call AArch64_BreakpointException(s_14_0)
        let s_14_1: () = AArch64_BreakpointException(state, tracer, s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var fault:struct
        let s_15_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_1: call AArch64_VectorCatchException(s_15_0)
        let s_15_1: () = AArch64_VectorCatchException(state, tracer, s_15_0);
        // N s_15_2: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var fault.2:struct
        let s_16_0: u8 = fn_state.fault._2;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 4u16);
        // C s_16_2: const #1328u : u32
        let s_16_2: u32 = 1328;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 4u16);
        // D s_16_5: cmp-eq s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) == (s_16_4));
        // D s_16_6: write-var gs#9991 <= s_16_5
        fn_state.gs_9991 = s_16_5;
        // N s_16_7: jump b13
        return block_13(state, tracer, fn_state);
    }
}
